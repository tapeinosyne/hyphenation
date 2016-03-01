#[macro_use]
extern crate lazy_static;
extern crate quickcheck;

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use quickcheck::{quickcheck, TestResult};

extern crate hyphenation;
use hyphenation::{load, Language, Corpus, Hyphenation, Standard};


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

    let v1: Vec<&str> = h1.clone().collect();
    let v2: Vec<&str> = h2.clone().collect();
    let v3: Vec<&str> = h3.clone().collect();
    assert_eq!(v1, vec!["hy", "phen", "ation"]);
    assert_eq!(v2, vec!["project"]);
    assert_eq!(v3, vec!["hy", "pha"]);

    let s1: String = h1.punctuate().collect();
    assert_eq!(s1, "hy\u{ad}phen\u{ad}ation".to_owned());
}
