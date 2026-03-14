use std::ffi::CStr;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::config::DocusaurusConfig;
use crate::error::DocusaurusError;

/// Compile `docusaurus.config.rs` in `site_dir` into a cdylib.
///
/// Expects a `Cargo.toml` at `site_dir` whose lib target has `crate-type = ["cdylib"]`
/// and exports the `config` symbol.
pub fn compile_config(site_dir: &Path) -> Result<PathBuf, DocusaurusError> {
    let config_rs = site_dir.join("docusaurus.config.rs");
    if !config_rs.exists() {
        return Err(DocusaurusError::ConfigNotFound(site_dir.to_path_buf()));
    }

    // Cargo requires the lib entry point to be at src/lib.rs. Copy docusaurus.config.rs
    // there before building, then remove the copy after (the original is never touched).
    let src_dir = site_dir.join("src");
    std::fs::create_dir_all(&src_dir)?;
    let lib_rs = src_dir.join("lib.rs");
    let injected = !lib_rs.exists();
    if injected {
        std::fs::copy(&config_rs, &lib_rs)?;
    }

    let manifest = site_dir.join("Cargo.toml");
    let output = Command::new("cargo")
        .args(["build", "--lib", "--manifest-path"])
        .arg(&manifest)
        .output()?;

    if injected {
        let _ = std::fs::remove_file(&lib_rs);
    }

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        return Err(DocusaurusError::CompileFailed(stderr));
    }

    // Locate the produced dylib in target/debug/
    let target_dir = site_dir.join("target").join("debug");
    let dylib = find_dylib(&target_dir)?;
    Ok(dylib)
}

fn find_dylib(target_dir: &Path) -> Result<PathBuf, DocusaurusError> {
    let suffixes: &[&str] = if cfg!(target_os = "macos") {
        &[".dylib"]
    } else if cfg!(target_os = "windows") {
        &[".dll"]
    } else {
        &[".so"]
    };

    let entries = std::fs::read_dir(target_dir)?;
    for entry in entries.flatten() {
        let path = entry.path();
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            let is_lib = name.starts_with("lib") || cfg!(target_os = "windows");
            let has_suffix = suffixes.iter().any(|s| name.ends_with(s));
            if is_lib && has_suffix {
                return Ok(path);
            }
        }
    }

    Err(DocusaurusError::Io(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!("no dylib found in {}", target_dir.display()),
    )))
}

/// Load a compiled cdylib and call its `config()` symbol to obtain a `DocusaurusConfig`.
///
/// The cdylib must export:
/// ```rust,ignore
/// #[no_mangle]
/// pub extern "C" fn config() -> *mut std::os::raw::c_char { /* ... */ }
/// ```
/// The returned pointer must be a valid, nul-terminated, malloc'd C string containing JSON.
///
/// # Safety
/// The dylib's `config` function must uphold the contract above.
pub fn load_config(dylib_path: &Path) -> Result<DocusaurusConfig, DocusaurusError> {
    // SAFETY: we trust the user-provided dylib to follow the documented ABI.
    unsafe {
        let lib = libloading::Library::new(dylib_path)?;
        let sym: libloading::Symbol<unsafe extern "C" fn() -> *mut std::os::raw::c_char> =
            lib.get(b"config\0")?;
        let raw = sym();
        if raw.is_null() {
            return Err(DocusaurusError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "config() returned null pointer",
            )));
        }
        let cstr = CStr::from_ptr(raw);
        let json = cstr.to_string_lossy();
        let cfg: DocusaurusConfig = serde_json::from_str(&json)?;
        // Free the C string — the dylib must have allocated it with the system allocator.
        // We use libc::free if available; otherwise we reconstruct a CString and let it drop.
        // Using CString::from_raw is the idiomatic way when both sides use the same allocator.
        drop(std::ffi::CString::from_raw(raw));
        Ok(cfg)
    }
}
