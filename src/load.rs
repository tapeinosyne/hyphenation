//! IO operations for pattern and exception data provided by `hyph-UTF8`
//! and stored in the `patterns` folder.

use std::error;
use std::fmt;
use std::io;

use serde_json::{self as json};

use klpattern::{KLPair, Exceptions, Patterns};
use language::{Corpus, Language, mins, tag};
use resources::ResourceId;


pub fn data_file(lang: Language, suffix: &str) -> Result<&[u8], Error> {
    let fname = format!("hyph-{}.{}.json", tag(lang), suffix);
    let res: Option<ResourceId> = ResourceId::from_name(&fname);

    match res {
        Some(data) => Ok(data.load()),
        None => Err(Error::Resource)
    }
}

pub fn patterns(lang: Language) -> Result<Vec<KLPair>, Error> {
    let f = try!(data_file(lang, "pat"));
    let pairs: Vec<(String, Vec<u8>)> = try!(json::from_slice(f));

    Ok(pairs)
}

pub fn exceptions(lang: Language) -> Result<Vec<KLPair>, Error> {
    let f = try!(data_file(lang, "hyp"));
    let pairs: Vec<(String, Vec<u8>)> = try!(json::from_slice(f));

    Ok(pairs)
}

/// Constructs the default `Corpus` for a given language.
pub fn language(lang: Language) -> Result<Corpus, Error> {
    let (l, r) = mins(lang);
    let pat_pairs = try!(patterns(lang));
    let ex_pairs = try!(exceptions(lang));

    let mut ps = Patterns::new();
    for p in pat_pairs {
        ps.insert(p);
    }

    let mut exs = Exceptions::new();
    for ex in ex_pairs {
        exs.insert(ex);
    }

    Ok(Corpus {
        language: lang,
        patterns: ps,
        exceptions: if !exs.0.is_empty() { Some(exs) } else { None },
        left_min: l,
        right_min: r
    })
}


/// Failure modes of pattern loading.
#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Deserialization(json::Error),
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

impl From<json::Error> for Error {
    fn from(err: json::Error) -> Error {
        Error::Deserialization(err)
    }
}
