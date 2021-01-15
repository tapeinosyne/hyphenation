/*!
Data structures for extended hyphenation[1].

[1]: [Automatic non-standard hyphenation in OpenOffice.org](https://www.tug.org/TUGboat/tb27-1/tb86nemeth.pdf)
*/

use std::collections::HashMap;

use dictionary::trie::{self, Trie};
use dictionary::{uniques, Locus, PrefixTallies};
use language::Language;
use parse::Parse;

/// The partial score carried by an extended hyphenation pattern.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Tally {
    /// The pattern tally, equivalent to that found in standard patterns.
    pub standard :  Vec<Locus>,
    /// An optional subregion which may replace part of the string around the
    /// opportunity.
    pub subregion : Option<(Locus, Subregion)>,
}

/// Word alterations extending a standard Knuth–Liang pattern.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Subregion {
    /// The number of bytes that the substitution will replace before the break.
    pub left :         usize,
    /// The number of bytes that the substitution will replace after the break.
    pub right :        usize,
    /// The replacement for the substring to be altered around the break, as
    /// delimited by the `left` and `right` subregion boundaries.
    pub substitution : String,
    /// An index into the substitution, denoting the hyphenation opportunity
    /// within this subregion.
    pub breakpoint :   usize,
}

/// A trie mapping hyphenation patterns to their extended tallies.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Patterns {
    tallies :   Vec<Tally>,
    automaton : Trie,
}

impl Patterns {
    pub fn from_iter<I>(iter : I) -> Result<Self, trie::Error>
        where I : IntoIterator<Item = (String, <Patterns as Parse>::Tally)>
    {
        let (kvs, tallies) = uniques(iter.into_iter());
        let automaton = Trie::from_iter(kvs.into_iter())?;
        Ok(Patterns { tallies, automaton })
    }
}


/// A specialized hashmap associating words to their known hyphenation.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exceptions(pub HashMap<String, Vec<(usize, Option<Subregion>)>>);

/// A dictionary for extended Knuth–Liang hyphenation, based on the strategy
/// described by Németh in "Automatic non-standard hyphenation in
/// OpenOffice.org".
///
/// It comprises the working language, the set of extended patterns and
/// exceptions, and the character boundaries for hyphenation.
#[derive(Debug, Serialize, Deserialize)]
pub struct Extended {
    language :       Language,
    patterns :       Patterns,
    pub exceptions : Exceptions,
    /// The minimum number of `char`s from the start and end of a word where
    /// breaks may not occur.
    pub minima :     (usize, usize),
}

impl Extended {
    /// The language for which this dictionary can provide hyphenation.
    pub fn language(&self) -> Language { self.language }

    /// An iterator over the tallies associated to all prefixes of the query,
    /// including the query itself.
    pub fn prefix_tallies<'f, 'q>(&'f self, query : &'q [u8]) -> PrefixTallies<'f, 'q, Tally> {
        PrefixTallies { matches : self.patterns.automaton.get_prefixes(query),
                        tallies : &self.patterns.tallies, }
    }
}

/// An intermediate dictionary builder, its primary purpose is visibility
/// hygiene.
#[derive(Debug)]
pub struct Builder {
    pub language :   Language,
    pub patterns :   Patterns,
    pub exceptions : Exceptions,
}

impl From<Builder> for Extended {
    fn from(b : Builder) -> Extended {
        Extended { language :   b.language,
                   patterns :   b.patterns,
                   exceptions : b.exceptions,
                   minima :     b.language.minima(), }
    }
}
