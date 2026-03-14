use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ReportingSeverity {
    Ignore,
    Log,
    Warn,
    Throw,
}

/// Plugin/preset config: either a bare name or a [name, options] pair.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PluginConfig {
    Named(String),
    WithOptions(String, serde_json::Value),
}

pub type PresetConfig = PluginConfig;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct I18nConfig {
    pub locale: String,
    pub locales: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locales_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FutureConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental_faster: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HtmlTagObject {
    pub tag_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "innerHTML")]
    pub inner_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub append_to_head: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScriptAttrs {
    pub src: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ScriptEntry {
    Src(String),
    Attrs(ScriptAttrs),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StylesheetAttrs {
    pub href: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum StylesheetEntry {
    Href(String),
    Attrs(StylesheetAttrs),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MarkdownConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mermaid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

/// Full Docusaurus site configuration — mirrors @docusaurus/types `DocusaurusConfig`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DocusaurusConfig {
    pub title: String,
    pub url: String,
    pub base_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_line: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,

    pub no_index: bool,
    pub on_broken_links: ReportingSeverity,
    pub on_broken_anchors: ReportingSeverity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_broken_markdown_links: Option<ReportingSeverity>,
    pub on_duplicate_routes: ReportingSeverity,

    pub base_url_issue_banner: bool,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub plugins: Vec<PluginConfig>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub presets: Vec<PresetConfig>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub themes: Vec<PluginConfig>,

    pub static_directories: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_delimiter: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n: Option<I18nConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future: Option<FutureConfig>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub scripts: Vec<ScriptEntry>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub stylesheets: Vec<StylesheetEntry>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub head_tags: Vec<HtmlTagObject>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub client_modules: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<MarkdownConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
}

impl Default for DocusaurusConfig {
    fn default() -> Self {
        Self {
            title: String::new(),
            url: String::new(),
            base_url: String::new(),
            tag_line: None,
            favicon: None,
            no_index: false,
            on_broken_links: ReportingSeverity::Throw,
            on_broken_anchors: ReportingSeverity::Warn,
            on_broken_markdown_links: Some(ReportingSeverity::Warn),
            on_duplicate_routes: ReportingSeverity::Warn,
            base_url_issue_banner: true,
            plugins: Vec::new(),
            presets: Vec::new(),
            themes: Vec::new(),
            static_directories: vec!["static".into()],
            title_delimiter: Some("|".into()),
            i18n: None,
            future: None,
            scripts: Vec::new(),
            stylesheets: Vec::new(),
            head_tags: Vec::new(),
            client_modules: Vec::new(),
            markdown: None,
            custom_fields: None,
        }
    }
}
