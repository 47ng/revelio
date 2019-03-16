/// Make sure a URL is safe to use
///
/// Resulting URL will be HTTPS and end with a trailing slash.
pub fn sanitize(url: &str) -> String {
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
  url
}
