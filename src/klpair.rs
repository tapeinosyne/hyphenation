//! Internal intermediate representation for Knuth-Liang hyphenation patterns.


/// A pair representing a Knuth-Liang hyphenation pattern. It comprises
/// alphabetical characters for subword matching and the score of each
/// hyphenation point.
pub type KLPair = (String, Vec<u32>);
