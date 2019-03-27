//! Command-line tool to generate and verify auditability on the web.
#![deny(missing_docs)]

use structopt::StructOpt;

mod commands;

use commands::{run, Commands};

fn main() {
  run(Commands::from_args());
}
