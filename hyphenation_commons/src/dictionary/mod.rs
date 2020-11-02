//! Data structures for the storage of hyphenation patterns and exceptions.

pub mod extended;
mod trie;

use std::hash::Hash;
use std::collections::HashMap;

use language::Language;
use dictionary::trie::PrefixMatches;
use parse::Parse;
pub use dictionary::trie::{Error, Trie};


#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Locus {
    pub index : u8,
    pub value : u8
}

/// A trie mapping hyphenation patterns to their tallies.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Patterns {
    tallies : Vec<Vec<Locus>>,
    automaton : Trie
}

impl Patterns {
    pub fn from_iter<I>(iter: I) -> Result<Self, trie::Error>
    where I : IntoIterator<Item = (String, <Patterns as Parse>::Tally)> {
        let (kvs, tallies) = uniques(iter.into_iter());
        let automaton = Trie::from_iter(kvs.into_iter()) ?;
        Ok(Patterns { tallies, automaton })
    }
}

/// A specialized hashmap associating words to their known hyphenation.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exceptions(pub HashMap<String, Vec<usize>>);

/// A dictionary for standard Knuth–Liang hyphenation.
///
/// It comprises the working language, the pattern and exception sets,
/// and the character boundaries for hyphenation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Standard {
    language : Language,
    patterns : Patterns,
    pub exceptions : Exceptions,
    /// The minimum number of `char`s from the start and end of a word where breaks
    /// may not occur.
    pub minima : (usize, usize)
}


impl Standard {
    /// The language for which this dictionary can provide hyphenation.
    pub fn language(&self) -> Language { self.language }

    /// An iterator over the tallies associated to all prefixes of the query, including
    /// the query itself.
    pub fn prefix_tallies<'f, 'q>(&'f self, query : &'q [u8]) -> PrefixTallies<'f, 'q, Vec<Locus>> {
        PrefixTallies {
            matches : self.patterns.automaton.get_prefixes(query),
            tallies : &self.patterns.tallies
        }
    }
}

pub struct PrefixTallies<'f, 'q, T> {
    tallies : &'f [T],
    matches : PrefixMatches<'f, 'q>
}

impl<'f, 'q, T> Iterator for PrefixTallies<'f, 'q, T> {
    type Item = &'f T;

    fn next(&mut self) -> Option<Self::Item> {
        self.matches.next()
            .and_then(|i| self.tallies.get(i as usize))
    }
}


/// An intermediate dictionary builder, primarily to retain field privacy in the
/// dictionary.
#[derive(Debug)]
pub struct Builder {
    pub language : Language,
    pub patterns : Patterns,
    pub exceptions : Exceptions
}

impl From<Builder> for Standard {
    fn from(b : Builder) -> Standard {
         Standard {
            language : b.language,
            patterns : b.patterns,
            exceptions : b.exceptions,
            minima : b.language.minima()
        }
    }
}


pub fn uniques<I, T>(iter : I) -> (Vec<(String, u64)>, Vec<T>)
where T : Eq + Clone + Hash
    , I : Iterator<Item = (String, T)>
{
    let mut pairs = Vec::new();
    let mut tally_ids = HashMap::new();
    let mut tallies : Vec<T> = Vec::with_capacity(256);
    for (pattern, tally) in iter {
        match tally_ids.get(&tally) {
            Some(&id) => pairs.push((pattern, id)),
            None => {
                let id = tallies.len() as u64;
                tallies.push(tally.clone());
                tally_ids.insert(tally, id);
                pairs.push((pattern, id));
            }
        }
    }
    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    pairs.dedup_by(|a, b| a.0 == b.0);
    (pairs, tallies)
}
