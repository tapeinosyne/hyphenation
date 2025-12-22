#![allow(dead_code)]

#[cfg(any(feature = "embed_all", feature = "embed_selected", feature = "embed_en-us"))]
extern crate pocket_resources;
#[cfg(any(feature = "nfc", feature = "nfd", feature = "nfkc", feature = "nfkd"))]
extern crate unicode_normalization;

extern crate bincode;
extern crate fst;
extern crate hyphenation_commons;
extern crate serde;

use bincode as bin;
use serde::ser;
use std::collections::HashMap;
use std::env;
use std::error;
use std::fmt;
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

use hyphenation_commons::dictionary::extended as ext;
use hyphenation_commons::dictionary::{self, *};
use hyphenation_commons::parse::Parse;
use hyphenation_commons::Language::{self, *};

#[cfg(feature = "embed_compressed")]
use flate2::{write::GzEncoder, Compression};

// Configuration of exclusive optional features

use configuration::*;
mod configuration {
    // In service of configurable normalization forms, a type alias and a function
    // are defined via conditional compilation.
    //
    // If no feature is explicitly set, normalization is avoided altogether.

    // Neither Cargo nor rustc allows us to set exclusive features; we must indulge
    // them with this clumsy branle of cfg declarations.
    #[cfg(not(any(feature = "nfc", feature = "nfd", feature = "nfkc", feature = "nfkd")))]
    pub fn normalize(s : &str) -> String { s.to_owned() }

    #[cfg(any(feature = "nfc", feature = "nfd", feature = "nfkc", feature = "nfkd"))]
    use unicode_normalization::*;

    #[cfg(feature = "nfc")]
    pub fn normalize(s : &str) -> String { s.nfc().collect() }
    #[cfg(feature = "nfd")]
    pub fn normalize(s : &str) -> String { s.nfd().collect() }
    #[cfg(feature = "nfkc")]
    pub fn normalize(s : &str) -> String { s.nfkc().collect() }
    #[cfg(feature = "nfkd")]
    pub fn normalize(s : &str) -> String { s.nfkd().collect() }
}


trait TryFromIterator<Tally>: Sized {
    fn try_from_iter<I>(iter : I) -> Result<Self, Error>
        where I : IntoIterator<Item = (String, Tally)>;
}

impl TryFromIterator<<Patterns as Parse>::Tally> for Patterns {
    fn try_from_iter<I>(iter : I) -> Result<Self, Error>
        where I : IntoIterator<Item = (String, <Patterns as Parse>::Tally)>
    {
        Ok(Patterns::from_iter(iter)?)
    }
}

impl TryFromIterator<<Exceptions as Parse>::Tally> for Exceptions {
    fn try_from_iter<I>(iter : I) -> Result<Self, Error>
        where I : IntoIterator<Item = (String, <Exceptions as Parse>::Tally)>
    {
        Ok(Exceptions(HashMap::from_iter(iter)))
    }
}

impl TryFromIterator<<ext::Patterns as Parse>::Tally> for ext::Patterns {
    fn try_from_iter<I>(iter : I) -> Result<Self, Error>
        where I : IntoIterator<Item = (String, <ext::Patterns as Parse>::Tally)>
    {
        Ok(ext::Patterns::from_iter(iter)?)
    }
}


// Dictionary building and serialization

#[derive(Clone, Debug)]
struct Paths {
    source : PathBuf,
    out :    PathBuf,
}

impl Paths {
    fn new() -> Result<Self, Error> {
        let source = env::var("CARGO_MANIFEST_DIR").map(PathBuf::from)?;
        let out = env::var("OUT_DIR").map(PathBuf::from)?;

        Ok(Paths { source, out })
    }

    fn place_item<P : AsRef<Path>>(&self, p : P) -> PathBuf { self.out.join(p.as_ref()) }
    fn source_item<P : AsRef<Path>>(&self, p : P) -> PathBuf { self.source.join(p.as_ref()) }

    fn source_pattern(&self, lang : Language, suffix : &str) -> PathBuf {
        let fname = format!("hyph-{}.{}.txt", lang.code(), suffix);
        self.source_item("patterns").join(fname)
    }

