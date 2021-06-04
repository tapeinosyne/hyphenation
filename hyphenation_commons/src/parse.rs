//! Pattern and exception parsing.

use dictionary::extended::{self as ext, Subregion};
use dictionary::*;

// TODO: make parsing fallible
pub trait Parse {
    type Tally: Eq;

    fn value(v : char) -> Option<u8>;
    fn tally(s : &str) -> Self::Tally;

    fn alphabetical(s : &str) -> String {
        s.chars()
         .filter(|c| Self::value(*c) == None)
         .collect()
    }

    fn pair<N>(str_klp : &str, normalize : N) -> (String, Self::Tally)
        where N : Fn(&str) -> String
    {
        let normalized = normalize(str_klp);
        (Self::alphabetical(&normalized), Self::tally(&normalized))
    }
}

impl<'a> Parse for Patterns {
    type Tally = Vec<Locus>;

    #[inline]
    fn value(c : char) -> Option<u8> { c.to_digit(10).map(|n| n as u8) }

    fn tally(pattern : &str) -> Self::Tally {
        pattern.bytes()
               .enumerate()
               .filter_map(|(i, b)| Self::value(b as char).map(|v| (i, v)))
               .enumerate()
               .map(|(j, (i, v))| Locus { index : (i - j) as u8,
                                          value : v, })
               .collect()
    }
}

impl<'a> Parse for Exceptions {
    type Tally = Vec<usize>;

    #[inline]
    fn value(c : char) -> Option<u8> {
        match c {
            '-' => Some(2),
            _ => None,
        }
    }

    fn tally(exception : &str) -> Self::Tally {
        exception.bytes()
                 .enumerate()
                 .filter_map(|(i, b)| Self::value(b as char).map(|_| i))
                 .enumerate()
                 .map(|(j, i)| i - j)
                 .collect()
    }
}

impl<'a> Parse for ext::Patterns {
    type Tally = ext::Tally;

    #[inline]
    fn value(c : char) -> Option<u8> { c.to_digit(10).map(|n| n as u8) }

    fn alphabetical(s : &str) -> String {
        match s.find('/') {
            None => Patterns::alphabetical(s),
            Some(i) => Patterns::alphabetical(&s[.. i]),
        }
    }

    fn tally(pattern : &str) -> Self::Tally {
        use std::str::FromStr;

        // TODO: refactor
        match pattern.find('/') {
            None => ext::Tally { standard :  Patterns::tally(pattern),
                                 subregion : None, },
            Some(i) => {
                // Exoneration: we unwrap liberally within this match arm, since failure
                // would denote a malformed pattern.
                let err = &["Malformed extended hyphenation pattern: ", pattern].concat();

                let (standard, extension) = (&pattern[.. i], &pattern[i + 1 ..]);
                let breakpoint = extension.find('=').expect(err);
                let sub_pattern_end = extension.find(',').expect(err);
                let sub_pattern = &extension[.. sub_pattern_end];
                let sub_idxs = &extension[sub_pattern_end + 1 ..];

                let dot_offset = if standard.starts_with('.') { 1 } else { 0 };
                let (chars_to_op, span) = {
                    let v : Vec<_> = sub_idxs.split(',')
                                             .map(|s| usize::from_str(s).expect(err))
                                             .collect();

                    assert!(v.len() == 2,
                            "Malformed extended hyphenation pattern: {}",
                            pattern);
                    (v[0] + dot_offset, v[1])
                };

                let tally = Patterns::tally(standard);
                let alphabetical = Patterns::alphabetical(standard);
                let substitution = sub_pattern.chars().filter(|&c| c.is_alphabetic()).collect();
                // Németh always starts the subregion at the character immediately preceding
                // the opportunity.
                let chars_to_start = chars_to_op.saturating_sub(1);
                let start = alphabetical.char_indices()
                                        .nth(chars_to_start)
                                        .expect(err)
                                        .0;
                let end = alphabetical.char_indices()
                                      .nth(chars_to_start + span)
                                      .expect(err)
                                      .0;
                let index = alphabetical.char_indices().nth(chars_to_op).expect(err).0 as u8;
                let (left, right) = (index as usize - start, end - index as usize);
                let value = tally.iter()
                                 .find(|&&locus| locus.index == index)
                                 .map(|&locus| locus.value)
                                 .expect(err);

                ext::Tally { standard :  tally,
                             subregion : (Locus { index, value },
                                          Subregion { left,
                                                      right,
                                                      substitution,
                                                      breakpoint })
                                                                   .into(), }
            }
        }
    }
}
