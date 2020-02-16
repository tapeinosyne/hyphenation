/*! # Evaluating potential hyphenation opportunities

In Knuth–Liang hyphenation, dictionaries identify valid word breaks by
searching whole words for sub-word patterns, ultimately producing a
*score* that evaluates each interval between letters as a candidate for
hyphenation.

As an example, consider the word "firkin":

```text
    f|i|r|k|i|n
```

Our British English dictionary recognizes six patterns therein, each of
which assigns a numeric value to one or more locations; when a location
is assigned multiple values, the highest one prevails. Patterns are so
compounded until a final score is produced. Using guillemets to mark
word boundaries, we find:

```text
   «f|i|r|k|i|n»
   -------------
   «f i2
      i2r
        r2k
        r5k i n
          k1i n
          k2i n»
   -------------
    f|i|r|k|i|n
     0 2 5 2 0
```

By convention, even values inhibit hyphenation, whereas odd values mark
valid breaks. Thus, having matched these patterns, the dictionary will
offer "fir·kin" as a valid hyphenation.
*/

use hyphenation_commons::dictionary::*;
use hyphenation_commons::dictionary::extended::*;


/// Methods to evaluate each index in a string as an opportunity for hyphenation.
pub trait Score<'d> {
    /// A value assigned to each index — which is to say, to each potential break
    /// between letters — to determine whether the string can be broken at that
    /// index.
    type Value;

    /// Generate a word's hyphenation score, from which opportunities may be
    /// extracted.
    ///
    /// The `Standard` and `Extended` dictionaries mark each byte index with
    /// an integer value, where an odd value marks the index as a valid break.
    fn score(&'d self, word : &str) -> Vec<Self::Value>;

    /// Whether the given value denotes a valid break.
    fn denotes_opportunity(value : Self::Value) -> bool;
}

impl<'d> Score<'d> for Standard {
    type Value = u8;

    #[inline] fn denotes_opportunity(v : Self::Value) -> bool { v % 2 != 0 }

    fn score(&'d self, word : &str) -> Vec<u8> {
        let match_str = [".", word, "."].concat();
        let hyphenable_length = word.len();
        let mut values : Vec<u8> = vec![0; hyphenable_length.saturating_sub(1)];

        for i in 0 .. match_str.len() - 1 {
            let substring = &match_str.as_bytes()[i..];
            for tally in self.prefix_tallies(substring) {
                for &Locus { index, value } in tally {
                    let k = i + index as usize;
                    if k > 1 && k <= hyphenable_length && value > values[k - 2] {
                        values[k - 2] = value;
                    }
                }
            }
        }
        values
    }

}

impl<'d> Score<'d> for Extended {
    type Value = (u8, Option<&'d Subregion>);

    #[inline] fn denotes_opportunity((v, _) : Self::Value) -> bool { v % 2 != 0 }

    fn score(&'d self, word : &str) -> Vec<Self::Value> {
        let match_str = [".", word, "."].concat();
        let hyphenable_length = word.len();
        let mut values : Vec<u8> = vec![0; hyphenable_length.saturating_sub(1)];
        let mut regions : Vec<Option<&Subregion>> = vec![None; values.len()];

        for i in 0 .. match_str.len() - 1 {
            let substring = &match_str.as_bytes()[i ..];
            for tally in self.prefix_tallies(substring) {
                // NOTE: By convention, competing standard and non-standard patterns
                // may not assign equal values to the same location.
                for &(Locus { index, value }, ref r) in tally.subregion.iter() {
                    let k = i + index as usize;
                    if k > 1 && k <= hyphenable_length && value > values[k - 2] {
                        values[k - 2] = value;
                        regions[k - 2] = Some(r);
                    }
                }
                // The order of these two traversals matters, because—
                for &Locus { index, value } in tally.standard.iter() {
                    let k = i + index as usize;
                    // —if a subregion was previously assigned to this location,
                    // then `w == values[k - 2]`, and it will not be replaced.
                    if k > 1 && k <= hyphenable_length && value > values[k - 2] {
                        values[k - 2] = value;
                        regions[k - 2] = None;
                    }
                }
            }
        }

        values.into_iter().zip(regions).collect()
    }
}
