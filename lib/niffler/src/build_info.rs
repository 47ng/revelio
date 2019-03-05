#[derive(Debug, Serialize, Deserialize)]
pub struct BuildInfo {
  /// URL of the build job on the CI platform
  pub build_url: String,

  /// URL of the sources in the public repository
  pub sources_url: String,

  /// SHA-1 hash of the commit at the HEAD of that build
  pub commit_sha1: String,

  /// Public URL to the commit at the HEAD of that build
  pub commit_url: String,

  /// Public URL to a list of commits that were included in that build
  pub compare_url: String,
}
