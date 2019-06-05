/*!
TODO
*/

#![deny(missing_docs)]
#![allow(unknown_lints)]
#![allow(bare_trait_objects)]
#![allow(warnings)]

#[cfg(test)]
#[macro_use]
extern crate doc_comment;
#[cfg(unix)]
extern crate libc;
extern crate same_file;
#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate winapi_util;

#[cfg(test)]
doctest!("../README.md");

pub use dent::DirEntry;
#[cfg(unix)]
pub use dent::DirEntryExt;
pub use error::{Error, Result};
pub use walk::WalkDir;

mod dent;
mod error;
pub mod os;
#[cfg(test)]
mod tests;
mod util;
mod walk;
