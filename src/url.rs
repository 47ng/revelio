//! Utilities for working with URLs

use log::warn;

/// Make sure a URL is safe to use
///
/// ## Usage
/// ```
/// use revelio::url::sanitize;
///
/// // Insert HTTPS and trailing slash:
/// assert_eq!(sanitize("example.com"), "https://example.com/");
///
/// // Convert HTTP to HTTPS (and insert trailing slash)
/// assert_eq!(sanitize("http://example.com"), "https://example.com/");
///
/// // Convert HTTP to HTTPS
/// assert_eq!(sanitize("http://example.com/"), "https://example.com/");
///
/// // Insert trailing slash:
/// assert_eq!(sanitize("https://example.com"), "https://example.com/");
///
/// // Already sanitized is a no-op:
/// assert_eq!(sanitize("https://example.com/"), "https://example.com/");
/// ```
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
      warn!("HTTPS is required, rewriting URL.");
      url.replace("http://", "https://")
    }
    _ => format!("https://{}", url),
  };
  url
}

#[test]
fn sanitize_domain_name() {
  let received = sanitize("example.com");
  let expected = "https://example.com/";
  assert_eq!(received, expected);
}

#[test]
fn sanitize_trailing_slash() {
  let received = sanitize("https://example.com");
  let expected = "https://example.com/";
  assert_eq!(received, expected);
}

#[test]
fn sanitize_https() {
  let received = sanitize("http://example.com/");
  let expected = "https://example.com/";
  assert_eq!(received, expected);
}

#[test]
fn sanitize_no_op() {
  let received = sanitize("https://example.com/");
  let expected = "https://example.com/";
  assert_eq!(received, expected);
}
