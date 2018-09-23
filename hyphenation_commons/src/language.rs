//! Available languages and related data.

use std::fmt;

macro_rules! fiant_linguae {
    ( $($lang:ident, $bounds:expr, $code:expr;)* ) => {
        fiant_linguae! { $($lang, $bounds, $code);* }
    };
    ( $($lang:ident, $bounds:expr, $code:expr);* ) => {
        /// The set of languages available for hyphenation.
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        pub enum Language {
            $( $lang, )*
        }

        impl Language {
            /// The default number of characters from the start and end of a word
            /// where breaks may not occur.
            pub fn minima(&self) -> (usize, usize) {
                match *self {
                    $( Language::$lang => $bounds, )*
                }
            }

            /// The TeX language code.
            pub fn code(&self) -> &'static str {
                match *self {
                    $( Language::$lang => $code, )*
                }
            }
        }

        impl fmt::Display for Language {
            fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:?}", *self)
            }
        }
    }
}

// NOTE: These hyphenation bounds were taken directly from the relevant TeX
// packages, but it is not entirely clear how well they map to the notion of
// Unicode `char` in Rust.
//
// In the worst case, a language featuring graphemes larger than 1 `char` may
// set boundaries mid-grapheme. This should be of no practical consequence,
// since well-formed hyphenation patterns only match full graphemes; moreover,
// well-behaved hyphenators are expected to validate hyphenation opportunities,
// discarding any which arise outside `char` boundaries.
fiant_linguae! {
    Afrikaans,              (1, 2),     "af";
    Armenian,               (1, 2),     "hy";
    Assamese,               (1, 1),     "as";
    Basque,                 (2, 2),     "eu";
    Belarusian,             (2, 2),     "be";
    Bengali,                (1, 1),     "bn";
    Bulgarian,              (2, 2),     "bg";
    Catalan,                (2, 2),     "ca";
    Chinese,                (1, 1),     "zh-latn-pinyin";
    Coptic,                 (1, 1),     "cop";
    Croatian,               (2, 2),     "hr";
    Czech,                  (2, 3),     "cs";
    Danish,                 (2, 2),     "da";
    Dutch,                  (2, 2),     "nl";
    EnglishGB,              (2, 3),     "en-gb";
    EnglishUS,              (2, 3),     "en-us";
    Esperanto,              (2, 2),     "eo";
    Estonian,               (2, 3),     "et";
    Ethiopic,               (1, 1),     "mul-ethi";
    Finnish,                (2, 2),     "fi";
    French,                 (2, 3),     "fr";
    Friulan,                (2, 2),     "fur";
    Galician,               (2, 2),     "gl";
    Georgian,               (1, 2),     "ka";
    German1901,             (2, 2),     "de-1901";
    German1996,             (2, 2),     "de-1996";
    GermanSwiss,            (2, 2),     "de-ch-1901";
    GreekAncient,           (1, 1),     "grc";
    GreekMono,              (1, 1),     "el-monoton";
    GreekPoly,              (1, 1),     "el-polyton";
    Gujarati,               (1, 1),     "gu";
    Hindi,                  (1, 1),     "hi";
    Hungarian,              (2, 2),     "hu";
    Icelandic,              (2, 2),     "is";
    Indonesian,             (2, 2),     "id";
    Interlingua,            (2, 2),     "ia";
    Irish,                  (2, 3),     "ga";
    Italian,                (2, 2),     "it";
    Kannada,                (1, 1),     "kn";
    Kurmanji,               (2, 2),     "kmr";
    Latin,                  (2, 2),     "la";
    LatinClassic,           (2, 2),     "la-x-classic";
    LatinLiturgical,        (2, 2),     "la-x-liturgic";
    Latvian,                (2, 2),     "lv";
    Lithuanian,             (2, 2),     "lt";
    Malayalam,              (1, 1),     "ml";
    Marathi,                (1, 1),     "mr";
    Mongolian,              (2, 2),     "mn-cyrl";
    NorwegianBokmal,        (2, 2),     "nb";
    NorwegianNynorsk,       (2, 2),     "nn";
    Occitan,                (2, 2),     "oc";
    Oriya,                  (1, 1),     "or";
    Pali,                   (1, 2),     "pi";
    Panjabi,                (1, 1),     "pa";
    Piedmontese,            (2, 2),     "pms";
    Polish,                 (2, 2),     "pl";
    Portuguese,             (2, 3),     "pt";
    Romanian,               (2, 2),     "ro";
    Romansh,                (2, 2),     "rm";
    Russian,                (2, 2),     "ru";
    Sanskrit,               (1, 3),     "sa";
    SerbianCyrillic,        (2, 2),     "sr-cyrl";
    SerbocroatianCyrillic,  (2, 2),     "sh-cyrl";
    SerbocroatianLatin,     (2, 2),     "sh-latn";
    SlavonicChurch,         (1, 2),     "cu";
    Slovak,                 (2, 3),     "sk";
    Slovenian,              (2, 2),     "sl";
    Spanish,                (2, 2),     "es";
    Swedish,                (2, 2),     "sv";
    Tamil,                  (1, 1),     "ta";
    Telugu,                 (1, 1),     "te";
    Thai,                   (2, 3),     "th";
    Turkish,                (2, 2),     "tr";
    Turkmen,                (2, 2),     "tk";
    Ukrainian,              (2, 2),     "uk";
    Uppersorbian,           (2, 2),     "hsb";
    Welsh,                  (2, 3),     "cy";
}
