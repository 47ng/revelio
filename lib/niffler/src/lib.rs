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

  #[serde(rename = "circle_project_reponame")]
  repository_name: String,

  #[serde(rename = "circle_project_username")]
  repository_org: String,

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
    return Some(BuildInfo::from(CiEnv::Travis(travis)));
  }
  if let Ok(circle) = envy::from_env::<CircleEnv>() {
    return Some(BuildInfo::from(CiEnv::Circle(circle)));
  }
  None
}

// -----------------------------------------------------------------------------

#[test]
fn travis() {
  std::env::set_var("CI", "true");
  std::env::set_var("TRAVIS", "true");
  let info = detect();
  assert!(info.is_none()); // Missing required environment variables

  std::env::set_var(
    "TRAVIS_JOB_WEB_URL",
    "https://travis-ci.com/foo/bar/jobs/42",
  );
  std::env::set_var("TRAVIS_COMMIT", "cafebabe");
  std::env::set_var("TRAVIS_REPO_SLUG", "egg/spam");
  std::env::set_var("TRAVIS_COMMIT_RANGE", "aa...ff");

  let info = detect();
  assert!(info.is_some());
  let info = info.unwrap();

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

  // Cleanup
  std::env::remove_var("CI");
  std::env::remove_var("TRAVIS");
  std::env::remove_var("TRAVIS_JOB_WEB_URL");
  std::env::remove_var("TRAVIS_COMMIT");
  std::env::remove_var("TRAVIS_REPO_SLUG");
  std::env::remove_var("TRAVIS_COMMIT_RANGE");
}

// -----------------------------------------------------------------------------

#[test]
fn circle_github() {
  std::env::set_var("CI", "true");
  std::env::set_var("CIRCLECI", "true");
  let info = detect();
  assert!(info.is_none()); // Missing required environment variables

  std::env::set_var("CIRCLE_BUILD_URL", "https://circleci.com/gh/foo/bar/42");
  std::env::set_var("CIRCLE_SHA1", "facade42");
  std::env::set_var("CIRCLE_REPOSITORY_URL", "git@github.com:baz/qux.git");
  std::env::set_var("CIRCLE_PROJECT_USERNAME", "egg");
  std::env::set_var("CIRCLE_PROJECT_REPONAME", "spam");
  std::env::set_var("CIRCLE_COMPARE_URL", "https://example.com/compare/aa...ff");

  let info = detect();
  assert!(info.is_some());
  let info = info.unwrap();
  assert_eq!(info.commit_sha1, "facade42");
  assert_eq!(info.build_url, "https://circleci.com/gh/foo/bar/42");
  assert_eq!(info.sources_url, "https://github.com/egg/spam");
  assert_eq!(info.compare_url, "https://example.com/compare/aa...ff");
  assert_eq!(
    info.commit_url,
    "https://github.com/egg/spam/commit/facade42"
  );

  // Cleanup
  std::env::remove_var("CI");
  std::env::remove_var("CIRCLECI");
  std::env::remove_var("CIRCLE_BUILD_URL");
  std::env::remove_var("CIRCLE_SHA1");
  std::env::remove_var("CIRCLE_REPOSITORY_URL");
  std::env::remove_var("CIRCLE_PROJECT_USERNAME");
  std::env::remove_var("CIRCLE_PROJECT_REPONAME");
  std::env::remove_var("CIRCLE_COMPARE_URL");
}

// -----------------------------------------------------------------------------

#[test]
fn circle_bitbucket() {
  std::env::set_var("CI", "true");
  std::env::set_var("CIRCLECI", "true");
  let info = detect();
  assert!(info.is_none()); // Missing required environment variables

  std::env::set_var("CIRCLE_BUILD_URL", "https://circleci.com/bb/foo/bar/42");
  std::env::set_var("CIRCLE_SHA1", "facade42");
  std::env::set_var("CIRCLE_REPOSITORY_URL", "git@bitbucket.org:baz/qux.git");
  std::env::set_var("CIRCLE_PROJECT_USERNAME", "egg");
  std::env::set_var("CIRCLE_PROJECT_REPONAME", "spam");
  std::env::set_var("CIRCLE_COMPARE_URL", "https://example.com/compare/aa...ff");

  let info = detect();
  assert!(info.is_some());
  let info = info.unwrap();
  assert_eq!(info.commit_sha1, "facade42");
  assert_eq!(info.build_url, "https://circleci.com/bb/foo/bar/42");
  assert_eq!(info.sources_url, "https://bitbucket.org/egg/spam");
  assert_eq!(info.compare_url, "https://example.com/compare/aa...ff");
  assert_eq!(
    info.commit_url,
    "https://bitbucket.org/egg/spam/commits/facade42"
  );

  // Cleanup
  std::env::remove_var("CI");
  std::env::remove_var("CIRCLECI");
  std::env::remove_var("CIRCLE_BUILD_URL");
  std::env::remove_var("CIRCLE_SHA1");
  std::env::remove_var("CIRCLE_REPOSITORY_URL");
  std::env::remove_var("CIRCLE_PROJECT_USERNAME");
  std::env::remove_var("CIRCLE_PROJECT_REPONAME");
  std::env::remove_var("CIRCLE_COMPARE_URL");
}
