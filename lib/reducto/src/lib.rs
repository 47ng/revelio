//! List files recursively in a directory along with their hash.
#![deny(missing_docs)]

use base64;
use sha2::digest::FixedOutput;
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use walkdir::WalkDir;

/// Public path to the payload file.
///
/// When appended to the base URL, this gives the public URL to the file.
pub type PayloadPath = PathBuf;

/// Cryptographic hash of the contents of the file
///
/// Format is `{algorithm}:{base64 of hash(contents of the file)}`.
/// Example: `sha256:XfZ1OW/e4KFHuo21GGVX/GhYLREzz4mjBcuYxJI7+WU=`
pub type PayloadHash = String;

/// Map of payload files and their hashes.
pub type PayloadMap = std::collections::HashMap<PayloadPath, PayloadHash>;

/// Generate the hash of a file's content
fn hash_file(path: &PayloadPath) -> std::io::Result<PayloadHash> {
  let contents = std::fs::read(path)?;
  let mut hash = Sha256::default();
  hash.input(contents);
  Ok(format!("sha256:{}", base64::encode(&hash.fixed_result())))
}

/// Recursively walk the given path to build a PayloadMap
///
/// If the given path is a file, it will be the only entry in the map.
/// Otherwise, the directory will be recursively traversed to generate a flat
/// map of path/hash key/value pairs.
pub fn run(root_path: &PathBuf) -> PayloadMap {
  let mut map = PayloadMap::new();
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
