//! Definitions for supported Continuous Integration environments

use crate::Context;

mod circle;
mod travis;

impl Context {
  /// Try to generate a Context by sniffing the environment
  ///
  /// This will call the available CI sniffers in turn.
  pub fn from_env() -> Option<Self> {
    if let Some(travis) = travis::Env::from_env() {
      return Some(travis.into());
    }
    if let Some(circle) = circle::Env::from_env() {
      return Some(circle.into());
    }
    None
  }
}
