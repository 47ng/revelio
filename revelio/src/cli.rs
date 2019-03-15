use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "revelio",
  about = "Generate and verify auditability on the web"
)]
pub enum Commands {
  #[structopt(name = "generate")]
  Generate {
    #[structopt(short = "p")]
    path: PathBuf,
    #[structopt(short = "u", long = "base-url")]
    base_url: String,
  },
  #[structopt(name = "verify")]
  Verify { url: String },
}
