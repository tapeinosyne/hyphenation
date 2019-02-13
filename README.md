# `hyphenation`

Hyphenation for UTF-8 strings in a variety of languages.

```toml
[dependencies]
hyphenation = "0.7.1"
```

Two strategies are available:
- Standard Knuth–Liang hyphenation, with dictionaries built from the [TeX UTF-8 patterns](http://www.ctan.org/tex-archive/language/hyph-utf8).
- Extended (“non-standard”) hyphenation based on László Németh's [Automatic non-standard hyphenation in OpenOffice.org](https://www.tug.org/TUGboat/tb27-1/tb86nemeth.pdf), with dictionaries built from Libre/OpenOffice patterns.


## Documentation

[Docs.rs](https://docs.rs/hyphenation)


## Usage

### Quickstart

The `hyphenation` library relies on hyphenation dictionaries, external files that must be loaded into memory. To start with, however, it can be more convenient to embed them in the compiled artifact.

```toml
[dependencies]
hyphenation = { version = "0.7.1", features = ["embed_all"] }
```

The topmost module of `hyphenation` offers a small prelude that can be imported to expose the most common functionality.

```rust
use hyphenation::*;

// Retrieve the embedded American English dictionary for `Standard` hyphenation.
let en_us = Standard::from_embedded(Language::EnglishUS) ?;

// Identify valid breaks in the given word.
let hyphenated = en_us.hyphenate("hyphenation");

// Word breaks are represented as byte indices into the string.
let break_indices = &hyphenated.breaks;
assert_eq!(break_indices, &[2, 6]);

// The segments of a hyphenated word can be iterated over.
let segments = hyphenated.into_iter();
let collected : Vec<String> = segments.collect();
assert_eq!(collected, vec!["hy", "phen", "ation"]);

/// `hyphenate()` is case-insensitive.
let uppercase : Vec<_> = en_us.hyphenate("CAPITAL").into_iter().collect();
assert_eq!(uppercase, vec!["CAP", "I", "TAL"]);
```


### Loading dictionaries at runtime

The current set of available dictionaries amounts to ~7MB of data, the embedding of which is seldom desirable. Most applications should prefer to load individual dictionaries at runtime, like so:

```rust
let path_to_dict = "/path/to/en-us.bincode";
let english_us = Standard::from_path(Language::EnglishUS, path_to_dict) ?;
```

Dictionaries bundled with `hyphenation` can be retrieved from the build folder under `target`, and packaged with the final application as desired.

```bash
$ find target -name "dictionaries"
target/debug/build/hyphenation-33034db3e3b5f3ce/out/dictionaries
```


### Segmentation

Dictionaries can be used in conjunction with text segmentation to hyphenate words within a text run. This short example uses the [`unicode-segmentation`](https://crates.io/crates/unicode-segmentation) crate for untailored Unicode segmentation.

```rust
use unicode_segmentation::UnicodeSegmentation;

let hyphenate_text = |text : &str| -> String {
    // Split the text on word boundaries—
    text.split_word_bounds()
        // —and hyphenate each word individually.
        .flat_map(|word| en_us.hyphenate(word).into_iter())
        .collect()
};

let excerpt = "I know noble accents / And lucid, inescapable rhythms; […]";
assert_eq!("I know no-ble ac-cents / And lu-cid, in-escapable rhythms; […]"
          , hyphenate_text(excerpt));
```


### Normalization

Hyphenation patterns for languages affected by normalization occasionally cover multiple forms, at the discretion of their authors, but most often they don’t. If you require `hyphenation` to operate strictly on strings in a known normalization form, as described by the [Unicode Standard Annex #15](http://unicode.org/reports/tr15/) and provided by the [`unicode-normalization`](https://github.com/unicode-rs/unicode-normalization) crate, you may specify it in your Cargo manifest, like so:

```toml
[dependencies.hyphenation]
version = "0.7.1"
features = ["nfc"]
```

The `features` field may contain exactly *one* of the following normalization options:

- `"nfc"`, for canonical composition;
- `"nfd"`, for canonical decomposition;
- `"nfkc"`, for compatibility composition;
- `"nfkd"`, for compatibility decomposition.

It is recommended to build `hyphenation` in release mode if normalization is enabled, since the bundled hyphenation patterns will need to be reprocessed into dictionaries.


## License

`hyphenation` © 2016 tapeinosyne, dual-licensed under the terms of either:
  - the Apache License, Version 2.0
  - the MIT license

`texhyphen` and other hyphenation patterns © their respective owners; see `patterns/*.lic.txt` files for licensing information.
