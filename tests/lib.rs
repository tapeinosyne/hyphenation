#[macro_use]
extern crate lazy_static;
extern crate quickcheck;

use std::io;
use quickcheck::{quickcheck};

extern crate hyphenation;
use hyphenation::{load, Language, Corpus, Hyphenation, Standard};


fn fiat_io(lang: Language) -> Corpus { load::language(lang).unwrap() }

lazy_static! {
    static ref EN_US: Corpus = fiat_io(Language::English_US);
}


#[test]
fn collected_equals_original() {
    fn property(original: String) -> bool {
        let collected: String = original.hyphenate(&EN_US).collect();

        collected == original
    }

    quickcheck(property as fn(String) -> bool);
}

#[test]
fn opportunities_within_bounds() {
    fn property(s: String) -> bool {
        let os = s.opportunities(&EN_US);
        let l = s.len();

        os.iter().all(|&i| i < l)
    }

    quickcheck(property as fn(String) -> bool);
}

#[test]
fn basics() {
    let h1: Standard = "hyphenation".hyphenate(&EN_US);
    let h2: Standard = "project".hyphenate(&EN_US);

    let v1: Vec<&str> = h1.clone().collect();
    let v2: Vec<&str> = h2.clone().collect();
    assert_eq!(v1, vec!["hy", "phen", "ation"]);
    assert_eq!(v2, vec!["project"]);

    let s1: String = h1.punctuate().collect();
    assert_eq!(s1, "hy\u{ad}phen\u{ad}ation".to_owned());
}
