#![allow(dead_code)]

#[cfg(any(feature = "nfc", feature = "nfd", feature = "nfkc", feature = "nfkd"))]
extern crate unicode_normalization;
#[cfg(feature = "embed_all")] extern crate pocket_resources;

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
use std::iter::{FromIterator};
use std::path::{Path, PathBuf};

use hyphenation_commons::dictionary::{self, *};
use hyphenation_commons::dictionary::extended as ext;
use hyphenation_commons::Language::{self, *};
use hyphenation_commons::parse::Parse;


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

    #[cfg(feature = "nfc")]  pub fn normalize(s : &str) -> String { s.nfc().collect() }
    #[cfg(feature = "nfd")]  pub fn normalize(s : &str) -> String { s.nfd().collect() }
    #[cfg(feature = "nfkc")] pub fn normalize(s : &str) -> String { s.nfkc().collect() }
    #[cfg(feature = "nfkd")] pub fn normalize(s : &str) -> String { s.nfkd().collect() }
}


trait TryFromIterator<Tally> : Sized {
    fn try_from_iter<I>(iter : I) -> Result<Self, Error>
    where I : IntoIterator<Item = (String, Tally)>;
}

impl TryFromIterator<<Patterns as Parse>::Tally> for Patterns {
    fn try_from_iter<I>(iter : I) -> Result<Self, Error>
    where I : IntoIterator<Item = (String, <Patterns as Parse>::Tally)> {
        Ok(Patterns::from_iter(iter) ?)
    }
}

impl TryFromIterator<<Exceptions as Parse>::Tally> for Exceptions {
    fn try_from_iter<I>(iter : I) -> Result<Self, Error>
    where I : IntoIterator<Item = (String, <Exceptions as Parse>::Tally)> {
        Ok(Exceptions(HashMap::from_iter(iter)))
    }
}

impl TryFromIterator<<ext::Patterns as Parse>::Tally> for ext::Patterns {
    fn try_from_iter<I>(iter : I) -> Result<Self, Error>
    where I : IntoIterator<Item = (String, <ext::Patterns as Parse>::Tally)> {
        Ok(ext::Patterns::from_iter(iter) ?)
    }
}


// Dictionary building and serialization

#[derive(Clone, Debug)]
struct Paths {
    source : PathBuf,
    out : PathBuf
}

impl Paths {
    fn new() -> Result<Self, Error> {
        let source = env::var("CARGO_MANIFEST_DIR").map(|p| PathBuf::from(p)) ?;
        let out = env::var("OUT_DIR").map(|p| PathBuf::from(p)) ?;

        Ok(Paths { source, out })
    }

    fn place_item<P : AsRef<Path>>(&self, p : P) -> PathBuf { self.out.join(p.as_ref()) }
    fn source_item<P : AsRef<Path>>(&self, p : P) -> PathBuf { self.source.join(p.as_ref()) }

    fn source_pattern(&self, lang : Language, suffix : &str) -> PathBuf {
        let fname = format!("hyph-{}.{}.txt", lang.code(), suffix);
        self.source_item("patterns").join(fname)
    }

    fn place_dict(&self, lang : Language, suffix : &str) -> PathBuf {
        self.place_item("dictionaries").join(Self::dict_name(lang, suffix))
    }

    fn dict_name(lang : Language, suffix : &str) -> String {
        format!("{}.{}.bincode", lang.code(), suffix)
    }
}


trait Build : Sized + Parse + TryFromIterator<<Self as Parse>::Tally> {
    fn suffix() -> &'static str;

    fn sourcepath(lang : Language, paths : &Paths) -> PathBuf {
        paths.source_pattern(lang, Self::suffix())
    }

    fn build(lang : Language, paths : &Paths) -> Result<Self, Error> {
        let file = File::open(Self::sourcepath(lang, paths)) ?;
        let by_line = io::BufReader::new(file).lines();
        let pairs : Vec<_> = by_line.map(|res| Self::pair(&res.unwrap(), normalize)).collect();

        Self::try_from_iter(pairs.into_iter())
    }
}

