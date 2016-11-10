#![feature(test)]

#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate test;

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path};
use test::Bencher;

use serde_json::{self as json};

extern crate hyphenation;
use hyphenation::{load, Corpus, Exceptions, Patterns, Hyphenation, FullTextHyphenation, Language};


fn fiat_io(lang: Language) -> Corpus { load::language(lang).unwrap() }

lazy_static! {
    static ref EN_US: Corpus = fiat_io(Language::English_US);

    static ref WORDS: Vec<String> = {
        let file = File::open(Path::new("/usr/share/dict/words")).unwrap();
        let words: Vec<_> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

        words
    };
}


#[bench]
fn parse_patterns_en_us(b: &mut Bencher) {
    let mut f = load::data_file(Language::English_US, "pat").unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer);

    let mut ps = Patterns::new();
    b.iter(|| {
        let pairs: Vec<(String, Vec<u8>)> = json::from_slice(&buffer).unwrap();
        for p in pairs {
            ps.insert(p);
        }
    });
}

#[bench]
fn parse_exceptions_en_us(b: &mut Bencher) {
    let mut f = load::data_file(Language::English_US, "hyp").unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer);

    let mut exs = Exceptions::new();
    b.iter(|| {
        let pairs: Vec<(String, Vec<u8>)> = json::from_slice(&buffer).unwrap();
        for ex in pairs {
            exs.insert(ex);
        }
    });
}

#[bench]
fn opportunities_en_us(b: &mut Bencher) {
    let mut ws = WORDS.iter();

    b.iter(|| {
        for w in ws.by_ref() {
            w.opportunities(&EN_US);
        }
    })
}

#[bench]
fn hyphenate_en_us(b: &mut Bencher) {
    let mut ws = WORDS.iter();

    b.iter(|| {
        for w in ws.by_ref() {
            w.hyphenate(&EN_US).count();
        }
    })
}

#[bench]
fn fulltext_opportunities_en_us(b: &mut Bencher) {
    let mut ws = WORDS.iter();

    b.iter(|| {
        for w in ws.by_ref() {
            w.fulltext_opportunities(&EN_US);
        }
    })
}

#[bench]
fn fulltext_hyphenate_en_us(b: &mut Bencher) {
    let mut ws = WORDS.iter();

    b.iter(|| {
        for w in ws.by_ref() {
            w.fulltext_hyphenate(&EN_US).count();
        }
    })
}
