#![feature(test)]

#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate test;

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use test::Bencher;

use serde_json::{self as json};

extern crate hyphenation;
use hyphenation::{load, Corpus, Hyphenation, Language};
use hyphenation::exception::{Exceptions};
use hyphenation::pattern::{Patterns};


fn fiat_io(lang: Language) -> Corpus {
    hyphenation::set_pattern_folder(&DATAPATH.as_path());
    load::language(lang).unwrap()
}

lazy_static! {
    static ref DATAPATH: PathBuf = {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("patterns");

        path
    };

    static ref EN_US: Corpus = fiat_io(Language::English_US);

    static ref WORDS: Vec<String> = {
        let file = File::open(Path::new("/usr/share/dict/words")).unwrap();
        let words: Vec<_> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

        words
    };
}


#[bench]
fn parse_patterns_en_us(b: &mut Bencher) {
    hyphenation::set_pattern_folder(DATAPATH.as_path());
    let mut f = load::data_file(Language::English_US, "pat").unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer);

    let mut ps = Patterns::empty();
    b.iter(|| {
        let pairs: Vec<(String, Vec<u32>)> = json::from_slice(&buffer).unwrap();
        for p in pairs {
            ps.insert(p);
        }
    });
}

#[bench]
fn parse_exceptions_en_us(b: &mut Bencher) {
    hyphenation::set_pattern_folder(DATAPATH.as_path());
    let mut f = load::data_file(Language::English_US, "hyp").unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer);

    let mut exs = Exceptions::empty();
    b.iter(|| {
        let pairs: Vec<(String, Vec<u32>)> = json::from_slice(&buffer).unwrap();
        for ex in pairs {
            exs.insert(ex);
        }
    });
}

#[bench]
fn opportunities_words(b: &mut Bencher) {
    hyphenation::set_pattern_folder(DATAPATH.as_path());
    let mut ws = WORDS.iter();

    b.iter(|| {
        for w in ws.by_ref() {
            w.opportunities(&EN_US);
        }
    })
}

#[bench]
fn hyphenate_words(b: &mut Bencher) {
    hyphenation::set_pattern_folder(DATAPATH.as_path());
    let mut ws = WORDS.iter();

    b.iter(|| {
        for w in ws.by_ref() {
            w.hyphenate(&EN_US).count();
        }
    })
}
