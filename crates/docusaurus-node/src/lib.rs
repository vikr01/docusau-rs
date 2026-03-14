use napi::Env;
use napi_derive::napi;

fn escape_js_string(s: &str) -> String {
    s.replace('\\', r"\\")
        .replace('\'', r"\'")
        .replace('\n', r"\n")
        .replace('\r', r"\r")
}

/// Build the JS snippet that calls a named `@docusaurus/core` export and returns its result.
fn docusaurus_script(
    command: &str,
    site_dir: &str,
    config_path: &str,
    cli_options_json: &str,
) -> String {
    let site_dir_esc = escape_js_string(site_dir);
    let config_path_esc = escape_js_string(config_path);
    format!(
        r#"(function() {{
    const core = require('@docusaurus/core/lib/index.js');
    if (typeof core.{command} !== 'function') {{
        throw new Error('@docusaurus/core does not export {command}');
    }}
    return core.{command}('{site_dir_esc}', {{
        config: '{config_path_esc}',
        ...({cli_options_json})
    }});
}})()"#,
        command = command,
        site_dir_esc = site_dir_esc,
        config_path_esc = config_path_esc,
        cli_options_json = cli_options_json,
    )
}

fn validate_json(s: &str, label: &str) -> napi::Result<()> {
    serde_json::from_str::<serde_json::Value>(s).map_err(|e| {
        napi::Error::new(
            napi::Status::InvalidArg,
            format!("{label} is not valid JSON: {e}"),
        )
    })?;
    Ok(())
}

/// Build the Docusaurus site.
///
/// Calls `@docusaurus/core`'s `build` export inside the current Node.js process via
/// `napi::Env::run_script` — no `execSync` involved.
///
/// - `site_dir`: absolute path to the site root
/// - `config_path`: absolute path to the temporary JS config shim
/// - `cli_options_json`: JSON-encoded `BuildCLIOptions` (may be `{}`)
#[napi]
pub fn build(
    env: Env,
    site_dir: String,
    config_path: String,
    cli_options_json: String,
) -> napi::Result<napi::JsObject> {
    validate_json(&cli_options_json, "cli_options_json")?;
    let script = docusaurus_script("build", &site_dir, &config_path, &cli_options_json);
    let result: napi::JsUnknown = env.run_script(script)?;
    result.coerce_to_object()
}

/// Start the Docusaurus dev server.
#[napi]
pub fn start(
    env: Env,
    site_dir: String,
    config_path: String,
    cli_options_json: String,
) -> napi::Result<napi::JsObject> {
    validate_json(&cli_options_json, "cli_options_json")?;
    let script = docusaurus_script("start", &site_dir, &config_path, &cli_options_json);
    let result: napi::JsUnknown = env.run_script(script)?;
    result.coerce_to_object()
}

/// Serve a pre-built Docusaurus site.
#[napi]
pub fn serve(
    env: Env,
    site_dir: String,
    config_path: String,
    cli_options_json: String,
) -> napi::Result<napi::JsObject> {
    validate_json(&cli_options_json, "cli_options_json")?;
    let script = docusaurus_script("serve", &site_dir, &config_path, &cli_options_json);
    let result: napi::JsUnknown = env.run_script(script)?;
    result.coerce_to_object()
}

/// Deploy the Docusaurus site.
#[napi]
pub fn deploy(
    env: Env,
    site_dir: String,
    config_path: String,
    cli_options_json: String,
) -> napi::Result<napi::JsObject> {
    validate_json(&cli_options_json, "cli_options_json")?;
    let script = docusaurus_script("deploy", &site_dir, &config_path, &cli_options_json);
    let result: napi::JsUnknown = env.run_script(script)?;
    result.coerce_to_object()
}

/// Clear the Docusaurus build cache.
#[napi]
pub fn clear(env: Env, site_dir: String) -> napi::Result<napi::JsObject> {
    let site_dir_esc = escape_js_string(&site_dir);
    let script = format!(
        r#"(function() {{
    const core = require('@docusaurus/core/lib/index.js');
    return core.clear('{site_dir_esc}', {{}});
}})()"#,
        site_dir_esc = site_dir_esc,
    );
    let result: napi::JsUnknown = env.run_script(script)?;
    result.coerce_to_object()
}

/// Swizzle a Docusaurus theme component.
#[napi]
pub fn swizzle(
    env: Env,
    site_dir: String,
    config_path: String,
    cli_options_json: String,
) -> napi::Result<napi::JsObject> {
    validate_json(&cli_options_json, "cli_options_json")?;
    let script = docusaurus_script("swizzle", &site_dir, &config_path, &cli_options_json);
    let result: napi::JsUnknown = env.run_script(script)?;
    result.coerce_to_object()
}

/// Write i18n translation files.
#[napi]
pub fn write_translations(
    env: Env,
    site_dir: String,
    config_path: String,
    cli_options_json: String,
) -> napi::Result<napi::JsObject> {
    validate_json(&cli_options_json, "cli_options_json")?;
    let script = docusaurus_script(
        "writeTranslations",
        &site_dir,
        &config_path,
        &cli_options_json,
    );
    let result: napi::JsUnknown = env.run_script(script)?;
    result.coerce_to_object()
}

/// Write heading IDs to MDX/Markdown files.
#[napi]
pub fn write_heading_ids(
    env: Env,
    site_dir: String,
    config_path: String,
    cli_options_json: String,
) -> napi::Result<napi::JsObject> {
    validate_json(&cli_options_json, "cli_options_json")?;
    let script = docusaurus_script(
        "writeHeadingIds",
        &site_dir,
        &config_path,
        &cli_options_json,
    );
    let result: napi::JsUnknown = env.run_script(script)?;
    result.coerce_to_object()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_single_quotes() {
        assert_eq!(escape_js_string("it's"), r"it\'s");
    }

    #[test]
    fn escape_backslash() {
        assert_eq!(escape_js_string(r"C:\path"), r"C:\\path");
    }

    #[test]
    fn escape_newlines() {
        assert_eq!(escape_js_string("line\nbreak"), r"line\nbreak");
    }

    #[test]
    fn script_contains_command_and_paths() {
        let script = docusaurus_script("build", "/my/site", "/tmp/cfg.js", r#"{"dev":true}"#);
        assert!(script.contains("core.build("), "must call build");
        assert!(script.contains("/my/site"), "must embed site_dir");
        assert!(script.contains("/tmp/cfg.js"), "must embed config_path");
        assert!(script.contains(r#"{"dev":true}"#), "must embed cli_options");
    }

    #[test]
    fn script_escapes_paths_with_single_quotes() {
        let script = docusaurus_script("build", "/my's/site", "/tmp/c'fg.js", "{}");
        assert!(
            !script.contains("'/my's/site'"),
            "unescaped quote would break JS string"
        );
        assert!(
            script.contains(r"/my\'s/site"),
            "single quote in path must be escaped"
        );
    }
}
