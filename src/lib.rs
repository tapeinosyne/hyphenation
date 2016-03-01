// Forsaken docs justly quibble the vexed programmer's waning zeal

extern crate fnv;
#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate unicode_normalization;
extern crate unicode_segmentation;

mod klpair;
mod utilia;
pub mod exception;
pub mod hyphenator;
pub mod language;
pub mod load;
pub mod pattern;

pub use hyphenator::{Hyphenation, Standard};
pub use language::{Language, Corpus};
pub use load::{set_pattern_folder, language as load};
