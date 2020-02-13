//! Data structures for the storage of hyphenation patterns and exceptions.

pub mod extended;
pub mod trie;

use std::collections::HashMap;

use language::Language;
pub use dictionary::trie::Trie;


#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Locus {
    pub index : u8,
    pub value : u8
}

/// A trie mapping hyphenation patterns to their tallies.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Patterns {
    pub tallies : Vec<Vec<Locus>>,
    pub automaton : Trie
}

/// A specialized hashmap associating words to their known hyphenation.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exceptions(pub HashMap<String, Vec<usize>>);

/// A dictionary for standard Knuth–Liang hyphenation.
///
/// It comprises the working language, the pattern and exception sets,
/// and the character boundaries for hyphenation.
#[derive(Debug, Serialize, Deserialize)]
pub struct Standard {
    pub language : Language,
    pub patterns : Patterns,
    pub exceptions : Exceptions,
    /// The minimum number of `char`s from the start and end of a word where breaks
    /// may not occur.
    pub minima : (usize, usize)
}
