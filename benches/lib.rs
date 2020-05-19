#![feature(test)]

#[macro_use] extern crate lazy_static;
extern crate bincode;
extern crate test;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path};
use test::{black_box, Bencher};

extern crate hyphenation;
use hyphenation::*;
use hyphenation::extended::*;
use hyphenation::Language::*;


fn fiat_std(lang : Language) -> Standard {
    let filename = format!("{}.standard.bincode", lang.code());
    let file = File::open(Path::new("dictionaries").join(filename)).unwrap();
    Standard::from_reader(lang, &mut BufReader::new(file)).unwrap()
}

fn fiat_ext(lang : Language) -> Extended {
    let filename = format!("{}.extended.bincode", lang.code());
    let file = File::open(Path::new("dictionaries").join(filename)).unwrap();
    Extended::from_reader(lang, &mut BufReader::new(file)).unwrap()
}


lazy_static! {
    static ref EN_US : Standard = fiat_std(EnglishUS);
    static ref HU_EXT : Extended = fiat_ext(Hungarian);
    static ref HU_STD : Standard = fiat_std(Hungarian);
    static ref TR : Standard = fiat_std(Turkish);

    static ref WORDS : Vec<String> = {
        let file = File::open(Path::new("/usr/share/dict/words")).unwrap();
        let words : Vec<_> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

        words
    };
}


#[bench]
fn word_opportunities_en_us(b : &mut Bencher) {
    lazy_static::initialize(&EN_US);
    b.iter(|| {
        for w in WORDS.iter() {
            EN_US.opportunities(w);
        }
    })
}

#[bench]
fn word_hyphenate_en_us(b : &mut Bencher) {
    lazy_static::initialize(&EN_US);
    b.iter(|| {
        for w in WORDS.iter() {
            EN_US.hyphenate(w);
        }
    })
}

#[bench]
fn standard_opportunities_hu(b : &mut Bencher) {
    lazy_static::initialize(&HU_STD);
    b.iter(|| HU_STD.opportunities("asszonnyal"))
}

#[bench]
fn extended_opportunities_hu(b : &mut Bencher) {
    lazy_static::initialize(&HU_EXT);
    b.iter(|| HU_EXT.opportunities("asszonnyal"))
}

#[bench]
fn standard_segments_hu(b : &mut Bencher) {
    lazy_static::initialize(&HU_STD);
    let w = "asszonnyal";
    b.iter(|| for s in HU_STD.hyphenate(w).into_iter().segments() { black_box(s); })
}

#[bench]
fn extended_segments_hu(b : &mut Bencher) {
    lazy_static::initialize(&HU_EXT);
    let w = "asszonnyal";
    b.iter(|| for s in HU_EXT.hyphenate(w).into_iter().segments() { black_box(s); })
}

#[bench]
fn special_casing_ignored(b : &mut Bencher) {
    lazy_static::initialize(&TR);
    b.iter(|| TR.opportunities("İLGİNÇ"))
}

#[bench]
fn special_casing_handled(b : &mut Bencher) {
    lazy_static::initialize(&TR);
    b.iter(|| TR.hyphenate("İLGİNÇ").breaks)
}



#[cfg(feature = "embed_all")] #[bench]
fn deserialize_patterns_en_us(b : &mut Bencher) {
    b.iter(|| EnglishUS.from_embed_allded(Standard).unwrap())
}
