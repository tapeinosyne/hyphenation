#![feature(test)]

#[macro_use]
extern crate lazy_static;
extern crate bincode;
extern crate test;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path};
use test::Bencher;

use bincode::serde as bin;

extern crate hyphenation;
use hyphenation::*;


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


#[bench]
fn deserialize_patterns_en_us(b: &mut Bencher) {
    let slice = load::data_file(Language::English_US, "patterns").unwrap();

    b.iter(|| {
        test::black_box(bin::deserialize::<Patterns>(slice).unwrap());
    });
}
