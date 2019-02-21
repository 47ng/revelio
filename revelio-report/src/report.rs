use niffler::BuildInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
  pub info: BuildInfo,
  pub payload: hashdir::DirNode,
}
