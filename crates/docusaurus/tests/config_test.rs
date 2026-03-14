use docusaurus::config::{DocusaurusConfig, PluginConfig, ReportingSeverity};

#[test]
fn default_values_match_upstream() {
    let cfg = DocusaurusConfig::default();
    assert!(!cfg.no_index);
    assert_eq!(cfg.on_broken_links, ReportingSeverity::Throw);
    assert_eq!(cfg.on_broken_anchors, ReportingSeverity::Warn);
    assert_eq!(cfg.on_broken_markdown_links, Some(ReportingSeverity::Warn));
    assert_eq!(cfg.on_duplicate_routes, ReportingSeverity::Warn);
    assert!(cfg.base_url_issue_banner);
    assert_eq!(cfg.static_directories, vec!["static"]);
    assert_eq!(cfg.title_delimiter, Some("|".to_string()));
}

#[test]
fn serializes_camel_case() {
    let cfg = DocusaurusConfig {
        title: "My Site".into(),
        url: "https://example.com".into(),
        base_url: "/".into(),
        no_index: true,
        on_broken_links: ReportingSeverity::Warn,
        ..Default::default()
    };
    let json = serde_json::to_value(&cfg).unwrap();
    assert!(json.get("baseUrl").is_some(), "baseUrl must be camelCase");
    assert!(json.get("base_url").is_none(), "snake_case must not appear");
    assert!(json.get("noIndex").is_some());
    assert!(json.get("onBrokenLinks").is_some());
    assert!(json.get("staticDirectories").is_some());
}

#[test]
fn reporting_severity_serializes_lowercase() {
    assert_eq!(
        serde_json::to_string(&ReportingSeverity::Ignore).unwrap(),
        r#""ignore""#
    );
    assert_eq!(
        serde_json::to_string(&ReportingSeverity::Log).unwrap(),
        r#""log""#
    );
    assert_eq!(
        serde_json::to_string(&ReportingSeverity::Warn).unwrap(),
        r#""warn""#
    );
    assert_eq!(
        serde_json::to_string(&ReportingSeverity::Throw).unwrap(),
        r#""throw""#
    );
}

#[test]
fn plugin_config_named_serializes_as_string() {
    let plugin = PluginConfig::Named("@docusaurus/plugin-content-docs".into());
    let json = serde_json::to_string(&plugin).unwrap();
    assert_eq!(json, r#""@docusaurus/plugin-content-docs""#);
}

#[test]
fn plugin_config_with_options_serializes_as_array() {
    let plugin = PluginConfig::WithOptions(
        "@docusaurus/plugin-content-docs".into(),
        serde_json::json!({ "path": "docs" }),
    );
    let json = serde_json::to_value(&plugin).unwrap();
    assert!(json.is_array());
    let arr = json.as_array().unwrap();
    assert_eq!(arr.len(), 2);
    assert_eq!(arr[0].as_str().unwrap(), "@docusaurus/plugin-content-docs");
    assert_eq!(arr[1]["path"].as_str().unwrap(), "docs");
}

#[test]
fn round_trip_serialization() {
    let cfg = DocusaurusConfig {
        title: "Test".into(),
        url: "https://test.com".into(),
        base_url: "/base/".into(),
        plugins: vec![
            PluginConfig::Named("@docusaurus/plugin-sitemap".into()),
            PluginConfig::WithOptions(
                "@docusaurus/plugin-content-docs".into(),
                serde_json::json!({ "path": "docs" }),
            ),
        ],
        ..Default::default()
    };

    let json = serde_json::to_string(&cfg).unwrap();
    let restored: DocusaurusConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(cfg, restored);
}
