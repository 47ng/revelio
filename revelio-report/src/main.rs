#[macro_use]
extern crate serde_derive;

mod report;

fn main() {
  let info = niffler::detect().unwrap();
  let root = std::env::current_dir().unwrap();
  let hash = hashdir::DirNode::from_path(&root, &root).unwrap();
  let report = report::Report {
    info,
    payload: hash,
  };
  println!("{}", serde_json::to_string_pretty(&report).unwrap());
}
