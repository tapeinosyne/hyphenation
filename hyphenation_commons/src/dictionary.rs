//! Data structures for the storage of hyphenation patterns and exceptions.

use atlatl::fst::FST;
use std::collections::HashMap;
use language::Language;


#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Locus {
    pub index : u8,
    pub value : u8
}

/// A trie mapping hyphenation patterns to their tallies.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Patterns {
    pub tallies : Vec<Vec<Locus>>,
    pub automaton : FST<u32, u16>
}

/// A specialized hashmap associating words to their known hyphenation.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exceptions(pub HashMap<String, Vec<usize>>);

/// A dictionary for standard Knuth–Liang hyphenation.
///
/// It comprises the working language, the pattern and exception sets,
/// and the character boundaries for hyphenation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Standard {
    pub language : Language,
    pub patterns : Patterns,
    pub exceptions : Exceptions,
    /// The minimum number of `char`s from the start and end of a word where breaks
    /// may not occur.
    pub minima : (usize, usize)
}


// Extended hyphenation

pub use self::extended::Extended;

pub mod extended {
    use atlatl::fst::FST;
    use std::collections::HashMap;

    use language::Language;
    use super::Locus;

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
    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Patterns {
        pub tallies : Vec<Tally>,
        pub automaton : FST<u32, u16>
    }

    /// A specialized hashmap associating words to their known hyphenation.
    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Exceptions(pub HashMap<String, Vec<(usize, Option<Subregion>)>>);

    /// A dictionary for extended Knuth–Liang hyphenation, based on the strategy
    /// described by Németh in "Automatic non-standard hyphenation in OpenOffice.org".
    ///
    /// It comprises the working language, the set of extended patterns and
    /// exceptions, and the character boundaries for hyphenation.
    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Extended {
        pub language : Language,
        pub patterns : Patterns,
        pub exceptions : Exceptions,
        /// The minimum number of `char`s from the start and end of a word where
        /// breaks may not occur.
        pub minima: (usize, usize)
    }
}
