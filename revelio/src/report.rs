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
  /// Version of the JSON file, to track schema changes
  pub version: u32,

  /// Date & time of generation, ISO-8601 UTC
  pub datetime: String,

  /// Public CI information
  pub context: BuildInfo,

  /// Dictionary of artifacts path: hash
  pub artifacts: reducto::ArtifactUrlMap,
}
