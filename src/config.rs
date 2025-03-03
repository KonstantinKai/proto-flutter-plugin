#[derive(Debug, schematic::Schematic, serde::Deserialize, serde::Serialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct FlutterPluginConfig {
    pub base_url: String,
}

impl Default for FlutterPluginConfig {
    fn default() -> Self {
        Self {
            base_url: "https://storage.googleapis.com/flutter_infra_release/releases".into(),
        }
    }
}
