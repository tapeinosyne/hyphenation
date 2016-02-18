//! IO operations for pattern and exception data provided by `hyph-UTF8`
//! and stored in the `patterns` folder.

use std::fs::File;
use std::io::{self as io, BufRead, BufReader, Lines};
use std::path::{Path, PathBuf};
use std::sync::{RwLock};

use language::{Corpus, Language, mins, tag};
use exception::{Exceptions};
use pattern::{Patterns};

lazy_static! {
    static ref PATTERN_FOLDER: RwLock<PathBuf> = RwLock::new(PathBuf::new());
}

pub fn set_pattern_folder(path: &Path) {
    let mut folder = PATTERN_FOLDER.write().unwrap();

    folder.push(path);
}

pub fn data_file(lang: Language, suffix: &str) -> io::Result<File> {
    let fname = format!("hyph-{}.{}.txt", tag(lang), suffix);
    let as_set = PATTERN_FOLDER.read().unwrap();
    let mut fpath = PathBuf::new();
    fpath.push(&*as_set);
    fpath.push(fname);

    File::open(fpath)
}

pub fn patterns(lang: Language) -> io::Result<Lines<BufReader<File>>> {
    let f = try!(data_file(lang, "pat"));
    let reader = BufReader::new(f);

    Ok(reader.lines())
}

pub fn exceptions(lang: Language) -> io::Result<Lines<BufReader<File>>> {
    let f = try!(data_file(lang, "hyp"));
    let reader = BufReader::new(f);

    Ok(reader.lines())
}


/// Constructs the default `Corpus` for a given language.
pub fn language(lang: Language) -> io::Result<Corpus> {
    let (l, r) = mins(lang);
    let pat_by_line = try!(patterns(lang));
    let ex_by_line = try!(exceptions(lang));

    let mut ps = Patterns::empty();
    for p in pat_by_line {
        for val in p { ps.insert(&*val) };
    }

    let mut exs = Exceptions::empty();
    for ex in ex_by_line {
        for val in ex { exs.insert(&*val) }
    }
    let exs = if !exs.0.is_empty() { Some(exs) } else { None };

    Ok(Corpus {
        language: lang,
        patterns: ps,
        exceptions: exs,
        left_min: l,
        right_min: r
    })
}
