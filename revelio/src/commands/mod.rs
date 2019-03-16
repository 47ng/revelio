use std::path::PathBuf;
use structopt::StructOpt;

pub mod generate;
pub mod verify;

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

/// Run the given command
pub fn run(command: Commands) {
  match command {
    Commands::Generate { path, base_url } => generate::run(&path, &base_url),
    Commands::Verify { url } => verify::run(&url),
  }
}
