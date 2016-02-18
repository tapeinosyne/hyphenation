# hyphenation
Standard Knuth-Liang hyphenation based on the [TeX UTF-8 patterns](http://www.ctan.org/tex-archive/language/hyph-utf8).

```toml
[dependencies]
hyphenation = "0.1.1"
```


## Quickstart

```rust
use hyphenation::{Hyphenation, Standard};
use hyphenation::Language::{English_US};

// Set the location of the pattern repository,
// absolute or relative to your executable
hyphenation::set_pattern_folder(Path::new("./patterns"));

// Load hyphenation data for American English from the pattern repository.
let english_us = hyphenation::load(English_US).unwrap();

// An iterator that breaks a word according to standard hyphenation practices.
let h: Standard = "hyphenation".hyphenate(&english_us);
                // hy-phen-ation

// Collect the lazy hyphenator `h` into substring slices over the original string.
let v: Vec<&str> = h.collect();
assert_eq!(v, vec!["hy", "phen", "ation"]);

// Mark hyphenation opportunities with soft hyphens,
// and render the result to a new String.
let h1 = "anfractuous".hyphenate(&english_us);
let s: String = h1.punctuate().collect();
assert_eq!(s, "an\u{ad}frac\u{ad}tu\u{ad}ous".to_owned());
```


## License
`hyphenation` © 2016 ndr-qef, dual-licensed under the terms of either:
  - The Apache License, Version 2.0
  - The MIT license

`texhyphen` hyphenation patterns © their respective owners; see `lic.txt` files for licensing information.
