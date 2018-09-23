/*! Extended Knuth-Liang hyphenation

Extended (“non-standard”[^1]) hyphenation supports orthographies where letters
can change around word breaks. Opportunities produced by [`Extended`]
dictionaries carry an optional [`Subregion`] to describe such changes.


[^1]: László Németh, [Automatic non-standard hyphenation in OpenOffice.org](https://www.tug.org/TUGboat/tb27-1/tb86nemeth.pdf)

[`Extended`]: struct.Extended.html
[`Subregion`]: struct.Subregion.html
*/

pub use hyphenation_commons::dictionary::extended::{Extended, Subregion};
