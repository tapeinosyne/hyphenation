//! Data structures and methods for parsing and applying Knuth-Liang
//! hyphenation patterns.

use std::cmp::{max};
use std::collections::hash_map::{HashMap, Entry};
use std::iter::{once};

use unicode_normalization::{UnicodeNormalization};


/// A basic trie, used to associate patterns to their hyphenation scores.
#[derive(Clone, Debug)]
pub struct Patterns {
    pub tally: Option<Vec<u32>>,
    pub descendants: HashMap<char, Patterns>
}

impl Patterns {
    /// Creates an empty `Patterns` trie.
    pub fn empty() -> Patterns {
        Patterns {
            tally: None,
            descendants: HashMap::new()
        }
    }

    /// Inserts a Knuth-Liang hyphenation pair into the trie.
    pub fn insert(&mut self, klpair: (String, Vec<u32>)) {
        let (p, tally) = klpair;
        let p_norm = p.nfc();

        let node = p_norm.fold(self, |t, c| {
            match t.descendants.entry(c) {
                Entry::Vacant(e) => e.insert(Patterns::empty()),
                Entry::Occupied(e) => e.into_mut()
            }
        });

        node.tally = Some(tally);
    }

    /// Assigns a score to each potential hyphenation point.
    ///
    /// All patterns matching a substring of `word` are compounded, and for
    /// each hyphenation point, the highest competing value is selected.
    pub fn score(&self, word: &str) -> Vec<u32> {
        let w = word.to_lowercase();
        let cs = once('.').chain(w.chars()).chain(once('.'));
        let length = cs.clone().count();
        let mut points: Vec<u32> = vec![0; length + 1];

        for i in 0..length {
            let mut m = &self.descendants;
            for c in cs.clone().skip(i) {
                match m.get(&c) {
                    Some(&Patterns { tally: Some(ref t), descendants: ref m1 }) =>
                        for (j, &p) in t.iter().enumerate() {
                            let p1 = points[i + j];
                            m = m1;
                            points[i + j] = max(p, p1)
                    },
                    Some(patterns) => m = &patterns.descendants,
                    _ => break
                }
            }
        }

        points.truncate(length - 1);
        points.remove(0);
        points
    }
}
