# hyphenation

Standard Knuth-Liang hyphenation based on the [TeX UTF-8 patterns](http://www.ctan.org/tex-archive/language/hyph-utf8).

```toml
[dependencies]
hyphenation = "0.6"
```


## Documentation

[Docs.rs](https://docs.rs/hyphenation)


## Quickstart

```rust
use hyphenation::{Hyphenation, Standard};
use hyphenation::Language::{English_US};

// Load hyphenation data for American English from the pattern repository.
let english_us = hyphenation::load(English_US).unwrap();

// Compute the byte indices of valid hyphenation points within a word.
let indices = "hyphenation".opportunities(&english_us);
assert_eq!(indices, vec![2, 6]);

// Build an iterator that breaks a word according to standard hyphenation practices.
let h: Standard = "hyphenation".hyphenate(&english_us);

// Collect the lazy hyphenator `h` into substring slices over the original string.
let v: Vec<&str> = h.collect();
assert_eq!(v, vec!["hy", "phen", "ation"]);


// Hyphenation works with full text as well as individual words.
use hyphenation::FullTextHyphenation;

let text_indices = "Word hyphenation by computer.".fulltext_opportunities(&english_us);
assert_eq!(text_indices, vec![7, 11, 23]);

let h2: Standard = "Word hyphenation by computer.".fulltext_hyphenate(&english_us);
let v2: Vec<&str> = h2.collect();
assert_eq!(v2, vec!["Word hy", "phen", "ation by com", "puter."]);


// Mark hyphenation opportunities with soft hyphens,
// and render the result to a new String.
let h3 = "anfractuous".hyphenate(&english_us);
let s3: String = h3.punctuate().collect();
assert_eq!(s3, "an\u{ad}frac\u{ad}tu\u{ad}ous".to_owned());
```


### Unicode Normalization

For preference, `hyphenation` should operate on strings in a known *normalization form*, as described by the [Unicode Standard Annex #15](http://unicode.org/reports/tr15/) and provided by the [`unicode-normalization`](https://github.com/unicode-rs/unicode-normalization) crate. This is particularly important when working with non-ASCII languages that feature combining marks.

(Notably exempt from such concerns are `English_US` and `English_GB`, for which normalization is ordinarily inconsequential.)

The normalization form expected by `hyphenation` is determined at build time. By default, `hyphenation` is compiled to work with strings in Normalization Form C; you may specify another form in your Cargo manifest, like so:

```toml
[dependencies.hyphenation]
version = "0.6.0"
features = ["nfd"]
```

The `features` field takes a list containing the desired normalization form; namely, the value of `features` must be *one* of the following:

- `["none"]`, to use the [TeX UTF-8 patterns](http://www.ctan.org/tex-archive/language/hyph-utf8) as they are;
- `["nfc"]`, for canonical composition;
- `["nfd"]`, for canonical decomposition;
- `["nfkc"]`, for compatibility composition;
- `["nfkd"]`, for compatibility decomposition.


## License

`hyphenation` © 2016 tapeinosyne, dual-licensed under the terms of either:
  - The Apache License, Version 2.0
  - The MIT license

`texhyphen` hyphenation patterns © their respective owners; see `lic.txt` files for licensing information.
