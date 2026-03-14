use std::fs;

use tempfile::TempDir;

#[test]
fn find_node_returns_existing_path() {
    let path = docusaurus::bridge::find_node().expect("node must be in PATH for tests");
    assert!(path.exists(), "node path must exist: {}", path.display());
    assert!(
        path.file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.starts_with("node"))
            .unwrap_or(false),
        "binary name must start with 'node'"
    );
}

#[test]
fn write_temp_config_produces_module_exports() {
    let json = r#"{"title":"Hello","url":"https://example.com","baseUrl":"/"}"#;
    let (_handle, path) = docusaurus::bridge::write_temp_config(json)
        .expect("write_temp_config should succeed");

    assert!(path.exists(), "temp file must exist while handle is alive");
    assert_eq!(
        path.extension().and_then(|e| e.to_str()),
        Some("js"),
        "temp file must have .js extension"
    );

    let contents = fs::read_to_string(&path).unwrap();
    assert!(
        contents.starts_with("module.exports = JSON.parse("),
        "must start with module.exports assignment"
    );
    assert!(
        contents.contains("Hello"),
        "must contain config content"
    );
}

#[test]
fn write_temp_config_file_deleted_when_handle_drops() {
    let json = r#"{"title":"T","url":"u","baseUrl":"/"}"#;
    let path = {
        let (handle, p) = docusaurus::bridge::write_temp_config(json).unwrap();
        let copy = p.clone();
        assert!(copy.exists());
        drop(handle);
        copy
    };
    assert!(
        !path.exists(),
        "temp file must be deleted after handle drops"
    );
}

#[test]
fn find_addon_locates_node_in_fake_structure() {
    let tmp = TempDir::new().unwrap();

    // Build the expected addon filename for the current platform
    let addon_dir = tmp.path().join("node_modules").join("docusau-rs");
    fs::create_dir_all(&addon_dir).unwrap();

    // Place all platform variants so the test is portable
    for name in &[
        "docusau_rs.darwin-arm64.node",
        "docusau_rs.darwin-x64.node",
        "docusau_rs.linux-x64-gnu.node",
        "docusau_rs.linux-arm64-gnu.node",
        "docusau_rs.win32-x64-msvc.node",
    ] {
        fs::write(addon_dir.join(name), b"").unwrap();
    }

    // Start search from a subdirectory to exercise the walk-up logic
    let subdir = tmp.path().join("packages").join("site");
    fs::create_dir_all(&subdir).unwrap();

    let addon = docusaurus::bridge::find_addon(&subdir)
        .expect("find_addon should locate the .node file");
    assert!(addon.exists(), "returned addon path must exist");
    assert_eq!(
        addon.extension().and_then(|e| e.to_str()),
        Some("node")
    );
}

#[test]
fn find_addon_returns_err_when_not_found() {
    let tmp = TempDir::new().unwrap();
    let err = docusaurus::bridge::find_addon(tmp.path()).unwrap_err();
    assert!(
        err.to_string().contains("docusau-rs addon not found"),
        "unexpected error: {err}"
    );
}
