extern crate once_cell;
extern crate quickcheck;
extern crate unicode_segmentation;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use once_cell::sync::Lazy;
use quickcheck::{quickcheck, TestResult};

extern crate hyphenation;
extern crate hyphenation_commons;
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

static EN_US : Lazy<Standard> = Lazy::new(|| fiat_std(EnglishUS));
static HU : Lazy<Extended> = Lazy::new(|| fiat_ext(Hungarian));
static TR : Lazy<Standard> = Lazy::new(|| fiat_std(Turkish));


#[test]
fn collected_equals_original() {
    fn property(original : String) -> bool {
        let collected : String = EN_US.hyphenate(&original).iter().segments().collect();

        collected == original
    }

    quickcheck(property as fn(String) -> bool);
}

#[test]
fn opportunities_within_bounds() {
    fn property(s : String) -> TestResult {
        let ci : Vec<_> = s.char_indices().collect();
        let (l_min, r_min) = EnglishUS.minima();
        let s_len = ci.len();
        if s_len < l_min + r_min { return TestResult::discard() }

        let os : Vec<_> = EN_US.opportunities(&s);
        let ((l, _), (r, _)) = (ci[l_min], ci[s_len - r_min]);
        let within_bounds = |&i| i >= l && i <= r;

        TestResult::from_bool(os.iter().all(within_bounds))
    }

    quickcheck(property as fn(String) -> TestResult);
}


#[test]
fn basics_standard() {
    // Standard hyphenation
    let w0 = "anfractuous";
    let w1 = "hypha";        // minimum hyphenable length
    // Exceptions
    let ex0 = "hyphenation";
    let ex1 = "bevies";     // unhyphenable (by exception)

    let h_w0 = EN_US.hyphenate(w0);
    let h_w1 = EN_US.hyphenate(w1);
    let h_ex0 = EN_US.hyphenate(ex0);
    let h_ex1 = EN_US.hyphenate(ex1);

    let seg0 = h_w0.iter().segments();
    let seg1 = h_ex0.iter().segments();
    let seg2 = h_ex1.iter().segments();
    let seg3 = h_w1.iter().segments();

    assert_eq!(seg0.size_hint(), (4, Some(4)));
    assert_eq!(seg1.size_hint(), (4, Some(4)));
    assert_eq!(seg2.size_hint(), (1, Some(1)));
    assert_eq!(seg3.size_hint(), (2, Some(2)));

    let v0 : Vec<&str> = seg0.clone().collect();
    let v1 : Vec<&str> = seg1.clone().collect();
    let v2 : Vec<&str> = seg2.clone().collect();
    let v3 : Vec<&str> = seg3.clone().collect();

    assert_eq!(v0, vec!["an", "frac", "tu", "ous"]);
    assert_eq!(v1, vec!["hy", "phen", "a", "tion"]);
    assert_eq!(v2, vec!["bevies"]);
    assert_eq!(v3, vec!["hy", "pha"]);

    // Additional size checks for partially consumed iterators.
    let mut seg2 = seg2;
    seg2.next();
    assert_eq!(seg2.size_hint(), (0, Some(0)));
    seg2.next();
    assert_eq!(seg2.size_hint(), (0, Some(0)));

    let mut seg3 = seg3;
    seg3.next();
    assert_eq!(seg3.size_hint(), (1, Some(1)));
    seg3.next();
    assert_eq!(seg3.size_hint(), (0, Some(0)));
}

#[test]
fn basics_extended() {
    let w0 = "asszonnyal";
    let w1 = "esszé";

    let v0 : Vec<_> = HU.hyphenate(w0).into_iter().segments().collect();
    let v1 : Vec<_> = HU.hyphenate(w1).into_iter().segments().collect();

    assert_eq!(v0, vec!["asz", "szony", "nyal"]);
    assert_eq!(v1, vec!["esz", "szé"]);
}

#[test]
fn special_casing() {
    let w0 = "İbrahim";
    let v0 : Vec<_> = TR.hyphenate(&w0).into_iter().segments().collect();
    assert_eq!(v0, vec!["İb", "ra", "him"]);

    let w1 = "İLGİNÇ";
    let v1 : Vec<_> = TR.hyphenate(w1).into_iter().segments().collect();
    assert_eq!(v1, vec!["İL", "GİNÇ"]);

    let w2 = "MİCRO";
    let v2 : Vec<_> = EN_US.hyphenate(w2).into_iter().segments().collect();
    assert_eq!(v2, vec!["Mİ", "CRO"]);

    let w4 = "İDİOM";
    let v4 : Vec<_> = EN_US.hyphenate(w4).into_iter().segments().collect();
    assert_eq!(v4, vec!["İD", "İOM"]);

    let w3 = "MUCİLAGİNOUS";
    let v3 : Vec<_> = EN_US.hyphenate(w3).into_iter().segments().collect();
    assert_eq!(v3, vec!["MU", "CİLAGİ", "NOUS"]);
}

#[test]
fn language_mismatch_on_load() {
    let file = File::open("./dictionaries/mul-ethi.standard.bincode").unwrap();
    let mut reader = BufReader::new(file);
    assert!(Standard::from_reader(EnglishUS, &mut reader).is_err());
}

#[test]
fn text() {
    use unicode_segmentation::UnicodeSegmentation;

    let hyphenate_text = |text : &str | -> String {
        text.split_word_bounds()
            .flat_map(|word| EN_US.hyphenate(word).into_iter())
            .collect()
    };

    let t0 = "I know noble accents / And lucid, inescapable rhythms; […]";
    let expect0 = "I know no-ble ac-cents / And lu-cid, in-escapable rhythms; […]";
    let seg0 = hyphenate_text(t0);
    assert_eq!(seg0, expect0);

    let t1 = "ever-burning sulphur unconsumed";
    let expect1 = "ever-burn-ing sul-phur un-con-sumed";
    let seg1 = hyphenate_text(t1);
    assert_eq!(seg1, expect1);

}

#[test]
fn bounded_exception() {
    let e = "anisotropic";  // an-iso-trop-ic, by exception

    let bounded = EN_US.exception(e);
    let unbounded = EN_US.exception_within(e, (0, e.len()));

    assert_eq!(bounded, Some(vec![2, 5]));
    assert_eq!(unbounded, Some(vec![2, 5, 9]));
}

#[test]
fn readme_examples() {
    let hyphenated = EN_US.hyphenate("hyphenation");

    let break_indices = &hyphenated.breaks;
    assert_eq!(break_indices, &[2, 6, 7]);

    let marked = hyphenated.iter();
    let collected : Vec<String> = marked.collect();
    assert_eq!(collected, vec!["hy-", "phen-", "a-", "tion"]);

    let unmarked = hyphenated.iter().segments();
    let collected : Vec<&str> = unmarked.collect();
    assert_eq!(collected, vec!["hy", "phen", "a", "tion"]);

    let uppercase : Vec<_> = EN_US.hyphenate("CAPITAL").into_iter().segments().collect();
    assert_eq!(uppercase, vec!["CAP", "I", "TAL"]);
}
