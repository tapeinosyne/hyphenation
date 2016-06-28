//! Data structures and methods for parsing and applying Knuth-Liang
//! hyphenation patterns.

use std::borrow::Cow;
use std::cmp::{max};
use std::collections::hash_map::{HashMap, Entry};
use std::hash::BuildHasherDefault;
use std::iter::{once};
use std::mem;

use fnv::FnvHasher;


/// A pair representing a Knuth-Liang hyphenation pattern. It comprises
/// alphabetical characters for subword matching and the score of each
/// hyphenation point.
pub type KLPair = (String, Vec<u8>);


/// A basic trie, used to associate patterns to their hyphenation scores.
#[derive(Clone, Debug)]
pub struct Patterns {
    pub tally: Option<Vec<u8>>,
    pub descendants: HashMap<char, Patterns, BuildHasherDefault<FnvHasher>>
}

impl Patterns {
    /// Creates an empty `Patterns` trie.
    pub fn empty() -> Patterns {
        let fnv = BuildHasherDefault::<FnvHasher>::default();

        Patterns {
            tally: None,
            descendants: HashMap::with_hasher(fnv)
        }
    }

    /// Inserts a Knuth-Liang hyphenation pair into the trie.
    ///
    /// If the pattern already existed, the old tally is returned; if not, `None` is.
    pub fn insert(&mut self, klpair: KLPair) -> Option<Vec<u8>>{
        let (p, tally) = klpair;

        let node = p.chars().fold(self, |t, c| {
            match t.descendants.entry(c) {
                Entry::Vacant(e) => e.insert(Patterns::empty()),
                Entry::Occupied(e) => e.into_mut()
            }
        });

        let mut retv = None;
        match node.tally {
            Some(ref mut old) => retv = Some(mem::replace(old, tally)),
            None => node.tally = Some(tally)
        }

        retv
    }

    /// Assigns a score to each potential hyphenation point.
    ///
    /// All patterns matching a substring of `word` are compounded, and for
    /// each hyphenation point, the highest competing value is selected.
    pub fn score(&self, word: &str) -> Vec<u8> {
        let w = match word.chars().any(|c| c.is_uppercase()) {
            true => Cow::Owned(word.to_lowercase()),
            false => Cow::Borrowed(word)
        };
        let cs = once('.').chain(w.chars()).chain(once('.'));
        let match_length = cs.clone().count();

        if match_length <= 3 {
            return vec![];
        }

        let hyphenable_length = match_length - 2;
        let mut points: Vec<u8> = vec![0; hyphenable_length - 1];

        for i in 0..match_length {
            let mut m = &self.descendants;
            for c in cs.clone().skip(i) {
                match m.get(&c) {
                    Some(&Patterns { tally: Some(ref t), descendants: ref m1 }) => {
                        m = m1;
                        for (j, &p) in t.iter().enumerate() {
                            let k = i + j;
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
}
