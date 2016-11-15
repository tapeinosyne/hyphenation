#[macro_use]
extern crate lazy_static;
extern crate quickcheck;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path};
use quickcheck::{quickcheck, TestResult};

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
fn punctuated_count() {
    fn property(s: String) -> bool {
        let l = s.chars().count();
        let os = s.opportunities(&EN_US);
        let h: String = s.hyphenate(&EN_US).punctuate().collect();

        h.chars().count() == l + os.len()
    }

    quickcheck(property as fn(String) -> bool);
}

#[test]
fn hyphenation_bounds() {
    fn property(s: String) -> TestResult {
        let ci: Vec<_> = s.char_indices().collect();
        let (l_min, r_min) = (&EN_US.left_min, &EN_US.right_min);
        let s_len = ci.len();
        if s_len < l_min + r_min {
            return TestResult::discard();
        }

        let os = s.opportunities(&EN_US);
        let ((l, _), (r, _)) = (ci[l_min - 1], ci[s_len - r_min]);
        let within_bounds = |&i| i > l && i <= r;

        TestResult::from_bool(os.iter().all(within_bounds))
    }

    quickcheck(property as fn(String) -> TestResult);
}

#[test]
fn basics() {
    let h1: Standard = "hyphenation".hyphenate(&EN_US);
    let h2: Standard = "project".hyphenate(&EN_US);
    let h3: Standard = "hypha".hyphenate(&EN_US);
    let h4: Standard = "Word hyphenation by computer.".fulltext_hyphenate(&EN_US);

    assert_eq!(h1.size_hint(), (3, Some(3)));
    assert_eq!(h2.size_hint(), (1, Some(1)));
    assert_eq!(h3.size_hint(), (2, Some(2)));
    assert_eq!(h4.size_hint(), (4, Some(4)));

    let v1: Vec<&str> = h1.clone().collect();
    let v2: Vec<&str> = h2.clone().collect();
    let v3: Vec<&str> = h3.clone().collect();
    let v4: Vec<&str> = h4.clone().collect();
    assert_eq!(v1, vec!["hy", "phen", "ation"]);
    assert_eq!(v2, vec!["project"]);
    assert_eq!(v3, vec!["hy", "pha"]);
    assert_eq!(v4, vec!["Word hy", "phen", "ation by com", "puter."]);

    let ex1: Standard = "retribution".hyphenate(&EN_US);
    let v_ex1: Vec<&str> = ex1.clone().collect();
    assert_eq!(v_ex1, vec!["ret", "ri", "bu", "tion"]);

    let s1: String = h1.punctuate().collect();
    assert_eq!(s1, "hy\u{ad}phen\u{ad}ation".to_owned());

    // And some further size_hint sanity checking for partially consumed iterators.
    let mut h2 = h2;
    h2.next();
    assert_eq!(h2.size_hint(), (0, Some(0)));
    h2.next();
    assert_eq!(h2.size_hint(), (0, Some(0)));

    let mut h3 = h3;
    h3.next();
    assert_eq!(h3.size_hint(), (1, Some(1)));
    h3.next();
    assert_eq!(h3.size_hint(), (0, Some(0)));
}

#[test]
#[should_panic(expected = "assertion failed")]
fn known_inaccuracies() {
    let example1: Vec<&str> = "chionididae".hyphenate(&EN_US).collect();

    assert!(example1 == vec!["chi", "o", "nid", "i", "dae"]);
}
