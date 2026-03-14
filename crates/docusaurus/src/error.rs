use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum DocusaurusError {
    #[error("docusaurus.config.rs not found in {0}")]
    ConfigNotFound(PathBuf),
    #[error("cargo build failed: {0}")]
    CompileFailed(String),
    #[error("failed to load dylib: {0}")]
    DylibLoad(#[from] libloading::Error),
    #[error("config() symbol returned invalid JSON: {0}")]
    ConfigJson(#[from] serde_json::Error),
    #[error("node not found in PATH")]
    NodeNotFound(#[from] which::Error),
    #[error("docusau-rs addon not found in node_modules")]
    AddonNotFound,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("docusaurus command failed with status {0}")]
    CommandFailed(i32),
}
