use mkdirp::mkdirp;
use revelio::Manifest;
use std::path::PathBuf;

pub fn run(path: &PathBuf, base_url: &str) {
  let manifest = Manifest::from_filesystem(path, base_url);
  // Generate output file
  mkdirp(&path.join(".well-known")).expect("Could not create .well-known directory");
  let json = serde_json::to_string_pretty(&manifest).unwrap();
  let path = path.join(".well-known/revelio.json");
  std::fs::write(path, json).expect("Could now write to output file");
}
