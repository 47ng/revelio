use niffler::detect;

fn main() {
  if let Some(build_info) = detect() {
    println!("Found build info: {:#?}", &build_info);
  } else {
    println!("No luck, could not find any env vars to sniff...");
  }
}
