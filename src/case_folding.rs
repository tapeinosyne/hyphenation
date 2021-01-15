/*! Special casing for the purpose of hyphenation

Implemented here is a "refolding" pass which covers complementary case
mappings (rarely) required for the correct hyphenation of uppercase or
mixed-case words.

Patterns maintained by the TeX project are not generated in strict accordance
with Unicode case folding; rather, they include multiple concrete sequences
at the discretion of the pattern developer, under the assumption that words
may be lowercased with varying degrees of Unicode awareness.

Although the approach works quite well when matching against `str`, two
discrepancies remain:

- Firstly, folding may shift character boundaries, invalidating the indices
of opportunities found by our dictionaries, which are byte-based and have
no notion of `char`. Thus, any opportunity found in a folded word must be
mapped back to its correct position in the original, unfolded word.
- Secondly, patterns may not account for all concrete sequences that occur
when using `str::to_lowercase` as a loose folding pass.

A zealous solution would be to adopt proper caseless matching, and pre-fold
the bundled patterns. Presently, however, we rely on an ad-hoc remedy based
on supplementary case mappings.


# Supplementary case mappings

| uppercase            | lowercase            | refold
|----------------------|----------------------|----------------
|'İ' \u{130}, length 2 | "i\u{307}", length 3 | "i", length 1

The Turkish `İ` in its recomposed form is – to the author's knowledge –
the only codepoint which changes size when lowercased without normalization
or tailoring. It is thus necessary to fold it regardless of language or
context, because the equivalence-preserving lowercase "i\u{307}" not only
disrupts pattern matching – be it byte-based or char-based – but also
shifts and invalidates any opportunity arising after it.
*/

use std::borrow::Cow;
use std::borrow::Cow::*;


#[derive(Copy, Clone, Debug)]
pub struct Shift {
    index : usize,
    delta : isize,
}

/// The opportunity `i`, index-corrected for use in the original string.
pub fn realign(i : usize, shifts : &[Shift]) -> usize {
    (i as isize - shift_at(i, shifts)) as usize
}

/// The shift at index `i` in the refolded string.
fn shift_at(i : usize, shifts : &[Shift]) -> isize {
    shifts.iter()
          .rev()
          .find(|&&shift| i > shift.index)
          .map_or(0, |&shift| shift.delta)
}

fn shifts(word : &str) -> Vec<Shift> {
    word.match_indices("İ")
        .map(|(i, _)| (i, -1))
        .scan(0, |delta, (i, d)| {
            let index = i as isize + *delta;
            *delta += d;
            Some(Shift { index : index as usize,
                         delta : *delta, })
        })
        .collect()
}


/// Should the original string contain special-cased codepoints, refold it
/// for hyphenation and provide the induced index shifts. Otherwise, merely
/// ensure that it is lowercase.
pub fn refold(original : &str) -> (Cow<str>, Vec<Shift>) {
    if original.chars().any(|c| c.is_uppercase()) {
        let lowercase = original.to_lowercase();
        // There is only one code point which changes size when lowercased,
        // thus comparing string lengths is sufficient to determine whether
        // it was present in the original word.
        if original.len() != lowercase.len() {
            (Owned(refold_lowercase(&lowercase)), shifts(original))
        } else {
            (Owned(lowercase), vec![])
        }
    } else {
        (Borrowed(original), vec![])
    }
}

/// Substitute lowercase sequences that would interfere with hyphenation.
/// Canonical equivalence is not necessarily preserved.
fn refold_lowercase(lowercase : &str) -> String { lowercase.replace("i\u{307}", "i") }
