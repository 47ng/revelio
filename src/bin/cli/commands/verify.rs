use rayon::prelude::*;
use reqwest;

use revelio::url;
use revelio::Manifest;

pub fn run(url: &str) {
  let revelio_url = url::sanitize(url) + ".well-known/revelio.json";
  let response = reqwest::get(&revelio_url).unwrap().error_for_status();
  if response.is_err() {
    println!("🔎  Could not read manifest from {}", revelio_url);
    return;
  }
  let manifest: Manifest = response.unwrap().json().unwrap();
  println!("🔎  Found {}", revelio_url);
  println!("🔨  Build context:");
  println!("");
  println!("     Build         {}", manifest.context.build_url);
  println!("     Sources       {}", manifest.context.sources_url);
  println!("     Commit URL    {}", manifest.context.commit_url);
  println!("     Compare URL   {}", manifest.context.compare_url);
  println!("     Commit SHA-1  {}", manifest.context.commit_sha1);
  println!("");
  println!("🔬  Integrity:");
  println!("");
  let verified = manifest
    .artifacts
    .par_iter()
    .map(|(url, declared_hash)| {
      let mut response = reqwest::get(url).unwrap().error_for_status().unwrap();
      let mut buf: Vec<u8> = vec![];
      response.copy_to(&mut buf).unwrap();
      let computed_hash = &revelio::hash(&buf);
      if declared_hash == computed_hash {
        println!("  ✅  {}", url);
        true
      } else {
        println!("  ❌  {}", url);
        false
      }
    })
    .reduce(|| true, |a, b| a & b);
  println!("");
  if verified {
    println!("✅  Verified {}", &url);
  } else {
    println!("❌  Failed verification for {}", &url);
    std::process::exit(1);
  }
}
