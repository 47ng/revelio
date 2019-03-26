use crate::Context;
use regex::Regex;
use reqwest;

#[derive(Debug, Serialize, Deserialize)]
pub struct GitCommitAuthor {
  pub name: String,
  pub email: String,
}

// -----------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
struct GitHubCommit {
  author: GitCommitAuthor,
}

#[derive(Debug, Serialize, Deserialize)]
struct BitBucketCommitAuthor {
  raw: String,
}

impl BitBucketCommitAuthor {
  fn try_into(&self) -> Option<GitCommitAuthor> {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"^(.+) <(.+)>").unwrap();
    }
    let captures = RE.captures(&self.raw)?;
    Some(GitCommitAuthor {
      name: String::from(captures.get(1)?.as_str()),
      email: String::from(captures.get(2)?.as_str()),
    })
  }
}

#[derive(Debug, Serialize, Deserialize)]
struct BitBucketCommit {
  author: BitBucketCommitAuthor,
}

// -----------------------------------------------------------------------------

type AuthorResult = Result<GitCommitAuthor, Box<std::error::Error>>;

fn github_extract_committer(info: &Context) -> AuthorResult {
  let slug = info
    .sources_url
    .trim_start_matches("https://github.com/")
    .trim_end_matches("/");
  let url = format!(
    "https://api.github.com/repos/{}/git/commits/{}",
    &slug, &info.commit_sha1
  );
  let data: GitHubCommit = reqwest::get(&url)?.json()?;
  Ok(data.author)
}

fn bitbucket_extract_committer(info: &Context) -> AuthorResult {
  let slug = info
    .sources_url
    .trim_start_matches("https://bitbucket.org/")
    .trim_end_matches("/");
  let url = format!(
    "https://api.bitbucket.org/2.0/repositories/{}/commit/{}",
    &slug, &info.commit_sha1
  );
  let data: BitBucketCommit = reqwest::get(&url)?.json()?;
  data
    .author
    .try_into()
    .ok_or(Box::from("Could not parse committer info"))
}

// -----------------------------------------------------------------------------

/// Attempt to extract the committer email for a given Context
///
/// This may fail for a variety of reasons.
pub fn find_commit_author(info: &Context) -> Option<GitCommitAuthor> {
  match &info.sources_url {
    url if url.starts_with("https://github.com") => github_extract_committer(info).ok(),
    url if url.starts_with("https://bitbucket.org") => bitbucket_extract_committer(info).ok(),
    _ => None,
  }
}

// -----------------------------------------------------------------------------

#[test]
fn find_commit_author_github() {
  let info = Context {
    build_url: String::from("irrelevant"),
    sources_url: String::from("https://github.com/47ng/revelio"),
    commit_sha1: String::from("3f5dd7c301184862f5da07cde403bfdc7609e61a"),
    commit_url: String::from("irrelevant"),
    compare_url: String::from("irrelevant"),
  };
  if let Some(author) = find_commit_author(&info) {
    assert_eq!(author.name, "Francois Best");
    assert_eq!(author.email, "francois@francoisbest.com");
  }
}

#[test]
fn find_commit_author_bitbucket() {
  let info = Context {
    build_url: String::from("irrelevant"),
    sources_url: String::from("https://bitbucket.org/francoisbest/revelio"),
    commit_sha1: String::from("3f5dd7c301184862f5da07cde403bfdc7609e61a"),
    commit_url: String::from("irrelevant"),
    compare_url: String::from("irrelevant"),
  };
  if let Some(author) = find_commit_author(&info) {
    assert_eq!(author.name, "Francois Best");
    assert_eq!(author.email, "francois@francoisbest.com");
  }
}
