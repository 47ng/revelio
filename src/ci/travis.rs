use crate::Context;
use envy;

#[derive(Debug, Deserialize)]
pub struct Env {
  // Detection
  pub ci: bool,
  pub travis: bool,

  #[serde(rename = "travis_commit")]
  pub commit: String,

  #[serde(rename = "travis_repo_slug")]
  pub repo_slug: String,

  #[serde(rename = "travis_job_web_url")]
  pub job_web_url: String,

  #[serde(rename = "travis_commit_range")]
  pub commit_range: String,
}

// --

impl Env {
  pub fn from_env() -> Option<Self> {
    if let Ok(env) = envy::from_env::<Self>() {
      if !(env.ci && env.travis) {
        return None; // Failed detection
      }
      return Some(env);
    }
    None // Failed detection
  }
}

// --

impl From<Env> for Context {
  fn from(travis: Env) -> Self {
    let repo = format!("https://github.com/{}", travis.repo_slug);
    Self {
      build_url: travis.job_web_url,
      sources_url: repo.clone(),
      commit_sha1: travis.commit.clone(),
      commit_url: format!("{}/commit/{}", repo, travis.commit),
      compare_url: format!("{}/compare/{}", repo, travis.commit_range),
    }
  }
}

// -----------------------------------------------------------------------------

#[test]
fn from_env() {
  let info: Context = (Env {
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
