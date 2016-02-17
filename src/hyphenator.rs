//! Hyphenating iterators.

use std::borrow::Cow;

use language::{Corpus};
use utilia::{Interspersable, Intersperse};

pub trait Hyphenation<Hyphenator> where Hyphenator : Iterator {
    /// Returns the byte indices of valid hyphenation points within a word.
    fn opportunities(self, corp: &Corpus) -> Vec<usize>;

    /// Returns an iterator over orthographic syllables separated by valid
    /// hyphenation points.
    ///
    /// Note that, in some orthographies, the syllables of a hyphenated word
    /// are not necessarily substrings of the original word.
    fn hyphenate(self, corp: &Corpus) -> Hyphenator;
}


/// The `Standard` hyphenator iterates over a word, returning slices
/// delimited by word boundaries and valid hyphenation points.
#[derive(Clone)]
pub struct Standard<'a> {
    word: &'a str,
    opportunities: Vec<usize>,
    prior: usize,
    i: usize
}

impl<'a> Standard<'a> {
    // Inserts a soft hyphen at hyphenation points.
    pub fn punctuate(self) -> Intersperse<Self> {
        self.intersperse("\u{ad}")
    }
}


impl<'a> Iterator for Standard<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let start = self.prior;
        let i = self.i;

        match self.opportunities.get(i) {
            Some(&end) => {
                self.prior = end;
                self.i = i + 1;
                Some(&self.word[start .. end])
            },
            None => {
                if i <= self.opportunities.len() {
                    self.i = i + 1;
                    Some(&self.word[start ..])
                } else {
                    None
                }
            }
        }
    }
}


impl<'a> Hyphenation<Standard<'a>> for &'a str {
    fn opportunities(self, corp: &Corpus) -> Vec<usize> {
        if self.chars().count() < corp.left_min + corp.right_min {
            return vec![];
        }

        let pts = match corp.exceptions.iter()
                            .filter_map(|exs| exs.score(self))
                            .next() {
                Some(vec) => Cow::Borrowed(vec),
                None => Cow::Owned(corp.patterns.score(self))
        };

        let (l, r) = (corp.left_min, pts.len() - corp.right_min + 1);

        self.char_indices().skip(l)
            .zip(&pts[l .. r])
            .filter(|&(_, p)| p % 2 != 0)
            .map(|((i, _), _)| i)
            .collect()
    }


    fn hyphenate(self, corp: &Corpus) -> Standard<'a> {
        let os = self.opportunities(corp);

        Standard {
            word: self,
            opportunities: os,
            prior: 0,
            i: 0
        }
    }
}
