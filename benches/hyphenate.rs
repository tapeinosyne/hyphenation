extern crate once_cell;
extern crate bincode;
extern crate criterion;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use once_cell::sync::Lazy;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
// use test::{black_box, Bencher};

extern crate hyphenation;
use hyphenation::extended::*;
use hyphenation::Language::*;
use hyphenation::*;


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

static EN_US : Lazy<Standard> = Lazy::new(|| fiat_std(EnglishUS));
static GRC : Lazy<Standard> = Lazy::new(|| fiat_std(GreekAncient));
static HU_EXT : Lazy<Extended> = Lazy::new(|| fiat_ext(Hungarian));
static HU_STD : Lazy<Standard> = Lazy::new(|| fiat_std(Hungarian));
static TR : Lazy<Standard> = Lazy::new(|| fiat_std(Turkish));
static WORDS : Lazy<Vec<String>> = Lazy::new(|| {
    let file = File::open(Path::new("/usr/share/dict/words")).unwrap();
    let octavate = BufReader::new(file).lines().map(|l| l.unwrap()).step_by(8);
    octavate.collect()
});


const OVERLONG_EN_US : &'static str =
 "Lopadotemachoselachogaleokranioleipsanodrimhypotrimmatosilphiokarabomelitokatakechymenokichlepik\
  ossyphophattoperisteralektryonoptekephalliokigklopeleiolagoiosiraiobaphetraganopterygon";

const OVERLONG_GRC : &'static str =
 "λοπαδοτεμαχοσελαχογαλεοκρανιολειψανοδριμυποτριμματοσιλφιοκαραβομελιτοκατακεχυμενοκιχλεπικοσσυφοφ\
  αττοπεριστεραλεκτρυονοπτοκεφαλλιοκιγκλοπελειολαγῳοσιραιοβαφητραγανοπτερύγων";


fn dictionary_opportunities_en_us(c : &mut Criterion) {
    Lazy::force(&EN_US);
    c.bench_function("dictionary, en-US", |b| {
         b.iter(|| {
              for w in WORDS.iter() {
                  EN_US.opportunities(w);
              }
          })
     });
}

fn opportunities_en_us(c : &mut Criterion) {
    Lazy::force(&EN_US);
    let w = "antidisestablishmentarianism";
    c.bench_function("opportunities, en-US", |b| {
         b.iter(|| EN_US.opportunities(black_box(w)))
     });
}

fn hyphenate_en_us(c : &mut Criterion) {
    Lazy::force(&EN_US);
    let w = "antidisestablishmentarianism";
    c.bench_function("hyphenate, en-US", |b| {
         b.iter(|| EN_US.hyphenate(black_box(w)))
     });
}

fn opportunities_hu_standard(c : &mut Criterion) {
    Lazy::force(&HU_STD);
    let w = "asszonnyal";
    c.bench_function("opportunities, HU std", |b| {
         b.iter(|| HU_STD.opportunities(black_box(w)))
     });
}

fn opportunities_hu_extended(c : &mut Criterion) {
    Lazy::force(&HU_EXT);
    let w = "asszonnyal";
    c.bench_function("opportunities, HU ext", |b| {
         b.iter(|| HU_EXT.opportunities(black_box(w)))
     });
}

fn segments_hu_standard(c : &mut Criterion) {
    Lazy::force(&HU_STD);
    let w = "asszonnyal";
    c.bench_function("segments, HU std", |b| {
         b.iter(|| {
              for s in HU_STD.hyphenate(black_box(w)).into_iter().segments() {
                  black_box(s);
              }
          })
     });
}

fn segments_hu_extended(c : &mut Criterion) {
    Lazy::force(&HU_EXT);
    let w = "asszonnyal";
    c.bench_function("segments, HU ext", |b| {
         b.iter(|| {
              for s in HU_STD.hyphenate(black_box(w)).into_iter().segments() {
                  black_box(s);
              }
          })
     });
}

fn special_casing_ignored(c : &mut Criterion) {
    Lazy::force(&TR);
    let w = "İLGİNÇ";
    c.bench_function("special casing, ignored", |b| {
         b.iter(|| TR.opportunities(black_box(w)))
     });
}

fn special_casing_handled(c : &mut Criterion) {
    Lazy::force(&TR);
    let w = "İLGİNÇ";
    c.bench_function("special casing, handled", |b| {
         b.iter(|| TR.hyphenate(black_box(w)).breaks)
     });
}

fn opportunities_en_us_overlong(c : &mut Criterion) {
    Lazy::force(&EN_US);
    c.bench_function("overlong, en-US", |b| {
         b.iter(|| EN_US.opportunities(black_box(OVERLONG_EN_US)))
     });
}

fn opportunities_grc_overlong(c : &mut Criterion) {
    Lazy::force(&EN_US);
    c.bench_function("overlong, GRC", |b| {
         b.iter(|| GRC.opportunities(black_box(OVERLONG_GRC)))
     });
}


criterion_group!(single_word,
                 hyphenate_en_us,
                 opportunities_en_us,
                 opportunities_en_us_overlong,
                 opportunities_grc_overlong,
                 opportunities_hu_extended,
                 opportunities_hu_standard,
                 segments_hu_extended,
                 segments_hu_standard,
                 special_casing_handled,
                 special_casing_ignored);

criterion_group! {
    name = many_words;
    config = Criterion::default().sample_size(50);
    targets = dictionary_opportunities_en_us
}

criterion_main!(single_word, many_words);
