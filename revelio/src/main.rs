//! Command-line tool to generate and verify auditability on the web.
#![deny(missing_docs)]

#[macro_use]
extern crate serde_derive;

use structopt::StructOpt;

mod commands;
mod report;
mod url;

use commands::{run, Commands};

fn main() {
  run(Commands::from_args());
}
