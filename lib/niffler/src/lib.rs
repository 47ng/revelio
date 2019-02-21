#[macro_use]
extern crate serde_derive;

use envy;

#[derive(Debug, Deserialize)]
struct TravisEnv {
  // Detection
  ci: bool,
  travis: bool,
  #[serde(rename = "travis_commit")]
  commit: String,

  #[serde(rename = "travis_repo_slug")]
  repo_slug: String,

  #[serde(rename = "travis_job_web_url")]
  job_web_url: String,
}

#[derive(Debug, Deserialize)]
struct CircleEnv {
  // Detection
  ci: bool,
  circleci: bool,

  #[serde(rename = "circle_sha1")]
  sha1: String,

  #[serde(rename = "circle_build_url")]
  build_url: String,

  #[serde(rename = "circle_repository_url")]
  repository_url: String,
}

// --

#[derive(Debug)]
enum CiEnv {
  Travis(TravisEnv),
  Circle(CircleEnv),
}

// --

#[derive(Debug, Deserialize)]
pub struct BuildInfo {
  /// URL of the build job on the CI platform
  pub build_url: String,

  /// URL of the sources in the public repository
  pub sources_url: String,

  /// SHA-1 hash of the commit at the HEAD of that build
  pub commit: String,
}

impl From<CiEnv> for BuildInfo {
  fn from(env: CiEnv) -> Self {
    match env {
      CiEnv::Travis(travis) => Self {
        build_url: travis.job_web_url,
        sources_url: format!("https://github.com/{}", travis.repo_slug),
        commit: travis.commit,
      },
      CiEnv::Circle(circle) => Self {
        build_url: circle.build_url,
        sources_url: circle.repository_url,
        commit: circle.sha1,
      },
    }
  }
}

/// Sniff the environment and try to generate a BuildInfo from it.
pub fn detect() -> Option<BuildInfo> {
  if let Ok(travis) = envy::from_env::<TravisEnv>() {
    return Some(BuildInfo::from(CiEnv::Travis(travis)));
  }
  if let Ok(circle) = envy::from_env::<CircleEnv>() {
    return Some(BuildInfo::from(CiEnv::Circle(circle)));
  }
  None
}
