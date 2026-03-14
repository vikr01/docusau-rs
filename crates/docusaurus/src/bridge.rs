use std::path::{Path, PathBuf};

use tempfile::NamedTempFile;

use crate::error::DocusaurusError;

/// Locate the `node` binary via PATH.
pub fn find_node() -> Result<PathBuf, DocusaurusError> {
    Ok(which::which("node")?)
}

/// Walk up from `site_dir` until a `node_modules/docusau-rs` directory is found,
/// then return the path to the `.node` addon inside it.
pub fn find_addon(site_dir: &Path) -> Result<PathBuf, DocusaurusError> {
    let addon_name = addon_filename();
    let mut dir = site_dir.to_path_buf();
    loop {
        let candidate = dir
            .join("node_modules")
            .join("docusau-rs")
            .join(&addon_name);
        if candidate.exists() {
            return Ok(candidate);
        }
        match dir.parent() {
            Some(parent) => dir = parent.to_path_buf(),
            None => return Err(DocusaurusError::AddonNotFound),
        }
    }
}

fn addon_filename() -> String {
    if cfg!(target_os = "windows") {
        "docusau_rs.win32-x64-msvc.node".to_string()
    } else if cfg!(target_os = "macos") {
        if cfg!(target_arch = "aarch64") {
            "docusau_rs.darwin-arm64.node".to_string()
        } else {
            "docusau_rs.darwin-x64.node".to_string()
        }
    } else if cfg!(target_arch = "aarch64") {
        "docusau_rs.linux-arm64-gnu.node".to_string()
    } else {
        "docusau_rs.linux-x64-gnu.node".to_string()
    }
}

/// Write `config_json` into a temporary `.js` file as a `module.exports` assignment.
///
/// The returned `NamedTempFile` **must** be kept alive for as long as the path is in use;
/// the file is deleted when the handle drops.
pub fn write_temp_config(config_json: &str) -> Result<(NamedTempFile, PathBuf), DocusaurusError> {
    use std::io::Write as _;

    // Embed the JSON string in a JS template literal. Three sequences must be escaped,
    // applied in order (E₁ first so later steps cannot re-introduce lone backslashes):
    //   E₁  \   → \\   (prevent unintended escape sequences)
    //   E₂  `   → \`   (prevent template literal from closing)
    //   E₃  ${  → \${  (prevent template interpolation)
    let escaped = config_json
        .replace('\\', r"\\")
        .replace('`', r"\`")
        .replace("${", r"\${");
    let js_content = format!("module.exports = JSON.parse(`{escaped}`);\n");

    let mut file = tempfile::Builder::new().suffix(".js").tempfile()?;
    file.write_all(js_content.as_bytes())?;
    file.flush()?;

    let path = file.path().to_path_buf();
    Ok((file, path))
}
