use crate::report::Report;
use chrono::prelude::*;
use mkdirp::mkdirp;
use scourgify::url;
use std::path::PathBuf;

pub fn run(path: &PathBuf, base_url: &str) {
  let context = niffler::detect().expect("Could not detect build environment");
  let artifacts = reducto::run(&path, &url::sanitize(base_url));
  let report = Report {
    version: 1,
    datetime: Utc::now().to_rfc3339(),
    context,
    artifacts,
  };
  // Generate output file
  mkdirp(&path.join(".well-known")).expect("Could not create .well-known directory");
  let json = serde_json::to_string_pretty(&report).unwrap();
  let path = path.join(".well-known/revelio.json");
  std::fs::write(path, json).expect("Could now write to output file");
}
