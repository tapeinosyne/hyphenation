//! Hyphenating iterators.

use unicode_segmentation::UnicodeSegmentation;

use klpattern::KLPTrie;
use language::{Corpus};
use utilia::{Interspersable, Intersperse};


pub trait Hyphenation<Hyphenator> where Hyphenator : Iterator {
    /// Returns the indices of valid hyphenation points within the given word.
    fn opportunities(self, corp: &Corpus) -> Vec<usize>;

    /// Returns an iterator over orthographic syllables of the given word,
    /// separated by valid hyphenation points.
    ///
    /// Note that, in some orthographies, the syllables of a hyphenated
    /// word are not necessarily substrings of the original word.
    fn hyphenate(self, corp: &Corpus) -> Hyphenator;
}

pub trait FullTextHyphenation<Hyphenator> : Hyphenation<Hyphenator>
    where Hyphenator : Iterator {
    /// Returns the indices of valid hyphenation points within the given text.
    fn fulltext_opportunities(self, corp: &Corpus) -> Vec<usize>;

    /// Returns an iterator over segments of the given text, separated by
    /// valid hyphenation points.
    fn fulltext_hyphenate(self, corp: &Corpus) -> Hyphenator;
}


/// The `Standard` hyphenator iterates over a string, returning slices
/// delimited by string boundaries and valid hyphenation points.
///
/// For individual words, such slices coincide with orthographic syllables.
#[derive(Clone, Debug)]
pub struct Standard<'a> {
    text: &'a str,
    opportunities: Vec<usize>,
    prior: usize,
    current: usize
}

impl<'a> Standard<'a> {
    /// Inserts a soft hyphen at hyphenation points.
    pub fn punctuate(self) -> Intersperse<Self> {
        self.intersperse("\u{ad}")
    }

    /// Inserts a given string at hyphenation points.
    pub fn punctuate_with(self, mark: &'a str) -> Intersperse<Self> {
        self.intersperse(mark)
    }
}


impl<'a> Iterator for Standard<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let start = self.prior;
        let current = self.current;

        match self.opportunities.get(current) {
            Some(&end) => {
                self.prior = end;
                self.current = current + 1;
                Some(&self.text[start .. end])
            },
            None => {
                if current <= self.opportunities.len() {
                    self.current = current + 1;
                    Some(&self.text[start ..])
                } else {
                    None
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.opportunities.len() + 1 - self.current;
        (remaining, Some(remaining))
    }
}

impl<'a> ExactSizeIterator for Standard<'a> {}


impl<'a> Hyphenation<Standard<'a>> for &'a str {
    /// Returns the byte indices of valid hyphenation points within the string.
    fn opportunities(self, corp: &Corpus) -> Vec<usize> {
        let (l_min, r_min) = (corp.left_min, corp.right_min);
        let length_min = l_min + r_min;

        if self.chars().count() < length_min {
            return vec![];
        }

        let score;
        match corp.exceptions.score(self) {
            None => score = corp.patterns.score(self),
            Some(known_score) => {
                let ops = known_score.iter()
                    .enumerate()
                    .filter(|&(_, &p)| p == 1)
                    .map(|(i, _)| i)
                    .collect();

                return ops;
            }
        };

        let cis = self.char_indices();
        let (l, r) = (cis.clone().skip(l_min).next().unwrap().0,
                      cis.rev().skip(r_min.saturating_sub(2)).next().unwrap().0);

        self.bytes()
            .enumerate().skip(1)
            .zip(score.as_slice())
            .filter(|&((i, _), p)| p % 2 != 0 && i >= l && i < r && self.is_char_boundary(i))
            .map(|((i, _), _)| i)
            .collect()
    }

    /// Returns an iterator over string slices separated by valid hyphenation
    /// points.
    fn hyphenate(self, corp: &Corpus) -> Standard<'a> {
        Standard {
            text: self,
            opportunities: self.opportunities(corp),
            prior: 0,
            current: 0
        }
    }
}

impl<'a> FullTextHyphenation<Standard<'a>> for &'a str {
    fn fulltext_opportunities(self, corp: &Corpus) -> Vec<usize> {
        let by_word = self.split_word_bound_indices();

        by_word.flat_map(|(i, word)| {
            word.opportunities(corp).into_iter().map(move |i1| i + i1)
        }).collect()
    }

    fn fulltext_hyphenate(self, corp: &Corpus) -> Standard<'a> {
        Standard {
            text: self,
            opportunities: self.fulltext_opportunities(corp),
            prior: 0,
            current: 0
        }

    }
}
