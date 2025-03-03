use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FlutterDistVersion {
    pub archive: String,
    pub hash: String,
    pub channel: String,
    pub version: String,
    pub sha256: String,
    #[serde(rename(deserialize = "dart_sdk_arch"))]
    pub arch: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct FlutterDistLatest {
    pub stable: String,
    pub beta: String,
}

#[derive(Deserialize, Debug)]
pub struct FlutterDist {
    #[serde(rename(deserialize = "current_release"))]
    pub latest: FlutterDistLatest,
    pub releases: Vec<FlutterDistVersion>,
}

#[derive(Deserialize, Debug)]
pub struct PubspecYamlEnvField {
    pub flutter: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PubspecYaml {
    pub name: String,
    pub environment: Option<PubspecYamlEnvField>,
}