impl Build for Patterns   { fn suffix() -> &'static str { "pat" } }
impl Build for Exceptions { fn suffix() -> &'static str { "hyp" } }
impl Build for ext::Patterns { fn suffix() -> &'static str { "ext" } }


fn write<T>(item : &T, path : &Path) -> Result<(), Error> where T : ser::Serialize {
    let mut buffer = File::create(&path).map(|f| io::BufWriter::new(f)) ?;
    bin::config().limit(5_000_000).serialize_into(&mut buffer, item) ?;
    Ok(())
}

fn copy_dir(from : &Path, to : &Path) -> Result<(), Error> {
    for entry in fs::read_dir(from) ? {
        entry.and_then(|e| fs::copy(e.path(), to.join(e.file_name()))) ?;
    }

    Ok(())
}


fn main() {
    let dict_folder = Path::new("dictionaries");
    let _std_out = "standard";
    let _ext_out = "extended";
    let paths = Paths::new().unwrap();
    let dict_source = paths.source_item(dict_folder);
    let dict_out = paths.place_item(dict_folder);

    let _ext_langs = vec![Catalan, Hungarian];
    let _std_langs =
        vec![ Afrikaans, Armenian, Assamese, Basque, Belarusian, Bengali, Bulgarian, Catalan,
              Chinese, Coptic, Croatian, Czech, Danish, Dutch, EnglishGB, EnglishUS, Esperanto,
              Estonian, Ethiopic, Finnish, French, Friulan, Galician, Georgian, German1901,
              German1996, GermanSwiss, GreekAncient, GreekMono, GreekPoly, Gujarati, Hindi,
              Hungarian, Icelandic, Indonesian, Interlingua, Irish, Italian, Kannada, Kurmanji,
              Latin, LatinClassic, LatinLiturgical, Latvian, Lithuanian, Macedonian, Malayalam,
              Marathi, Mongolian, NorwegianBokmal, NorwegianNynorsk, Occitan, Oriya, Pali,
              Panjabi, Piedmontese, Polish, Portuguese, Romanian, Romansh, Russian, Sanskrit,
              SerbianCyrillic, SerbocroatianCyrillic, SerbocroatianLatin, SlavonicChurch, Slovak,
              Slovenian, Spanish, Swedish, Tamil, Telugu, Thai, Turkish, Turkmen, Ukrainian,
              Uppersorbian, Welsh ];

    fs::create_dir_all(&dict_out).unwrap();

    // If no dictionary is to be rebuilt, copy the bundled ones into the `target`
    // folder.
    #[cfg(not(any(feature = "build_dictionaries", feature = "nfc", feature = "nfd",
                  feature = "nfkc", feature = "nfkd")))]
    {
        copy_dir(dict_source.as_path(), dict_out.as_path()).unwrap();
    }

    // Otherwise, process the bundled patterns into new dictionaries and similarly
    // bundle them.
    #[cfg(any(feature = "build_dictionaries", feature = "nfc", feature = "nfd",
              feature = "nfkc", feature = "nfkd"))]
    {
        println!("Building `Standard` dictionaries:");
        for &language in _std_langs.iter() {
            println!("  - {:?}", language);
            let builder = Builder {
                language,
                patterns : Patterns::build(language, &paths).unwrap(),
                exceptions : Exceptions::build(language, &paths).unwrap_or(Exceptions::default())
            };

            let dict = Standard::from(builder);
            write(&dict, &paths.place_dict(language, _std_out)).unwrap();
        }

        println!("Building `Extended` dictionaries:");
        for &language in _ext_langs.iter() {
            println!("  - {:?}", language);
            let builder = ext::Builder {
                language,
                patterns : ext::Patterns::build(language, &paths).unwrap(),
                exceptions : ext::Exceptions::default()
            };

            let dict = ext::Extended::from(builder);
            write(&dict, &paths.place_dict(language, _ext_out)).unwrap();
        }
    }

    #[cfg(all(feature = "embed_en-us", not(feature = "embed_all")))] {
        use std::iter;

        let dict = (&dict_folder, Paths::dict_name(EnglishUS, _std_out));
        pocket_resources::package(iter::once(&dict)).unwrap();
    }

    #[cfg(feature = "embed_all")] {
        // HEED: `pocket_resources` requires paths to be relative
        let std_p = _std_langs.iter().map(|&l| (&dict_folder, Paths::dict_name(l, _std_out)));
        let ext_p = _ext_langs.iter().map(|&l| (&dict_folder, Paths::dict_name(l, _ext_out)));
        let all_paths : Vec<_> = std_p.chain(ext_p).collect();
        pocket_resources::package(all_paths.iter()).unwrap();
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
    Resource
    // TODO: Parsing
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
            Error::Resource => f.write_str("dictionary could not be embedded")
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
