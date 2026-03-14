use std::path::PathBuf;
use std::process::Command;

use crate::bridge::{find_node, write_temp_in, write_temp_json};
use crate::compile::{compile_config, load_config};
use crate::error::DocusaurusError;

const RUNNER_JS: &str = include_str!(concat!(env!("OUT_DIR"), "/runner.js"));

pub struct RunnerOptions {
    pub site_dir: PathBuf,
    pub cli_options: serde_json::Value,
}

/// Compile `docusaurus.config.rs`, serialize the resulting config to JSON, write a
/// temporary JSON config file, then invoke the runner shim via node.
///
/// `command` is a Docusaurus API export name: `"build"`, `"start"`, `"serve"`, etc.
pub fn run_command(command: &str, opts: RunnerOptions) -> Result<(), DocusaurusError> {
    let dylib = compile_config(&opts.site_dir)?;
    let config = load_config(&dylib)?;
    let config_json = serde_json::to_string(&config)?;

    let (_temp_config, config_path) = write_temp_json(&config_json)?;
    // Shim lives inside site_dir so Node's upward module search finds @docusaurus/core.
    let (_temp_shim, shim_path) = write_temp_in(RUNNER_JS, &opts.site_dir)?;

    let node = find_node()?;
    let site_dir_str = opts.site_dir.display().to_string();

    let status = Command::new(node)
        .arg(&shim_path)
        .arg(command)
        .arg(&site_dir_str)
        .arg(&config_path)
        .status()?;

    if !status.success() {
        return Err(DocusaurusError::CommandFailed(status.code().unwrap_or(-1)));
    }

    Ok(())
}
