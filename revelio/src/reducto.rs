//! List files recursively in a directory along with their hash.
#![deny(missing_docs)]

use crate::hash::hash_file;
use crate::url;

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

/// Recursively walk the given path to build a ArtifactUrlMap
///
/// If the given path is a file, it will be the only entry in the map.
/// Otherwise, the directory will be recursively traversed to generate a flat
/// map of path/hash key/value pairs.
pub fn scan_artifacts(root_path: &PathBuf, base_url: &str) -> ArtifactUrlMap {
  let mut map = ArtifactPathMap::new();
  if root_path.is_file() {
    map.insert(root_path.to_path_buf(), hash_file(&root_path).unwrap());
    return prepend_url(map, base_url);
  }
  assert!(root_path.is_dir());
  let walker = WalkDir::new(root_path).follow_links(true).into_iter();
  for entry in walker.filter_entry(ignore_previous_reports) {
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
  prepend_url(map, base_url)
}

// -----------------------------------------------------------------------------

/// Filter out previous reports from directory walking
///
/// If running multiple times in the same directory, the previously generated
/// `revelio.json` file would show up in the contents of the new one.
/// This filter ensures the file is skipped from analysis if found.
fn ignore_previous_reports(entry: &DirEntry) -> bool {
  entry.file_name().to_str().unwrap_or("") != "revelio.json"
}

/// Prepend a base_url to the artifact paths
///
/// This will generate URLs where the artifacts can be publicly accessed.
fn prepend_url(path_map: ArtifactPathMap, base_url: &str) -> ArtifactUrlMap {
  let base_url = url::sanitize(base_url);
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

// -----------------------------------------------------------------------------

#[test]
fn prepend_domain() {
  let mut path_map = ArtifactPathMap::new();
  path_map.insert(PathBuf::from("foo"), String::from("hash:foo"));
  path_map.insert(PathBuf::from("bar"), String::from("hash:bar"));
  let url_map = prepend_url(path_map, "example.com");
  assert_eq!(url_map.get("https://example.com/foo").unwrap(), "hash:foo");
  assert_eq!(url_map.get("https://example.com/bar").unwrap(), "hash:bar");
}

#[test]
fn prepend_https_url_with_trailing_slash() {
  let mut path_map = ArtifactPathMap::new();
  path_map.insert(PathBuf::from("foo"), String::from("hash:foo"));
  path_map.insert(PathBuf::from("bar"), String::from("hash:bar"));
  let url_map = prepend_url(path_map, "https://example.com/");
  assert_eq!(url_map.get("https://example.com/foo").unwrap(), "hash:foo");
  assert_eq!(url_map.get("https://example.com/bar").unwrap(), "hash:bar");
}
