use std::path::PathBuf;
use std::process::Command;

use crate::bridge::{find_addon, find_node, write_temp_config};
use crate::compile::{compile_config, load_config};
use crate::error::DocusaurusError;

pub struct RunnerOptions {
    pub site_dir: PathBuf,
    pub cli_options: serde_json::Value,
}

/// Compile `docusaurus.config.rs`, serialize the resulting config to JSON, write a
/// temporary JS shim, then invoke the napi-rs addon via `node` as a subprocess.
///
/// `command` must match a named export of the `docusau-rs` addon (e.g. `"build"`, `"start"`).
pub fn run_command(command: &str, opts: RunnerOptions) -> Result<(), DocusaurusError> {
    let dylib = compile_config(&opts.site_dir)?;
    let config = load_config(&dylib)?;
    let config_json = serde_json::to_string(&config)?;

    // Keep `_temp_file` alive until the node process finishes so the temp file exists.
    let (_temp_file, config_path) = write_temp_config(&config_json)?;

    let node = find_node()?;
    let addon = find_addon(&opts.site_dir)?;

    let site_dir_str = opts.site_dir.display().to_string();
    let config_path_str = config_path.display().to_string();
    let addon_path_str = addon.display().to_string();
    let cli_options_json = opts.cli_options.to_string();

    // Inline JS that loads the napi-rs addon and calls the requested command.
    // This is not execSync — it runs inside the same Node.js process via require().
    let script = format!(
        "require('{addon_path_str}').{command}('{site_dir_str}', '{config_path_str}', {cli_options_json});"
    );

    let status = Command::new(node)
        .args(["-e", &script])
        .status()?;

    if !status.success() {
        return Err(DocusaurusError::CommandFailed(
            status.code().unwrap_or(-1),
        ));
    }

    Ok(())
}
