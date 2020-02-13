/*!
Data structures for extended hyphenation[1].

[1]: [Automatic non-standard hyphenation in OpenOffice.org](https://www.tug.org/TUGboat/tb27-1/tb86nemeth.pdf)
*/

use std::collections::HashMap;

use dictionary::Locus;
use language::Language;
use dictionary::trie::Trie;

/// The partial score carried by an extended hyphenation pattern.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Tally {
    /// The pattern tally, equivalent to that found in standard patterns.
    pub standard : Vec<Locus>,
    /// An optional subregion which may replace part of the string around the
    /// opportunity.
    pub subregion : Option<(Locus, Subregion)>
}

/// Word alterations extending a standard Knuth–Liang pattern.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Subregion {
    /// The number of bytes that the substitution will replace before the break.
    pub left : usize,
    /// The number of bytes that the substitution will replace after the break.
    pub right : usize,
    /// The replacement for the substring to be altered around the break, as
    /// delimited by the `left` and `right` subregion boundaries.
    pub substitution : String,
    /// An index into the substitution, denoting the hyphenation opportunity
    /// within this subregion.
    pub breakpoint : usize,
}

/// A trie mapping hyphenation patterns to their extended tallies.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Patterns {
    pub tallies : Vec<Tally>,
    pub automaton : Trie
}

/// A specialized hashmap associating words to their known hyphenation.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exceptions(pub HashMap<String, Vec<(usize, Option<Subregion>)>>);

/// A dictionary for extended Knuth–Liang hyphenation, based on the strategy
/// described by Németh in "Automatic non-standard hyphenation in OpenOffice.org".
///
/// It comprises the working language, the set of extended patterns and
/// exceptions, and the character boundaries for hyphenation.
#[derive(Debug, Serialize, Deserialize)]
pub struct Extended {
    pub language : Language,
    pub patterns : Patterns,
    pub exceptions : Exceptions,
    /// The minimum number of `char`s from the start and end of a word where
    /// breaks may not occur.
    pub minima: (usize, usize)
}
