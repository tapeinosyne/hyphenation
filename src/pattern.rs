//! Data structures and methods for parsing and applying Knuth-Liang
//! hyphenation patterns.

use std::cmp::{max};
use std::collections::hash_map::{HashMap, Entry};
use std::iter::{once, repeat};

use unicode_normalization::{Recompositions, UnicodeNormalization};


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

    /// Inserts a standard Knuth-Liang hyphenation pattern into the trie.
    pub fn insert(&mut self, pattern: &str) {
        let p_norm = pattern.nfc();
        let tally = points(p_norm.clone());
        let cs = p_norm.filter(|c| !c.is_digit(10));

        let node = cs.fold(self, |t, c| {
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
        let mut points: Vec<u32> = repeat(0).take(length + 1).collect();

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


#[derive(Clone)]
struct Tallying<I> where I: Iterator<Item = char> {
    inner: Recompositions<I>,
    skip_one: bool
}

impl<I> Iterator for Tallying<I> where I: Iterator<Item = char> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        for c in self.inner.by_ref() {
            match (c.to_digit(10), self.skip_one) {
                (n@Some(_), _) => {
                    self.skip_one = true;
                    return n;
                },
                (None, false) => return Some(0),
                (None, true) => self.skip_one = false
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
