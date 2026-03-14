use std::path::PathBuf;

use tempfile::NamedTempFile;

use crate::error::DocusaurusError;

pub fn find_node() -> Result<PathBuf, DocusaurusError> {
    Ok(which::which("node")?)
}

/// Write `config_json` into a temporary `.json` file.
///
/// The returned `NamedTempFile` **must** be kept alive for as long as the path is in use;
/// the file is deleted when the handle drops.
pub fn write_temp_json(config_json: &str) -> Result<(NamedTempFile, PathBuf), DocusaurusError> {
    use std::io::Write as _;

    let mut file = tempfile::Builder::new().suffix(".json").tempfile()?;
    file.write_all(config_json.as_bytes())?;
    file.flush()?;

    let path = file.path().to_path_buf();
    Ok((file, path))
}

pub fn write_temp_js(js: &str) -> Result<(NamedTempFile, PathBuf), DocusaurusError> {
    use std::io::Write as _;

    let mut file = tempfile::Builder::new().suffix(".js").tempfile()?;
    file.write_all(js.as_bytes())?;
    file.flush()?;

    let path = file.path().to_path_buf();
    Ok((file, path))
}
