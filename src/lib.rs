// Forsaken docs justly quibble the vexed programmer's waning zeal
//! Text hyphenation in a variety of languages.
//!
//!
//! ## Usage
//!
//! A typical import comprises the `Hyphenation` trait, the `Standard`
//! hyphenator, and the `Language` enum. This exposes the crate's core
//! functionality, and the set of available languages.
//!
//! ```ignore
//! extern crate hyphenation;
//
//! use hyphenation::{Hyphenation, Standard, Language};
//! ```
//!
//! To begin with, we must initiate the `Corpus` for our working language.
//!
//! ```ignore
//! let english_us = hyphenation::load(Language::English_US).unwrap();
//! ```
//!
//! Our English `Corpus` can now be used by `Hyphenation` methods.
//! Core functionality is provided by `opportunities()`, which returns the
//! byte indices of valid hyphenation points within a word.
//!
//! ```ignore
//! let indices = "hyphenation".opportunities(&english_us);
//! assert_eq!(indices, vec![2, 6]);
//! ```
//!
//! The same `Corpus` may also be used by *hyphenators*: iterators which
//! segment words in accordance with hyphenation practices, as described
//! by the corpus.
//!
//! The simplest (and, presently, only) hyphenator is `Standard`:
//!
//! ```ignore
//! let h: Standard = "hyphenation".hyphenate(&english_us);
//! ```
//!
//! The `Standard` hyphenator does not allocate new strings, returning
//! slices instead.
//!
//! ```ignore
//! let v: Vec<&str> = h.collect();
//! assert_eq!(v, vec!["hy", "phen", "ation"]);
//! ```
//!
//! ## Full-text Hyphenation
//! While hyphenation is always performed on a per-word basis, convenience
//! calls for a subtrait to provide methods to work with full text.
//!
//! ```ignore
//! use hyphenation::{FullTextHyphenation};
//!
//! let h2: Standard = "Word hyphenation by computer.".fulltext_hyphenate(&english_us);
//! let v2: Vec<&str> = h2.collect();
//! assert_eq!(v2, vec!["Word hy", "phen", "ation by com", "puter."]);
//! ```
//!
//! Hyphenators also expose some simple methods to render hyphenated text:
//! `punctuate()` and `punctuate_with(string)`, which mark hyphenation
//! opportunities respectively with soft hyphens (Unicode `U+00AD SOFT HYPHEN`)
//! and any given `string`.
//!
//! ```ignore
//! let h3 = "anfractuous".hyphenate(&english_us);
//! let s3: String = h2.clone().punctuate().collect();
//! assert_eq!(s3, "an\u{ad}frac\u{ad}tu\u{ad}ous".to_owned());
//!
//! let s4: String = h2.punctuate_with("-").collect()
//! assert_eq!(s4, "an-frac-tu-ous".to_owned());
//! ```

extern crate bincode;
extern crate fnv;
extern crate klpattern;
extern crate unicode_segmentation;

mod resources;
mod utilia;
pub mod hyphenator;
pub mod language;
pub mod load;

pub use klpattern::{KLPair, KLPTrie, Exceptions, Patterns};
pub use hyphenator::{Hyphenation, FullTextHyphenation, Standard};
pub use language::{Language, Corpus};
pub use load::{language as load};
