## 0.4.1
- Updated patterns to `[hyph-utf8.json](https://github.com/ndr-qef/hyph-utf8.json)` v0.1.0.0.

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
