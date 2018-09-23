/*
Hyphenation internals
*/

#[macro_use] extern crate serde;
extern crate atlatl;

mod language;
pub mod dictionary;
pub mod parse;

pub use language::*;
