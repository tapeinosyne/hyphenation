# hyphenation
Standard Knuth-Liang hyphenation based on the [TeX UTF-8 patterns](http://www.ctan.org/tex-archive/language/hyph-utf8).

```toml
[dependencies]
hyphenation = "0.4.1"
```

[Documentation](https://ndr-qef.github.io/hyphenation)


## Quickstart

```rust
use hyphenation::{Hyphenation, Standard};
use hyphenation::Language::{English_US};

// Load hyphenation data for American English from the pattern repository.
let english_us = hyphenation::load(English_US).unwrap();

// An iterator that breaks a word according to standard hyphenation practices.
let h: Standard = "hyphenation".hyphenate(&english_us);
                // hy-phen-ation

// Collect the lazy hyphenator `h` into substring slices over the original string.
let v: Vec<&str> = h.collect();
assert_eq!(v, vec!["hy", "phen", "ation"]);

// Hyphenators work with full text as well as individual words.
let h2: Standard = "Word hyphenation by computer.".hyphenate(&english_us);
let v2: Vec<&str> = h2.collect();
assert_eq!(v2, vec!["Word hy", "phen", "ation by com", "puter."]);

// Mark hyphenation opportunities with soft hyphens,
// and render the result to a new String.
let h3 = "anfractuous".hyphenate(&english_us);
let s3: String = h3.punctuate().collect();
assert_eq!(s3, "an\u{ad}frac\u{ad}tu\u{ad}ous".to_owned());
```


### Unicode Normalization
`hyphenation` operates on strings in Normalization Form C, as described by the [Unicode Standard Annex #15](http://unicode.org/reports/tr15/) and provided by the [`unicode-normalization`](https://github.com/unicode-rs/unicode-normalization) crate.

This form is ubiquitous, and you probably need not worry about it. Nevertheless, it would be best to ensure NFC when working with any of the following languages:

- Assamese
- Bengali
- Greek (Ancient, Monotonic, Polytonic)
- Punjabi
- Sanskrit


## Pattern Data

The script used to parse, normalize, and convert the TeX hyphenation patterns may be found at [ndr-qef/hyph-utf8.json](https://github.com/ndr-qef/hyph-utf8.json).


## License
`hyphenation` © 2016 ndr-qef, dual-licensed under the terms of either:
  - The Apache License, Version 2.0
  - The MIT license

`texhyphen` hyphenation patterns © their respective owners; see `lic.txt` files for licensing information.
