//! Data structures and methods for parsing and applying Knuth-Liang
//! hyphenation patterns and exceptions.

#![cfg_attr(feature = "serde_derive", feature(proc_macro))]

#[cfg(feature = "serde_derive")]
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate fnv;

use std::borrow::Cow;
use std::cmp::{max};
use std::collections::hash_map::{HashMap, Entry};
use std::hash::BuildHasherDefault;
use std::mem;

use fnv::FnvHasher;

/// A pair representing a Knuth-Liang hyphenation pattern. It comprises
/// alphabetical characters for subword matching and the score of each
/// hyphenation point.
pub type KLPair = (String, Vec<u8>);


#[cfg(feature = "serde_derive")]
include!("serde_types.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));


pub trait KLPTrie<'a> {
    type Score;
    type Tally : Eq;

    fn new() -> Self;

    fn insert(&mut self, (String, Self::Tally)) -> Option<Self::Tally>;

    fn score(&'a self, &str) -> Self::Score;

    fn is_empty(&self) -> bool;
}


impl<'a> KLPTrie<'a> for Patterns {
    type Score = Vec<u8>;
    type Tally = Vec<(u8, u8)>;

    /// Creates an empty `Patterns` trie.
    fn new() -> Patterns {
        let fnv = BuildHasherDefault::<FnvHasher>::default();

        Patterns {
            tally: None,
            descendants: HashMap::with_hasher(fnv)
        }
    }

    /// Inserts a Knuth-Liang hyphenation pair into the trie.
    ///
    /// If the pattern already exists, the old tally is returned; if not, `None` is.
    fn insert(&mut self, (pattern, tally): (String, Self::Tally)) -> Option<Self::Tally> {
        let node = pattern.bytes().fold(self, |t, b| {
            match t.descendants.entry(b) {
                Entry::Vacant(e) => e.insert(Patterns::new()),
                Entry::Occupied(e) => e.into_mut()
            }
        });

        match node.tally {
            Some(ref mut old) => Some(mem::replace(old, tally)),
            None => {
                node.tally = Some(tally);
                None
            }
        }
    }

    /// Assigns a score to each potential hyphenation point.
    ///
    /// All patterns matching a substring of `word` are compounded, and for
    /// each hyphenation point, the highest competing value is selected.
    fn score(&self, word: &str) -> Self::Score {
        let w = match word.chars().any(|c| c.is_uppercase()) {
            true => Cow::Owned(word.to_lowercase()),
            false => Cow::Borrowed(word)
        };
        let match_str = [".", w.as_ref(), "."].concat();
        let match_length = match_str.len();

        if match_length <= 3 {
            return vec![];
        }

        let hyphenable_length = match_length - 2;
        let mut points: Vec<u8> = vec![0; hyphenable_length - 1];

        for i in 0..match_length {
            let mut m = &self.descendants;
            for b in match_str.bytes().skip(i) {
                match m.get(&b) {
                    Some(&Patterns { tally: Some(ref t), descendants: ref m1 }) => {
                        m = m1;
                        for &(j, p) in t.iter() {
                            let k = i + j as usize;
                            if k > 1 && k <= hyphenable_length {
                                let p1 = points[k - 2];
                                points[k - 2] = max(p, p1)
                            }
                        }
                    },
                    Some(patterns) => m = &patterns.descendants,
                    _ => break
                }
            }
        }

        points
    }

    fn is_empty(&self) -> bool {
        self.descendants.is_empty()
    }
}


impl<'a> KLPTrie<'a> for Exceptions {
    type Score = Option<&'a Vec<usize>>;
    type Tally = Vec<usize>;

    /// Creates an empty `Exceptions` map.
    fn new() -> Exceptions {
        Exceptions(HashMap::new())
    }

    /// Inserts a Knuth-Liang exception pair into the map.
    ///
    /// If the pattern already exists, the old score is returned; if not, `None` is.
    fn insert(&mut self, (pattern, score): (String, Self::Tally)) -> Option<Self::Tally> {
        let Exceptions(ref mut m) = *self;

        m.insert(pattern, score)
    }

    /// Retrieves the score for each hyphenation point of `word`.
    fn score(&'a self, word: &str) -> Self::Score {
        let Exceptions(ref m) = *self;
        let w = match word.chars().any(|c| c.is_uppercase()) {
            true => Cow::Owned(word.to_lowercase()),
            false => Cow::Borrowed(word)
        };

        m.get(w.as_ref())
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
