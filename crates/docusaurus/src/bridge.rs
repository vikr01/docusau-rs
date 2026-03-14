use std::path::{Path, PathBuf};

use tempfile::NamedTempFile;

use crate::error::DocusaurusError;

/// Walk up from `site_dir` until a `node_modules/.bin/docusaurus` executable is found.
pub fn find_docusaurus_bin(site_dir: &Path) -> Result<PathBuf, DocusaurusError> {
    let mut dir = site_dir.to_path_buf();
    loop {
        let candidate = dir.join("node_modules").join(".bin").join("docusaurus");
        if candidate.exists() {
            return Ok(candidate);
        }
        match dir.parent() {
            Some(parent) => dir = parent.to_path_buf(),
            None => return Err(DocusaurusError::DocusaurusBinNotFound),
        }
    }
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
