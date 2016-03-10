//! Data structures and methods for parsing and applying exceptions, which
//! assign predetermined scores to specific words.

use std::collections::hash_map::{HashMap};

use klpair::KLPair;


/// A specialized hash map of pattern-score pairs.
#[derive(Clone, Debug)]
pub struct Exceptions(pub HashMap<String, Vec<u32>>);

impl Exceptions {
    /// Creates an empty `Exceptions` map.
    pub fn empty() -> Exceptions {
        Exceptions(HashMap::new())
    }

    /// Inserts a Knuth-Liang exception pair into the map.
    pub fn insert(&mut self, klpair: KLPair) {
        let (p, tally) = klpair;
        let Exceptions(ref mut m) = *self;

        m.insert(p, tally);
    }

    /// Retrieves the score for each hyphenation point of `word`.
    pub fn score(&self, word: &str) -> Option<&Vec<u32>> {
        let Exceptions(ref m) = *self;
        let w = word.to_lowercase();

        m.get(&w)
    }
}
