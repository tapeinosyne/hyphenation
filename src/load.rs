//! IO operations for pattern and exception data provided by `hyph-UTF8`
//! and stored in the `patterns` folder.

use std::error;
use std::fmt;
use std::io;

use bincode::serde as bin;

use hyphenation_commons::{Exceptions, Patterns};
use language::{Corpus, Language, mins, tag};
use resources::ResourceId;


pub fn retrieve_resource(lang: Language, suffix: &str) -> Result<&[u8], Error> {
    let fname = format!("{}.{}.bincode", tag(lang), suffix);
    let res: Option<ResourceId> = ResourceId::from_name(&fname);

    match res {
        Some(data) => Ok(data.load()),
        None => Err(Error::Resource)
    }
}

pub fn patterns(lang: Language) -> Result<Patterns, Error> {
    let f = try!(retrieve_resource(lang, "patterns"));
    let trie: Patterns = try!(bin::deserialize(f));

    Ok(trie)
}

pub fn exceptions(lang: Language) -> Result<Exceptions, Error> {
    let f = try!(retrieve_resource(lang, "exceptions"));
    let trie: Exceptions = try!(bin::deserialize(f));

    Ok(trie)
}

/// Constructs the default `Corpus` for a given language.
pub fn language(lang: Language) -> Result<Corpus, Error> {
    let (l, r) = mins(lang);
    let ps = try!(patterns(lang));
    let exs = try!(exceptions(lang));

    Ok(Corpus {
        language: lang,
        patterns: ps,
        exceptions: exs,
        left_min: l,
        right_min: r
    })
}


/// Failure modes of pattern loading.
#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Deserialization(bin::DeserializeError),
    Resource
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IO(ref e) => e.description(),
            Error::Deserialization(ref e) => e.description(),
            Error::Resource => "Pattern resource failed to load"
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IO(ref e) => e.fmt(f),
            Error::Deserialization(ref e) => e.fmt(f),
            Error::Resource => {
                let e = self as &error::Error;
                e.description().fmt(f)
            }
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<bin::DeserializeError> for Error {
    fn from(err: bin::DeserializeError) -> Error {
        Error::Deserialization(err)
    }
}
