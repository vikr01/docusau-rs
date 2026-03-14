use std::path::PathBuf;
use std::process::Command;

use crate::bridge::{find_docusaurus_bin, write_temp_json};
use crate::compile::{compile_config, load_config};
use crate::error::DocusaurusError;

pub struct RunnerOptions {
    pub site_dir: PathBuf,
    pub cli_options: serde_json::Value,
}

/// Compile `docusaurus.config.rs`, serialize the resulting config to JSON, write a
/// temporary JSON config file, then invoke the `docusaurus` CLI from `node_modules/.bin/`.
///
/// `command` is a Docusaurus CLI subcommand: `"build"`, `"start"`, `"serve"`, etc.
pub fn run_command(command: &str, opts: RunnerOptions) -> Result<(), DocusaurusError> {
    let dylib = compile_config(&opts.site_dir)?;
    let config = load_config(&dylib)?;
    let config_json = serde_json::to_string(&config)?;

    // Keep `_temp_file` alive until the subprocess finishes so the file exists.
    let (_temp_file, config_path) = write_temp_json(&config_json)?;

    let bin = find_docusaurus_bin(&opts.site_dir)?;

    let mut cmd = Command::new(bin);
    cmd.current_dir(&opts.site_dir)
        .arg(command)
        .arg("--config")
        .arg(&config_path);

    // Forward non-null CLI options as --key value flags.
    // Object keys are expected in camelCase; convert to kebab-case for the CLI.
    if let serde_json::Value::Object(map) = opts.cli_options {
        for (key, val) in map {
            if val.is_null() {
                continue;
            }
            let flag = format!("--{}", camel_to_kebab(&key));
            if val == serde_json::Value::Bool(true) {
                cmd.arg(flag);
            } else if val != serde_json::Value::Bool(false) {
                cmd.arg(flag).arg(val.to_string().trim_matches('"').to_owned());
            }
        }
    }

    let status = cmd.status()?;

    if !status.success() {
        return Err(DocusaurusError::CommandFailed(status.code().unwrap_or(-1)));
    }

    Ok(())
}

fn camel_to_kebab(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for ch in s.chars() {
        if ch.is_uppercase() {
            out.push('-');
            out.extend(ch.to_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}
