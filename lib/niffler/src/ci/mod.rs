use crate::BuildInfo;

mod circle;
mod travis;

impl BuildInfo {
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
