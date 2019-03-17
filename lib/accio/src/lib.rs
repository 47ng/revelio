//! Retrieve information from the public internet.
#![deny(missing_docs)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

mod commit;

pub use commit::find_commit_author;