    fn place_dict(&self, lang : Language, suffix : &str) -> PathBuf {
        self.place_item("dictionaries")
            .join(Self::dict_name(lang, suffix))
    }

    fn dict_name(lang : Language, suffix : &str) -> String {
        format!("{}.{}.bincode", lang.code(), suffix)
    }
}


trait Build: Sized + Parse + TryFromIterator<<Self as Parse>::Tally> {
    fn suffix() -> &'static str;

    fn sourcepath(lang : Language, paths : &Paths) -> PathBuf {
        paths.source_pattern(lang, Self::suffix())
    }

    fn build(lang : Language, paths : &Paths) -> Result<Self, Error> {
        let file = File::open(Self::sourcepath(lang, paths))?;
        let by_line = io::BufReader::new(file).lines();
        let pairs = by_line.map(|res| Self::pair(&res.unwrap(), normalize));

        Self::try_from_iter(pairs.into_iter())
    }
}

impl Build for Patterns {
    fn suffix() -> &'static str { "pat" }
}
impl Build for Exceptions {
    fn suffix() -> &'static str { "hyp" }
}
impl Build for ext::Patterns {
    fn suffix() -> &'static str { "ext" }
}


fn write<T>(item : &T, path : &Path) -> Result<(), Error>
    where T : ser::Serialize
{
    let mut buffer = File::create(&path).map(io::BufWriter::new)?;
    bin::serialize_into(&mut buffer, item)?;
    Ok(())
}

fn copy_dir(from : &Path, to : &Path) -> Result<(), Error> {
    for entry in fs::read_dir(from)? {
        entry.and_then(|e| fs::copy(e.path(), to.join(e.file_name())))?;
    }

    Ok(())
}

#[cfg(feature = "embed_compressed")]
fn relative_to(base : &Path, target : &Path) -> PathBuf {
    use std::path::Component;

    if let Ok(stripped) = target.strip_prefix(base) {
        return stripped.to_path_buf();
    }

    let base_components : Vec<_> = base.components().collect();
    let target_components : Vec<_> = target.components().collect();

    if let (Some(Component::Prefix(b)), Some(Component::Prefix(t))) =
        (base_components.first(), target_components.first())
    {
        if b != t {
            panic!(
                "embed_compressed requires OUT_DIR to be on the same drive as CARGO_MANIFEST_DIR"
            );
        }
    }

    let mut i = 0;
    while i < base_components.len()
          && i < target_components.len()
          && base_components[i] == target_components[i]
    {
        i += 1;
    }

    let mut rel = PathBuf::new();
    for _ in i..base_components.len() {
        rel.push("..");
    }
    for comp in &target_components[i..] {
        rel.push(comp.as_os_str());
    }

    rel
}

#[cfg(feature = "embed_compressed")]
fn compress_dictionary(src : &Path, dst : &Path) -> io::Result<()> {
    let mut input = File::open(src)?;
    let output = File::create(dst)?;
    let mut encoder = GzEncoder::new(output, Compression::best());
    io::copy(&mut input, &mut encoder)?;
    encoder.finish()?;
    Ok(())
}

#[cfg(any(feature = "embed_all", feature = "embed_selected", feature = "embed_en-us"))]
fn package_dicts(_paths : &Paths, dict_folder : &Path, dict_out : &Path, dict_names : &[String]) {
    #[cfg(feature = "embed_compressed")]
    {
        let compressed_out = dict_out.join("compressed");
        fs::create_dir_all(&compressed_out).unwrap();

        for name in dict_names {
            println!("cargo:rerun-if-changed={}/{}", dict_folder.display(), name);
            let src = dict_out.join(name);
            let dst = compressed_out.join(name);
            compress_dictionary(&src, &dst).unwrap();
        }

        let base_rel = relative_to(&_paths.source, &compressed_out);
        let dicts : Vec<(PathBuf, PathBuf)> = dict_names
            .iter()
            .map(|name| (base_rel.clone(), PathBuf::from(name)))
            .collect();
        pocket_resources::package(dicts.iter()).unwrap();
    }

    #[cfg(not(feature = "embed_compressed"))]
    {
        let dicts : Vec<(PathBuf, PathBuf)> = dict_names
            .iter()
            .map(|name| (dict_folder.to_path_buf(), PathBuf::from(name)))
            .collect();
        pocket_resources::package(dicts.iter()).unwrap();
    }
}


