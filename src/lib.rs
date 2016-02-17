// Forsaken docs justly quibble the vexed programmer's waning zeal

extern crate unicode_normalization;

mod utilia;
pub mod exception;
pub mod hyphenator;
pub mod language;
pub mod load;
pub mod pattern;

pub use hyphenator::{Hyphenation, Standard};
pub use language::{Language, Corpus};
pub use load::language as load;
