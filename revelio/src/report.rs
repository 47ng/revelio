#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
  /// Version of the JSON file, to track schema changes
  pub version: u32,

  /// Date & time of generation, ISO-8601 UTC
  pub datetime: String,

  /// Public CI information
  pub context: niffler::BuildInfo,

  /// Dictionary of artifacts path: hash
  pub artifacts: reducto::ArtifactUrlMap,
}
