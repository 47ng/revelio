use niffler::BuildInfo;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deployment {
  /// Base URL where the payloads will be deployed.
  ///
  /// The Revelio manifest will be found at `{base_url}/.well-known/revelio.json`.
  /// All payloads can be found at `{base_url}/{payload key}`.
  pub base_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
  pub version: u32,
  pub context: BuildInfo,
  pub payloads: reducto::PayloadMap,
}
