#[macro_use]
extern crate serde_derive;

use envy;

#[derive(Debug, Deserialize)]
struct TravisEnv {
  // Detection
  ci: bool,
  travis: bool,

  #[serde(rename = "TRAVIS_COMMIT")]
  commit: String,

  #[serde(rename = "TRAVIS_REPO_SLUG")]
  repo_slug: String,

  #[serde(rename = "TRAVIS_JOB_WEB_URL")]
  job_web_url: String,
}

#[derive(Debug, Deserialize)]
struct CircleEnv {
  // Detection
  ci: bool,
  circleci: bool,

  #[serde(rename = "CIRCLE_SHA1")]
  sha1: String,

  #[serde(rename = "CIRCLE_BUILD_URL")]
  build_url: String,

  #[serde(rename = "CIRCLE_REPOSITORY_URL")]
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
