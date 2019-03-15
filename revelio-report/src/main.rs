#[macro_use]
extern crate serde_derive;

mod report;

fn main() {
  let context = niffler::detect().expect("Could not detect environment");
  let root = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/src"));
  let payloads = reducto::run(&root);
  let report = report::Report {
    version: 1,
    context,
    payloads,
  };
  println!("{}", serde_json::to_string_pretty(&report).unwrap());
}