fn main() {
    let dict_folder = Path::new("dictionaries");
    let _std_out = "standard";
    let _ext_out = "extended";
    let paths = Paths::new().unwrap();
    let _dict_source = paths.source_item(dict_folder);
    let dict_out = paths.place_item(dict_folder);

    let _ext_langs = vec![Catalan, Hungarian];
    let _std_langs = vec![Afrikaans,
                          Albanian,
                          Armenian,
                          Assamese,
                          Basque,
                          Belarusian,
                          Bengali,
                          Bulgarian,
                          Catalan,
                          Chinese,
                          Coptic,
                          Croatian,
                          Czech,
                          Danish,
                          Dutch,
                          EnglishGB,
                          EnglishUS,
                          Esperanto,
                          Estonian,
                          Ethiopic,
                          Finnish,
                          FinnishScholastic,
                          French,
                          Friulan,
                          Galician,
                          Georgian,
                          German1901,
                          German1996,
                          GermanSwiss,
                          GreekAncient,
                          GreekMono,
                          GreekPoly,
                          Gujarati,
                          Hindi,
                          Hungarian,
                          Icelandic,
                          Indonesian,
                          Interlingua,
                          Irish,
                          Italian,
                          Kannada,
                          Kurmanji,
                          Latin,
                          LatinClassic,
                          LatinLiturgical,
                          Latvian,
                          Lithuanian,
                          Macedonian,
                          Malayalam,
                          Marathi,
                          Mongolian,
                          NorwegianBokmal,
                          NorwegianNynorsk,
                          Occitan,
                          Oriya,
                          Pali,
                          Panjabi,
                          Piedmontese,
                          Polish,
                          Portuguese,
                          Romanian,
                          Romansh,
                          Russian,
                          Sanskrit,
                          SerbianCyrillic,
                          SerbocroatianCyrillic,
                          SerbocroatianLatin,
                          SlavonicChurch,
                          Slovak,
                          Slovenian,
                          Spanish,
                          Swedish,
                          Tamil,
                          Telugu,
                          Thai,
                          Turkish,
                          Turkmen,
                          Ukrainian,
                          Uppersorbian,
                          Welsh];

    fs::create_dir_all(&dict_out).unwrap();

    // If no dictionary is to be rebuilt, copy the bundled ones into the `target`
    // folder.
    #[cfg(not(any(feature = "build_dictionaries",
                  feature = "nfc",
                  feature = "nfd",
                  feature = "nfkc",
                  feature = "nfkd")))]
    {
        copy_dir(_dict_source.as_path(), dict_out.as_path()).unwrap();
    }

    // Otherwise, process the bundled patterns into new dictionaries and similarly
    // bundle them.
    #[cfg(any(feature = "build_dictionaries",
              feature = "nfc",
              feature = "nfd",
              feature = "nfkc",
              feature = "nfkd"))]
    {
        println!("Building `Standard` dictionaries:");
        for &language in _std_langs.iter() {
            println!("  - {:?}", language);
            let builder =
                Builder { language,
                          patterns : Patterns::build(language, &paths).unwrap(),
                          exceptions:
                              Exceptions::build(language, &paths).unwrap_or(Exceptions::default()) };

            let dict = Standard::from(builder);
            write(&dict, &paths.place_dict(language, _std_out)).unwrap();
        }

        println!("Building `Extended` dictionaries:");
        for &language in _ext_langs.iter() {
            println!("  - {:?}", language);
            let builder = ext::Builder { language,
                                         patterns:
                                             ext::Patterns::build(language, &paths).unwrap(),
                                         exceptions : ext::Exceptions::default() };

            let dict = ext::Extended::from(builder);
            write(&dict, &paths.place_dict(language, _ext_out)).unwrap();
        }
    }

    #[cfg(all(feature = "embed_en-us",
              not(any(feature = "embed_all", feature = "embed_selected"))))]
    {
        let dicts = vec![Paths::dict_name(EnglishUS, _std_out)];
        package_dicts(&paths, dict_folder, &dict_out, &dicts);
    }

    #[cfg(all(feature = "embed_selected", not(feature = "embed_all")))]
    {
        use std::collections::HashSet;

        println!("cargo:rerun-if-env-changed=HYPHENATION_EMBED");

        let raw = env::var("HYPHENATION_EMBED").unwrap_or_else(|_| {
            panic!(
                "HYPHENATION_EMBED must be set when `embed_selected` is enabled \
(example: \"en-us,de-1996,hu.extended\")"
            );
        });

        let std_set: HashSet<Language> = _std_langs.iter().copied().collect();
        let ext_set: HashSet<Language> = _ext_langs.iter().copied().collect();

        let mut dicts = Vec::new();
        for raw_item in raw.split(',') {
            let item = raw_item.trim();
            if item.is_empty() {
                continue;
            }

            let (code_raw, suffix_raw) = match item.split_once('.') {
                Some((code, suffix)) => (code, suffix),
                None => (item, _std_out),
            };
            let code = code_raw.trim().to_ascii_lowercase();
            let suffix = suffix_raw.trim().to_ascii_lowercase();

            let lang = Language::try_from_code(&code).unwrap_or_else(|| {
                panic!("Unknown language code `{}` in HYPHENATION_EMBED", code);
            });

            match suffix.as_str() {
                "standard" => {
                    if !std_set.contains(&lang) {
                        panic!("No standard dictionary for `{}`", code);
                    }
                }
                "extended" => {
                    if !ext_set.contains(&lang) {
                        panic!("No extended dictionary for `{}`", code);
                    }
                }
                _ => {
                    panic!(
                        "Unknown dictionary kind `{}` in HYPHENATION_EMBED \
(use `standard` or `extended`)",
                        suffix
                    );
                }
            }

            dicts.push(Paths::dict_name(lang, suffix.as_str()));
        }

        if dicts.is_empty() {
            panic!(
                "HYPHENATION_EMBED was set but empty; provide a comma-separated list \
like \"en-us,de-1996,hu.extended\""
            );
        }

        dicts.sort();
        dicts.dedup();
        package_dicts(&paths, dict_folder, &dict_out, &dicts);
    }

    #[cfg(feature = "embed_all")]
    {
        let mut dicts : Vec<String> = _std_langs
            .iter()
            .map(|&l| Paths::dict_name(l, _std_out))
            .collect();
        dicts.extend(_ext_langs.iter().map(|&l| Paths::dict_name(l, _ext_out)));
        package_dicts(&paths, dict_folder, &dict_out, &dicts);
    }

    println!("cargo:rerun-if-changed=build.rs");
}


// Error type boilerplate

#[derive(Debug)]
pub enum Error {
    Build(fst::Error),
    Env(env::VarError),
    IO(io::Error),
    Serialization(bin::Error),
    Resource, // TODO: Parsing
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Build(ref e) => Some(e),
            Error::Env(ref e) => Some(e),
            Error::IO(ref e) => Some(e),
            Error::Serialization(ref e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Build(ref e) => e.fmt(f),
            Error::Env(ref e) => e.fmt(f),
            Error::IO(ref e) => e.fmt(f),
            Error::Serialization(ref e) => e.fmt(f),
            Error::Resource => f.write_str("dictionary could not be embedded"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err : io::Error) -> Error { Error::IO(err) }
}

impl From<env::VarError> for Error {
    fn from(err : env::VarError) -> Error { Error::Env(err) }
}

impl From<bin::Error> for Error {
    fn from(err : bin::Error) -> Error { Error::Serialization(err) }
}

impl From<fst::Error> for Error {
    fn from(err : fst::Error) -> Error { Error::Build(err) }
}

impl From<dictionary::Error> for Error {
    fn from(err : dictionary::Error) -> Error { Error::Build(err.0) }
}
