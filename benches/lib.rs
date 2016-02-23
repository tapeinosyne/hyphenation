#![feature(test)]

#[macro_use]
extern crate lazy_static;
extern crate test;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use test::Bencher;

extern crate hyphenation;
use hyphenation::{load, Corpus, Hyphenation, Language};
use hyphenation::exception::{Exceptions};
use hyphenation::pattern::{Patterns};


fn fiat_io(lang: Language) -> Corpus {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("patterns");

    hyphenation::set_pattern_folder(path.as_path());
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

    let by_line = load::patterns(Language::English_US).unwrap();
    let v: Vec<_> = by_line.collect();

    let mut ps = Patterns::empty();
    b.iter(|| {
        for p in &v {
            for val in p { ps.insert(&*val) };
        }
    });
}

#[bench]
fn parse_exceptions_en_us(b: &mut Bencher) {
    hyphenation::set_pattern_folder(DATAPATH.as_path());

    let by_line = load::exceptions(Language::English_US).unwrap();
    let v: Vec<_> = by_line.collect();

    let mut exs = Exceptions::empty();
    b.iter(|| {
        for ex in &v {
            for val in ex { exs.insert(&*val) };
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
