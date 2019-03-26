use crate::Context;
use envy;

#[derive(Debug, Deserialize)]
pub struct Env {
  // Detection
  pub ci: bool,
  pub circleci: bool,

  #[serde(rename = "circle_sha1")]
  pub sha1: String,

  #[serde(rename = "circle_build_url")]
  pub build_url: String,

  #[serde(rename = "circle_repository_url")]
  pub git_url: String,

  #[serde(rename = "circle_project_username")]
  pub repository_org: String,

  #[serde(rename = "circle_project_reponame")]
  pub repository_name: String,

  /// Compare URL could be empty (first deployment or config issues)
  ///
  /// https://discuss.circleci.com/t/does-circleci-2-0-work-with-monorepos/10378/16
  #[serde(rename = "circle_compare_url")]
  pub compare_url: String,
}

// --

impl Env {
  pub fn from_env() -> Option<Self> {
    if let Ok(env) = envy::from_env::<Self>() {
      if !(env.ci && env.circleci) {
        return None; // Failed detection
      }
      return Some(env);
    }
    None // Failed detection
  }
}

// --

impl From<Env> for Context {
  fn from(circle: Env) -> Self {
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

// -----------------------------------------------------------------------------

#[test]
fn from_env_github() {
  let info: Context = (Env {
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
fn from_env_bitbucket() {
  let info: Context = (Env {
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
