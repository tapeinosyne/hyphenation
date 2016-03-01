//! Hyphenating iterators.

use std::borrow::Cow;

use unicode_segmentation::UnicodeSegmentation;

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
/// delimited by string boundaries and valid hyphenation points.
#[derive(Clone, Debug)]
pub struct Standard<'a> {
    text: &'a str,
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
                Some(&self.text[start .. end])
            },
            None => {
                if i <= self.opportunities.len() {
                    self.i = i + 1;
                    Some(&self.text[start ..])
                } else {
                    None
                }
            }
        }
    }
}


impl<'a> Hyphenation<Standard<'a>> for &'a str {
    fn opportunities(self, corp: &Corpus) -> Vec<usize> {
        let (l_min, r_min) = (corp.left_min, corp.right_min);
        let length_min = l_min + r_min;

        if self.chars().count() < length_min {
            return vec![];
        }

        let by_word = self.split_word_bound_indices();

        by_word.flat_map(|(i, word)| {
            let pts = match corp.exceptions.iter()
                                .filter_map(|exs| exs.score(word))
                                .next() {
                    Some(vec) => Cow::Borrowed(vec),
                    None => Cow::Owned(corp.patterns.score(word))
            }.into_owned();
            let length = pts.len();
            let l = l_min;
            let r = if length >= length_min { length - l_min - r_min + 1 } else { 0 };

            word.char_indices().skip(l)
                .zip(pts.into_iter().skip(l).take(r))
                .filter(|&(_, p)| p % 2 != 0)
                .map(move |((i1, _), _)| i1 + i)
        }).collect()
    }


    fn hyphenate(self, corp: &Corpus) -> Standard<'a> {
        let os = self.opportunities(corp);

        Standard {
            text: self,
            opportunities: os,
            prior: 0,
            i: 0
        }
    }
}
