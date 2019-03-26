use crate::{ArtifactUrlMap, Context};

/// Structure of the `revelio.json` manifest file
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
  /// Version of the JSON file, to track schema changes
  pub version: u32,

  /// Date & time of generation, ISO-8601 UTC
  pub datetime: String,

  /// Public CI information
  pub context: Context,

  /// Dictionary of artifacts `url: hash`
  pub artifacts: ArtifactUrlMap,
}
