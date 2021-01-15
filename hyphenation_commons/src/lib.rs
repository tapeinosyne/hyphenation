/*
Hyphenation internals
*/

#[macro_use] extern crate serde;
extern crate fst;

pub mod dictionary;
mod language;
pub mod parse;

pub use language::*;
