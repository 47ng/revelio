use crate::{scan_artifacts, url, ArtifactUrlMap, Context};

use chrono::prelude::*;
use std::path::PathBuf;

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

impl Manifest {
  /// Create a Manifest from the filesystem and environment
  ///
  /// This will sniff out a Context from the environment and
  /// walk the given directory to build the artifact map.
  pub fn from_filesystem(path: &PathBuf, base_url: &str) -> Self {
    let context = Context::from_env().expect("Could not detect build environment");
    let artifacts = scan_artifacts(&path, &url::sanitize(base_url));
    Self {
      version: 1,
      datetime: Utc::now().to_rfc3339(),
      context,
      artifacts,
    }
  }
}
