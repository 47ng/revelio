//! Auditability and integrity checks for open-source web projects
#![deny(missing_docs)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

mod ci;
mod commit;
mod context;
mod hash;
mod manifest;
mod reducto;
pub mod url;

// Aliases & re-exports
pub use self::commit::find_commit_author;
pub use self::context::Context;
pub use self::hash::*;
pub use self::manifest::Manifest;
pub use self::reducto::*;
