/// Contains public information about discoverability of a build and its sources
///
/// All URLs stored in these fields should be publicly accessible on the web
/// (ie: not behind a paywall or requiring subscription to a free service).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildInfo {
  /// URL of the build job on the CI platform
  ///
  /// Example: `https://travis-ci.com/47ng/revelio`
  pub build_url: String,

  /// URL of the sources in the public repository
  ///
  /// Example: `https://github.com/47ng/revelio`
  pub sources_url: String,

  /// SHA-1 hash of the commit at the HEAD of that build
  ///
  /// Example: `3f5dd7c301184862f5da07cde403bfdc7609e61a`
  pub commit_sha1: String,

  /// Public URL to the commit at the HEAD of that build
  ///
  /// Example: `https://github.com/47ng/revelio/commit/3f5dd7c301184862f5da07cde403bfdc7609e61a`
  pub commit_url: String,

  /// Public URL to a list of commits that were included in that build
  ///
  /// Example: `https://github.com/47ng/revelio/compare/c8eee0fa854a...3f5dd7c30118`
  pub compare_url: String,
}
