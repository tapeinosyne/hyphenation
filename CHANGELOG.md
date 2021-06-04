# `hyphenation`

## 0.8.3

Supported languages now include:
  - Albanian (`sq`)
  - Scholastic Finnish (`fi-x-school`)

Updated source hyphenation patterns to CTAN version 2021-03-22:
  - Updates and fixes to Macedonian (`mk`)
  - Updates to German (`de-1901`, `de-1996`, `de-ch-1901`)


## 0.8.2

- `unsafe` code is now forbidden crate-wide.
- The `Standard` dictionary implements `Clone`.
- Moved to Rust 2018.
- Updated dependencies.


## 0.8.1 — *yanked*


## 0.8.0

Language support:
- New language: Macedonian [mk]
- `hyph-utf8` patterns updated to CTAN version 2020.03.25:
  - Major update to Spanish [es] patterns
  - Major update to Classical Latin patterns [la-x-classic]

Library changes:
- Dictionaries now have a much more compact representation. Although hyphenation is ~3x slower, there should be no discernible difference in typical workloads.
- `hyphenator::exact` has been renamed to `hyphenator::exception`, in accordance with TeX conventions.
- Exceptions can now be removed from dictionaries with `Hyphenator::remove_exception()`

Fixes:
- `hyphenation` will no longer attempt to rebuild (#19) upon compilation of a dependant.
- `Hyphenator::exception()` (née `exact()`) no longer ignores hyphenation boundaries (#20) set by the dictionary. The old behavior can be replicated with [`exception_within()`](https://docs.rs/hyphenation/0.8.0/hyphenation/hyphenator/trait.Hyphenator.html#tymethod.exception_within).


## 0.7.1

- Backward compatibility with Rust 1.21.0 and earlier.
- Downstream dependency: `num-traits` 0.1 → 0.2


## 0.7.0

This release brings [extended hyphenation](https://www.tug.org/TUGboat/tb27-1/tb86nemeth.pdf), a substantial API reform, and a more flexible approach to dictionary loading.


### Changes

- API reform.

  The `hyphenation` API is now based on concrete dictionary types (“hyphenators”), and includes a richer set of methods and capabilities. Refer to the [documentation](https://docs.rs/hyphenation) for more.

- Full-text hyphenation has been removed. To hyphenate words within text runs, see the [notes on segmentation](https://docs.rs/hyphenation/0.7.0/hyphenation/#segmentation).
- Hyphenation patterns are not normalized by default. Normalization is now behind [feature flags](https://docs.rs/hyphenation/0.7.0/hyphenation/#normalization).
- Hyphenation dictionaries are bundled with the artifact, and need only be rebuilt if normalization is required.
- Hyphenation dictionaries are not embedded by default; to enable embedding, use the `embed_all` feature.


### New Features

- Extended (“non-standard”) hyphenation, available for Hungarian and Catalan. Extended hyphenation can improve orthography by altering or inserting characters around word breaks.
- Supported languages now include:
  - Belarusian (be)
  - Ecclesiastical Latin (la-x-liturgic)
  - Pāli (pi)


### Fixes

- Special casing

  Words containing characters that change byte size when lowercased (the Turkish “İ”) are now hyphenated correctly.


### Improvements

- Hyphenation dictionaries are ~40% smaller.
- Hyphenation is ~20% faster.


### Dependencies:

- `serde` → 1.0
- `bincode` → 1.0


## 0.6.1

- Slightly reduced embedded pattern size.


## 0.6.0

- Unicode normalization can be configured via build flags.
- Patterns are now serialized by [`bincode`](https://github.com/TyOverby/bincode) at build time.
- Hyphenation is faster by:
  - 10–20%, generally;
  - 40–60%, for lowercase ASCII words (in a mostly ASCII language).
- Embedded pattern size increased by ~25%.
- The `Standard` hyphenating iterator implements `size_hint()` and `ExactSizeIterator`.


## 0.5.0

- Reverted from previous change in 0.3.0: the `Hyphenation` trait only performs word hyphenation. Full text hyphenation is now part of the `FullTextHyphenation` trait.
- Language support: added Church Slavonic.


## 0.4.1

- Updated patterns to [hyph-utf8.json](https://github.com/ndr-qef/hyph-utf8.json) v0.1.0.0.

  Previously, patterns containing multibyte letters with combining marks could be mapped to mismatching scores of incorrect length, due to normalization issues. The following languages were affected:
  - Assamese
  - Bengali
  - Panjabi
  - Sanskrit


## 0.4.0

- The pattern repository is now bundled with the crate, and no longer requires manual initialization.


## 0.3.2

- Hyphenation of lowercase words is ~10% faster.
- `Patterns` and `Exceptions` now return preexisting values on `insert()`, analogously to HashMap.


## 0.3.1

- Language loading time is halved.


## 0.3.0

- Word hyphenation is ~15% faster.
- The `Hyphenation` trait accepts full text as well as individual words.
- `Standard` hyphenators expose a `punctuate_with()` method.


## 0.2.0

- Loading a language is ~30% faster.
- Hyphenation patterns are stored as JSON.
