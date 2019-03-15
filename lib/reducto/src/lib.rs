//! List files recursively in a directory along with their hash.
#![deny(missing_docs)]

use base64;
use sha2::digest::FixedOutput;
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

/// Filesystem path to the artifact file.
///
/// When appended to the base URL, this gives the public URL to the file.
pub type ArtifactPath = PathBuf;

/// Public URL to access the artifact
pub type ArtifactUrl = String;

/// Cryptographic hash of the contents of the file
///
/// Format is `{algorithm}:{base64 of hash(contents of the file)}`.
/// Example: `sha256:XfZ1OW/e4KFHuo21GGVX/GhYLREzz4mjBcuYxJI7+WU=`
pub type ArtifactHash = String;

/// Map of artifact file paths to their hashes.
pub type ArtifactPathMap = std::collections::HashMap<ArtifactPath, ArtifactHash>;

/// Map of artifact URLs to their hashes
pub type ArtifactUrlMap = std::collections::HashMap<ArtifactUrl, ArtifactHash>;

/// Generate the hash of a file's content
fn hash_file(path: &ArtifactPath) -> std::io::Result<ArtifactHash> {
  let contents = std::fs::read(path)?;
  let mut hash = Sha256::default();
  hash.input(contents);
  Ok(format!("sha256:{}", base64::encode(&hash.fixed_result())))
}

/// Recursively walk the given path to build a ArtifactMap
///
/// If the given path is a file, it will be the only entry in the map.
/// Otherwise, the directory will be recursively traversed to generate a flat
/// map of path/hash key/value pairs.
pub fn run(root_path: &PathBuf) -> ArtifactPathMap {
  let mut map = ArtifactPathMap::new();
  if root_path.is_file() {
    map.insert(root_path.to_path_buf(), hash_file(&root_path).unwrap());
    return map;
  }
  assert!(root_path.is_dir());
  for entry in WalkDir::new(root_path).follow_links(true) {
    if let Ok(entry) = entry {
      let path = entry.path().to_path_buf();
      if path.is_file() {
        map.insert(
          path.strip_prefix(&root_path).unwrap().to_path_buf(),
          hash_file(&path).unwrap(),
        );
      }
    }
  }
  map
}

/// Prepend a base_url to the artifact paths
///
/// This will generate URLs where the artifacts can be publicly accessed.
pub fn prepend_url(path_map: ArtifactPathMap, base_url: &str) -> ArtifactUrlMap {
  // Make sure base_url ends with a trailing slash:
  let base_url = match base_url.ends_with("/") {
    true => String::from(base_url),
    false => String::from(base_url) + "/",
  };
  path_map
    .iter()
    .map(|(path, hash)| {
      (
        format!("{}{}", &base_url, path.to_str().unwrap()),
        hash.to_owned(),
      )
    })
    .collect()
}

#[test]
fn hash_known_file() {
  let path = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/dummy.txt"));
  let hash = hash_file(&path).unwrap();
  assert_eq!(hash, "sha256:pWVAQ+fZlJcMoge/pDJpNPg7r+m8Joqcjw5eG7irQUg=");
}

#[test]
fn hash_missing_file() {
  let path = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/does-not-exist"));
  let hash = hash_file(&path);
  assert!(hash.is_err());
}

#[test]
fn prepend_url_without_trailing_slash() {
  let mut path_map = ArtifactPathMap::new();
  path_map.insert(PathBuf::from("foo"), String::from("hash:foo"));
  path_map.insert(PathBuf::from("bar"), String::from("hash:bar"));
  let url_map = prepend_url(path_map, "https://example.com");
  assert_eq!(url_map.get("https://example.com/foo").unwrap(), "hash:foo");
  assert_eq!(url_map.get("https://example.com/bar").unwrap(), "hash:bar");
}

#[test]
fn prepend_url_with_trailing_slash() {
  let mut path_map = ArtifactPathMap::new();
  path_map.insert(PathBuf::from("foo"), String::from("hash:foo"));
  path_map.insert(PathBuf::from("bar"), String::from("hash:bar"));
  let url_map = prepend_url(path_map, "https://example.com/");
  assert_eq!(url_map.get("https://example.com/foo").unwrap(), "hash:foo");
  assert_eq!(url_map.get("https://example.com/bar").unwrap(), "hash:bar");
}
