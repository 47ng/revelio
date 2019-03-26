use chrono::prelude::*;
use mkdirp::mkdirp;
use revelio::url;
use revelio::{Context, Manifest};
use std::path::PathBuf;

pub fn run(path: &PathBuf, base_url: &str) {
  let context = Context::from_env().expect("Could not detect build environment");
  let artifacts = revelio::scan_artifacts(&path, &url::sanitize(base_url));
  let manifest = Manifest {
    version: 1,
    datetime: Utc::now().to_rfc3339(),
    context,
    artifacts,
  };
  // Generate output file
  mkdirp(&path.join(".well-known")).expect("Could not create .well-known directory");
  let json = serde_json::to_string_pretty(&manifest).unwrap();
  let path = path.join(".well-known/revelio.json");
  std::fs::write(path, json).expect("Could now write to output file");
}
