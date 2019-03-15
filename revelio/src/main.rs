//! Command-line tool to generate and verify auditability on the web.
#![deny(missing_docs)]

#[macro_use]
extern crate serde_derive;

use chrono::prelude::*;
use mkdirp::mkdirp;
use std::path::PathBuf;
use structopt::StructOpt;

mod cli;
mod report;

fn generate(path: &PathBuf, base_url: &str) {
  let context = niffler::detect().expect("Could not detect build environment");
  let artifacts = reducto::prepend_url(reducto::run(&path), base_url);
  let report = report::Report {
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

fn verify(url: &str) {
  // Insert trailing / if needed
  let url = match url.ends_with("/") {
    true => String::from(url),
    false => String::from(url) + "/",
  };
  // Insert protocol if needed
  let url = match url {
    _ if url.starts_with("https://") => url,
    _ if url.starts_with("http://") => {
      eprintln!("Warning: HTTPS is required, rewriting URL.");
      url.replace("http://", "https://")
    }
    _ => format!("https://{}", url),
  };
  let revelio_url = format!("{}{}", url, ".well-known/revelio.json");
  println!("🔎  Found {}", revelio_url);
  println!("🔨  Build context:");
  let context = niffler::BuildInfo {
    build_url: String::from("https://travis-ci.com/47ng/revelio"),
    sources_url: String::from("https://github.com/47ng/revelio"),
    commit_sha1: String::from("3f5dd7c301184862f5da07cde403bfdc7609e61a"),
    commit_url: String::from(
      "https://github.com/47ng/revelio/commit/3f5dd7c301184862f5da07cde403bfdc7609e61a",
    ),
    compare_url: String::from(
      "https://github.com/47ng/revelio/compare/c8eee0fa854a...3f5dd7c30118",
    ),
  };
  println!("");
  println!("     Build         {}", context.build_url);
  println!("     Sources       {}", context.sources_url);
  println!("     Commit URL    {}", context.commit_url);
  println!("     Compare URL   {}", context.compare_url);
  println!("     Commit SHA-1  {}", context.commit_sha1);
  println!("");
  println!("🔬  Validation:");
  println!("");
  println!("  ✅  https://example.com/index.html");
  println!("  ✅  https://example.com/app.css");
  println!("  ✅  https://example.com/app.js");
  println!("");
  println!("✅  Validated {}", &url);
}

fn main() {
  let command = cli::Commands::from_args();
  match command {
    cli::Commands::Generate { path, base_url } => generate(&path, &base_url),
    cli::Commands::Verify { url } => verify(&url),
  }
}
