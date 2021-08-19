/*
Hyphenation internals
*/

#[macro_use] extern crate serde;

pub mod dictionary;
mod language;
pub mod parse;

pub use language::*;
