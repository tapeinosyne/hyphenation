#![feature(test)]

#[macro_use]
extern crate lazy_static;
extern crate test;

use std::path::PathBuf;
use test::Bencher;

extern crate hyphenation;
use hyphenation::{load, Language};
use hyphenation::exception::{Exceptions};
use hyphenation::pattern::{Patterns};


lazy_static! {
    static ref DATAPATH: PathBuf = {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("patterns");

        path
    };
}


#[bench]
fn parse_patterns_en_us(bench: &mut Bencher) {
    hyphenation::set_pattern_folder(DATAPATH.as_path());

    let by_line = load::patterns(Language::English_US).unwrap();
    let v: Vec<_> = by_line.collect();

    let mut ps = Patterns::empty();
    bench.iter(|| {
        for p in &v {
            for val in p { ps.insert(&*val) };
        }
    });
}

#[bench]
fn parse_exceptions_en_us(bench: &mut Bencher) {
    hyphenation::set_pattern_folder(DATAPATH.as_path());

    let by_line = load::exceptions(Language::English_US).unwrap();
    let v: Vec<_> = by_line.collect();

    let mut exs = Exceptions::empty();
    bench.iter(|| {
        for ex in &v {
            for val in ex { exs.insert(&*val) };
        }
    });
}
