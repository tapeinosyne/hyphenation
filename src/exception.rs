//! Data structures and methods for parsing and applying exceptions, which
//! assign predetermined scores to specific words.

use std::collections::hash_map::{HashMap};

use unicode_normalization::{Recompositions, UnicodeNormalization};


/// A specialized hash map of pattern-score pairs.
#[derive(Clone, Debug)]
pub struct Exceptions(pub HashMap<String, Vec<u32>>);

impl Exceptions {
    /// Creates an empty `Exceptions` map.
    pub fn empty() -> Exceptions {
        Exceptions(HashMap::new())
    }

    /// Inserts a standard Knuth-Liang exception into the map.
    pub fn insert(&mut self, pattern: &str) {
        let p_norm = pattern.nfc();
        let tally = points(p_norm.clone());
        let cs: String = p_norm.filter(|&c| c != '-').collect();
        let Exceptions(ref mut m) = *self;

        m.insert(cs, tally);
    }

    /// Retrieves the score for each hyphenation point of `word`.
    pub fn score(&self, word: &str) -> Option<&Vec<u32>> {
        let Exceptions(ref m) = *self;
        let w = word.to_lowercase();

        m.get(&w)
    }
}


#[derive(Clone)]
struct Tallying<I> where I: Iterator<Item = char> {
    inner: Recompositions<I>,
    skip_one: bool
}

impl<I> Iterator for Tallying<I> where I: Iterator<Item = char> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        for c in self.inner.by_ref() {
            match (c == '-', self.skip_one) {
                (true, _) => {
                    self.skip_one = true;
                    return Some(1);
                },
                (false, false) => return Some(0),
                (false, true) => self.skip_one = false
            }
        }

        if !self.skip_one {
            self.skip_one = true;
            Some(0)
        } else {
            None
        }
    }
}

fn points<I: Iterator<Item=char>>(cs: Recompositions<I>) -> Vec<u32> {
    Tallying { inner: cs, skip_one: false }.collect()
}
