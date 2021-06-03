/*! # Reading and loading hyphenation dictionaries

To hyphenate words in a given language, it is first necessary to load
the relevant hyphenation dictionary into memory. This module offers
convenience methods for common retrieval patterns, courtesy of the
[`Load`] trait.

```
use hyphenation::Load;
use hyphenation::{Standard, Language};
```

The primary function of [`Load`] is to deserialize dictionaries from
buffers – usually, file buffers.

```ignore
use std::io;
use std::fs::File;

let path_to_dict = "/path/to/english-dictionary.bincode";
let dict_file = File::open(path_to_dict) ?;
let mut reader = io::BufReader::new(dict_file);
let english_us = Standard::from_reader(Language::EnglishUS, &mut reader) ?;
```

Dictionaries can be loaded from the file system rather more succintly with
the [`from_path`] shorthand:

```ignore
let path_to_dict = "/path/to/english-dictionary.bincode";
let english_us = Standard::from_path(Language::EnglishUS, path_to_dict) ?;
```

Dictionaries bundled with the `hyphenation` library are copied to Cargo's
output directory at build time. To locate them, look for a `dictionaries`
folder under `target`:

```text
$ find target -name "dictionaries"
target/debug/build/hyphenation-33034db3e3b5f3ce/out/dictionaries
```


## Embedding

Optionally, hyphenation dictionaries can be embedded in the compiled
artifact by enabling the `embed_all` feature. Embedded dictionaries can be
accessed directly from memory.

```ignore
use hyphenation::{Standard, Language, Load};

let english_us = Standard::from_embedded(Language::EnglishUS) ?;
```

Note that embedding significantly increases the size of the compiled artifact.


[`Load`]: trait.Load.html
[`from_path`]: trait.Load.html#method.from_path
*/

use bincode as bin;
#[cfg(any(feature = "embed_all", feature = "embed_en-us"))]
use crate::resources::ResourceId;
use std::error;
use std::fmt;
use std::fs::File;
use std::io;
use std::path::Path;
use std::result;

use hyphenation_commons::dictionary::{extended::Extended, Standard};
use hyphenation_commons::Language;


/// Convenience methods for the retrieval of hyphenation dictionaries.
pub trait Load: Sized {
    /// Read and deserialize the dictionary at the given path, verifying that it
    /// belongs to the expected language.
    fn from_path<P>(lang : Language, path : P) -> Result<Self>
        where P : AsRef<Path>
    {
        let file = File::open(path)?;
        Self::from_reader(lang, &mut io::BufReader::new(file))
    }

    /// Deserialize a dictionary from the provided reader, verifying that it
    /// belongs to the expected language.
    fn from_reader<R>(lang : Language, reader : &mut R) -> Result<Self>
        where R : io::Read;

    /// Deserialize a dictionary from the provided reader.
    fn any_from_reader<R>(reader : &mut R) -> Result<Self>
        where R : io::Read;

    #[cfg(any(feature = "embed_all", feature = "embed_en-us"))]
    /// Deserialize the embedded dictionary for the given language.
    fn from_embedded(lang : Language) -> Result<Self>;
}

macro_rules! impl_load {
    ($dict:ty, $suffix:expr) => {
        impl Load for $dict {
            fn from_reader<R>(lang : Language, reader : &mut R) -> Result<Self>
                where R : io::Read
            {
                let dict : Self = bin::deserialize_from(reader)?;
                let (found, expected) = (dict.language(), lang);
                if found != expected {
                    Err(Error::LanguageMismatch { expected, found })
                } else {
                    Ok(dict)
                }
            }

            fn any_from_reader<R>(reader : &mut R) -> Result<Self>
                where R : io::Read
            {
                let dict : Self = bin::deserialize_from(reader)?;
                Ok(dict)
            }

            #[cfg(any(feature = "embed_all", feature = "embed_en-us"))]
            fn from_embedded(lang : Language) -> Result<Self> {
                let dict_bytes = retrieve_resource(lang.code(), $suffix)?;
                let dict = bin::deserialize(dict_bytes)?;
                Ok(dict)
            }
        }
    };
}

impl_load! { Standard, "standard" }
impl_load! { Extended, "extended" }


#[cfg(any(feature = "embed_all", feature = "embed_en-us"))]
fn retrieve_resource<'a>(code : &str, suffix : &str) -> Result<&'a [u8]> {
    let name = format!("{}.{}.bincode", code, suffix);
    let res : Option<ResourceId> = ResourceId::from_name(&name);
    match res {
        Some(data) => Ok(data.load()),
        None => Err(Error::Resource),
    }
}


pub type Result<T> = result::Result<T, Error>;

/// Failure modes of dictionary loading.
#[derive(Debug)]
pub enum Error {
    /// The dictionary could not be deserialized.
    Deserialization(bin::Error),
    /// The dictionary could not be read.
    IO(io::Error),
    /// The loaded dictionary is for the wrong language.
    LanguageMismatch {
        expected : Language,
        found :    Language,
    },
    /// The embedded dictionary could not be retrieved.
    Resource,
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Deserialization(ref e) => Some(e),
            Error::IO(ref e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Deserialization(ref e) => e.fmt(f),
            Error::IO(ref e) => e.fmt(f),
            Error::LanguageMismatch { expected, found } => write!(
                                                                  f,
                                                                  "\
Language mismatch: attempted to load a dictionary for `{}`, but found
a dictionary for `{}` instead.",
                                                                  expected, found
            ),
            Error::Resource => f.write_str("the embedded dictionary could not be retrieved"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err : io::Error) -> Error { Error::IO(err) }
}

impl From<bin::Error> for Error {
    fn from(err : bin::Error) -> Error { Error::Deserialization(err) }
}
