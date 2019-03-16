use crate::report::Report;
use rayon::prelude::*;
use reducto::hash::hash;
use reqwest;
use scourgify::url;

pub fn run(url: &str) {
  let revelio_url = url::sanitize(url) + ".well-known/revelio.json";
  let response = reqwest::get(&revelio_url).unwrap().error_for_status();
  if response.is_err() {
    println!("🔎  Could not read manifest from {}", revelio_url);
    return;
  }
  let report: Report = response.unwrap().json().unwrap();
  println!("🔎  Found {}", revelio_url);
  println!("🔨  Build context:");
  println!("");
  println!("     Build         {}", report.context.build_url);
  println!("     Sources       {}", report.context.sources_url);
  println!("     Commit URL    {}", report.context.commit_url);
  println!("     Compare URL   {}", report.context.compare_url);
  println!("     Commit SHA-1  {}", report.context.commit_sha1);
  println!("");
  println!("🔬  Validation:");
  println!("");
  let valid = report
    .artifacts
    .par_iter()
    .map(|(url, declared_hash)| {
      let mut response = reqwest::get(url).unwrap().error_for_status().unwrap();
      let mut buf: Vec<u8> = vec![];
      response.copy_to(&mut buf).unwrap();
      let computed_hash = &hash(&buf);
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
  if valid {
    println!("✅  Validated {}", &url);
  } else {
    println!("❌  Failed validation for {}", &url);
    std::process::exit(1);
  }
}
