/*! A library for the hyphenation of UTF-8 strings

## Usage

A typical import comprises the [`Hyphenator`] trait, the [`Standard`]
dictionary type, and the [`Language`] enum. This exposes the crate's core
functionality, as well as the set of available languages.

```ignore
extern crate hyphenation;

use hyphenation::{Hyphenator, Standard, Language};
```

To begin with, we must initiate the hyphenation dictionary for our working
language. Dictionaries come bundled with the `hyphenation` crate, but they
must still be loaded into memory. The most convenient way to do so is the
[`Load`] trait.

```ignore
use hyphenation::Load;

let path_to_dict = "/path/to/english-dictionary.bincode";
let en_us = Standard::from_path(Language::EnglishUS, path_to_dict) ?;
```

Our English dictionary can now be used as a [`Hyphenator`].


### Hyphenators

As the primary interface of this library, hyphenators take care of seeking
out opportunities for hyphenation within individual words.

```ignore
let hyphenated = en_us.hyphenate("anfractuous");
```

The [`hyphenate`] method computes the indices of valid word breaks and wraps
them in a a small intermediate structure that can be further used to [iterate]
over word segments.

```ignore
let breaks = &hyphenated.breaks;
assert_eq!(breaks, &[2, 6, 8]);

let hyphenated_segments : Vec<&str>= hyphenated.iter().collect()
assert_eq!(hyphenated_segments, &["an-", "frac-", "tu-", "ous"]);
```

Both the [`Standard`] and [`Extended`] hyphenators are case-insensitive and
prioritize existing soft hyphens (U+00AD) over dictionary hyphenation.

```ignore
let word = "ribonuclease";
let word_shy = "ri\u{00ad}bo\u{00ad}nu\u{00ad}cle\u{00ad}ase";

let by_dictionary : Vec<&str> = en_us.hyphenate(word).into_iter().segments().collect();
let by_shy : Vec<&str> = en_us.hyphenate(word_shy).into_iter().segments().collect();

assert_eq!(by_dictionary, vec!["ri", "bonu", "cle", "ase"]);
assert_eq!(by_shy, vec!["ri", "\u{00ad}bo", "\u{00ad}nu", "\u{00ad}cle", "\u{00ad}ase"]);
assert_ne!(by_dictionary, by_shy);
```


## Identifying "words"

Knuth–Liang hyphenation operates at the level of individual words, but there
can be ambiguity as to what constitutes a *word*. All hyphenation dictionaries
handle the expected set of word-forming graphemes from their respective
alphabets, but some also accept punctuation marks such as hyphens and
apostrophes, and are thus capable of handling hyphen-joined compound words or
elisions. Even so, it's generally preferable to handle punctuation at the
level of segmentation, as it affords greater control over the final result
(such as where to break hyphen-joined compounds, or whether to set a leading
hyphen on new lines).


[`Hyphenator`]: hyphenator/trait.Hyphenator.html
[`Standard`]: struct.Standard.html
[`Language`]: enum.Language.html
[`Load`]: load/trait.Load.html
[`hyphenate`]: hyphenator/trait.Hyphenator#tymethod.hyphenate.html
[iterate]: iter/struct.Hyphenating.html
[`Extended`]: extended/struct.Extended.html
*/

extern crate fst;
extern crate bincode;
extern crate hyphenation_commons;


#[cfg(feature = "embed_all")] mod resources;
mod case_folding;
pub mod hyphenator;
pub mod extended;
pub mod iter;
pub mod load;
pub mod score;

pub use hyphenation_commons::Language;
pub use hyphenation_commons::dictionary::Standard;
pub use hyphenator::Hyphenator;
pub use iter::Iter;
pub use load::Load;
