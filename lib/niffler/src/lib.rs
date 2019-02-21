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

  #[serde(rename = "travis_commit_range")]
  commit_range: String,
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
  git_url: String,

  #[serde(rename = "circle_project_username")]
  repository_org: String,

  #[serde(rename = "circle_project_reponame")]
  repository_name: String,

  /// Compare URL could be empty (first deployment or config issues)
  ///
  /// https://discuss.circleci.com/t/does-circleci-2-0-work-with-monorepos/10378/16
  #[serde(rename = "circle_compare_url")]
  compare_url: String,
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
  pub commit_sha1: String,

  /// Public URL to the commit at the HEAD of that build
  pub commit_url: String,

  /// Public URL to a list of commits that were included in that build
  pub compare_url: String,
}

impl From<CiEnv> for BuildInfo {
  fn from(env: CiEnv) -> Self {
    match env {
      CiEnv::Travis(travis) => {
        let repo = format!("https://github.com/{}", travis.repo_slug);
        Self {
          build_url: travis.job_web_url,
          sources_url: repo.clone(),
          commit_sha1: travis.commit.clone(),
          commit_url: format!("{}/commit/{}", repo, travis.commit),
          compare_url: format!("{}/compare/{}", repo, travis.commit_range),
        }
      }
      CiEnv::Circle(circle) => {
        let sources_url = match circle.git_url {
          ref url if url.contains("github") => format!(
            "https://github.com/{}/{}",
            circle.repository_org, circle.repository_name
          ),
          ref url if url.contains("bitbucket") => format!(
            "https://bitbucket.org/{}/{}",
            circle.repository_org, circle.repository_name
          ),
          _ => circle.git_url.clone(), // Unknown CI provider, fallback to Git URL
        };
        let commit_url = match sources_url {
          ref url if url.contains("github") => format!("{}/commit/{}", url, circle.sha1),
          ref url if url.contains("bitbucket") => format!("{}/commits/{}", url, circle.sha1),
          _ => circle.git_url,
        };
        Self {
          build_url: circle.build_url,
          sources_url,
          commit_sha1: circle.sha1,
          commit_url,
          compare_url: circle.compare_url, // /!\ could be empty
        }
      }
    }
  }
}

/// Sniff the environment and try to generate a BuildInfo from it.
pub fn detect() -> Option<BuildInfo> {
  if let Ok(travis) = envy::from_env::<TravisEnv>() {
    if !(travis.ci && travis.travis) {
      return None; // Failed detection
    }
    return Some(BuildInfo::from(CiEnv::Travis(travis)));
  }
  if let Ok(circle) = envy::from_env::<CircleEnv>() {
    if !(circle.ci && circle.circleci) {
      return None; // Failed detection
    }
    return Some(BuildInfo::from(CiEnv::Circle(circle)));
  }
  None
}

// -----------------------------------------------------------------------------

#[test]
fn travis() {
  let info: BuildInfo = CiEnv::Travis(TravisEnv {
    ci: true,
    travis: true,
    commit: String::from("cafebabe"),
    repo_slug: String::from("egg/spam"),
    job_web_url: String::from("https://travis-ci.com/foo/bar/jobs/42"),
    commit_range: String::from("aa...ff"),
  })
  .into();

  assert_eq!(info.commit_sha1, "cafebabe");
  assert_eq!(info.build_url, "https://travis-ci.com/foo/bar/jobs/42");
  assert_eq!(info.sources_url, "https://github.com/egg/spam");
  assert_eq!(
    info.compare_url,
    "https://github.com/egg/spam/compare/aa...ff"
  );
  assert_eq!(
    info.commit_url,
    "https://github.com/egg/spam/commit/cafebabe"
  );
}

// -----------------------------------------------------------------------------

#[test]
fn circle_github() {
  let info: BuildInfo = CiEnv::Circle(CircleEnv {
    ci: true,
    circleci: true,
    sha1: String::from("facade42"),
    build_url: String::from("https://circleci.com/gh/foo/bar/42"),
    git_url: String::from("git@github.com:baz/qux.git"),
    repository_org: String::from("egg"),
    repository_name: String::from("spam"),
    compare_url: String::from("https://example.com/compare/aa...ff"),
  })
  .into();

  assert_eq!(info.commit_sha1, "facade42");
  assert_eq!(info.build_url, "https://circleci.com/gh/foo/bar/42");
  assert_eq!(info.sources_url, "https://github.com/egg/spam");
  assert_eq!(info.compare_url, "https://example.com/compare/aa...ff");
  assert_eq!(
    info.commit_url,
    "https://github.com/egg/spam/commit/facade42"
  );
}

// -----------------------------------------------------------------------------

#[test]
fn circle_bitbucket() {
  let info: BuildInfo = CiEnv::Circle(CircleEnv {
    ci: true,
    circleci: true,
    sha1: String::from("facade42"),
    build_url: String::from("https://circleci.com/bb/foo/bar/42"),
    git_url: String::from("git@bitbucket.org:baz/qux.git"),
    repository_org: String::from("egg"),
    repository_name: String::from("spam"),
    compare_url: String::from("https://example.com/compare/aa...ff"),
  })
  .into();

  assert_eq!(info.commit_sha1, "facade42");
  assert_eq!(info.build_url, "https://circleci.com/bb/foo/bar/42");
  assert_eq!(info.sources_url, "https://bitbucket.org/egg/spam");
  assert_eq!(info.compare_url, "https://example.com/compare/aa...ff");
  assert_eq!(
    info.commit_url,
    "https://bitbucket.org/egg/spam/commits/facade42"
  );
}
