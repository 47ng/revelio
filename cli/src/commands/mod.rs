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
  /// Generate a revelio.json manifest file to be deployed with your artifacts
  #[structopt(name = "generate")]
  Generate {
    #[structopt(short = "p")]
    path: PathBuf,
    #[structopt(short = "u", long = "base-url")]
    base_url: String,
  },

  /// Verify the integrity of artifacts on the given URL and print build context
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
