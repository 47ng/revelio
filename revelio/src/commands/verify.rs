use scourgify::url;

pub fn run(url: &str) {
  let revelio_url = url::sanitize(url) + ".well-known/revelio.json";
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
