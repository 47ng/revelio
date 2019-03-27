//! Hashing utilities

use crate::reducto::{ArtifactHash, ArtifactPath};
use base64;
use sha2::digest::FixedOutput;
use sha2::{Digest, Sha256};

/// Generate the hash of a byte array
pub fn hash(data: &Vec<u8>) -> ArtifactHash {
  let mut hash = Sha256::default();
  hash.input(data);
  format!("sha256:{}", base64::encode(&hash.fixed_result()))
}

/// Generate the hash of a file's content
pub fn hash_file(path: &ArtifactPath) -> std::io::Result<ArtifactHash> {
  let contents = std::fs::read(path)?;
  Ok(hash(&contents))
}

// -----------------------------------------------------------------------------

#[test]
fn hash_empty_byte_array() {
  let hash = hash(&vec![]);
  assert_eq!(hash, "sha256:47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFU=");
}

#[test]
fn hash_byte_array() {
  let hash = hash(&vec![
    // Fibonnaci that fit in u8
    0u8, 1u8, 1u8, 2u8, 3u8, 5u8, 8u8, 13u8, 21u8, 34u8, 55u8, 89u8, 144u8, 233u8,
  ]);
  assert_eq!(hash, "sha256:AB1aP/7KLL6jJjAD5Ko71/eP2zigi9UQlCk8bgVUzr8=");
}

#[test]
fn hash_known_file() {
  let path = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/dummy.txt"));
  let hash = hash_file(&path).unwrap();
  assert_eq!(hash, "sha256:pWVAQ+fZlJcMoge/pDJpNPg7r+m8Joqcjw5eG7irQUg=");
}

#[test]
fn hash_missing_file() {
  let path = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/does-not-exist"));
  let hash = hash_file(&path);
  assert!(hash.is_err());
}
