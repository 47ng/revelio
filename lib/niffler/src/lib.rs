#[macro_use]
extern crate serde_derive;

mod build_info;
mod ci;

pub use build_info::BuildInfo;

// --

/// Sniff the environment and try to generate a BuildInfo from it.
pub fn detect() -> Option<BuildInfo> {
  BuildInfo::from_env()
}
