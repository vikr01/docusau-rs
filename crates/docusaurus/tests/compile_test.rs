/// Integration tests for compile_config + load_config.
///
/// These tests require `cargo` in PATH and will compile a real cdylib.
/// They are gated behind the `integration` feature to avoid slowing down
/// `cargo test` by default. Run with:
///   cargo test --test compile_test -- --include-ignored
use std::fs;

use tempfile::TempDir;

fn make_config_crate(dir: &std::path::Path, title: &str) {
    // Cargo.toml for a cdylib
    fs::write(
        dir.join("Cargo.toml"),
        format!(
            r#"[package]
name = "test-docusaurus-config"
version = "0.0.1"
edition = "2021"

[lib]
name = "test_docusaurus_config"
crate-type = ["cdylib"]
path = "docusaurus.config.rs"

[dependencies]
serde = {{ version = "1", features = ["derive"] }}
serde_json = "1"
"#
        ),
    )
    .unwrap();

    // docusaurus.config.rs
    fs::write(
        dir.join("docusaurus.config.rs"),
        format!(
            r#"
use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn config() -> *mut c_char {{
    let json = serde_json::json!({{
        "title": "{title}",
        "url": "https://example.com",
        "baseUrl": "/",
        "noIndex": false,
        "onBrokenLinks": "throw",
        "onBrokenAnchors": "warn",
        "onBrokenMarkdownLinks": "warn",
        "onDuplicateRoutes": "warn",
        "baseUrlIssueBanner": true,
        "staticDirectories": ["static"],
        "titleDelimiter": "|"
    }});
    CString::new(json.to_string()).unwrap().into_raw()
}}
"#,
            title = title,
        ),
    )
    .unwrap();
}

fn cargo_in_path() -> bool {
    which::which("cargo").is_ok()
}

#[test]
#[ignore = "integration: requires cargo in PATH and compiles a real cdylib (~20s)"]
fn compile_and_load_config() {
    if !cargo_in_path() {
        eprintln!("skipping: cargo not found");
        return;
    }

    let tmp = TempDir::new().unwrap();
    make_config_crate(tmp.path(), "Integration Test Site");

    let dylib = docusaurus::compile_config(tmp.path()).expect("compile_config should succeed");
    assert!(dylib.exists(), "dylib must exist at {}", dylib.display());

    let cfg = docusaurus::load_config(&dylib).expect("load_config should succeed");
    assert_eq!(cfg.title, "Integration Test Site");
    assert_eq!(cfg.url, "https://example.com");
    assert_eq!(cfg.base_url, "/");
}

#[test]
fn compile_config_returns_err_when_config_rs_missing() {
    let tmp = TempDir::new().unwrap();
    // No docusaurus.config.rs — expect ConfigNotFound
    let err = docusaurus::compile_config(tmp.path()).unwrap_err();
    assert!(
        err.to_string().contains("docusaurus.config.rs not found"),
        "unexpected error: {err}"
    );
}
