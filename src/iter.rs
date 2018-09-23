/*!
Hyphenating iterators over strings.
*/

use std::borrow::Cow;
use std::iter::{Cloned, IntoIterator, ExactSizeIterator};
use std::slice;
use std::vec;

use hyphenator::*;
use extended::*;


/// A hyphenating iterator that breaks text into segments delimited by word
/// breaks, and marks them with a hyphen where appropriate.
///
/// Such segments generally coincide with orthographic syllables, albeit
/// within the limited accuracy of Knuth-Liang hyphenation.
#[derive(Clone, Debug)]
pub struct Hyphenating<'m, I> {
    inner : I,
    mark : &'m str
}

impl<'m, I, S> Hyphenating<'m, I>
where I : Iterator<Item = S>
    , S : AsRef<str>
{
    /// Turn into an iterator that yields word segments only, without inserting
    /// a hyphen or other mark before breaks.
    pub fn segments(self) -> I {
        self.inner
    }

    /// Set the mark that will be inserted before word breaks.
    pub fn mark_with(&mut self, mark : &'m str) {
        self.mark = mark;
    }

    /// Build a hyphenating iterator from an iterator over string segments.
    pub fn new(iter : I) -> Self { Hyphenating { inner : iter, mark : "-" } }
}

impl<'m, I, S> Iterator for Hyphenating<'m, I>
where I : Iterator<Item = S> + ExactSizeIterator
    , S : AsRef<str>
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|segment|
            if self.inner.len() > 0 {
                [segment.as_ref(), self.mark].concat()
            } else { segment.as_ref().to_owned() }
        )
    }

    fn size_hint(&self) -> (usize, Option<usize>) { self.inner.size_hint() }
}

impl<'m, I, S> ExactSizeIterator for Hyphenating<'m, I>
where I : Iterator<Item = S> + ExactSizeIterator
    , S : AsRef<str> {}


/// A hyphenating iterator with borrowed data.
pub trait Iter<'t> {
    type Iter;

    fn iter(&'t self) -> Hyphenating<'t, Self::Iter>;
}

impl<'t> Iter<'t> for Word<'t, usize> {
    type Iter = Segments<'t, Cloned<slice::Iter<'t, usize>>>;

    fn iter(&'t self) -> Hyphenating<'t, Self::Iter> {
        Hyphenating::new(Segments::new(self.text, self.breaks.iter().cloned()))
    }
}

impl<'t> IntoIterator for Word<'t, usize> {
    type Item = String;
    type IntoIter = Hyphenating<'t, Segments<'t, vec::IntoIter<usize>>>;

    fn into_iter(self) -> Self::IntoIter {
        Hyphenating::new(Segments::new(self.text, self.breaks.into_iter()))
    }
}

impl<'t> IntoIterator for Word<'t, (usize, Option<&'t Subregion>)> {
    type Item = String;
    type IntoIter = Hyphenating<'t,
        SegmentsExt<'t, vec::IntoIter<(usize, Option<&'t Subregion>)>>
    >;

    fn into_iter(self) -> Self::IntoIter {
        Hyphenating::new(SegmentsExt::new(self.text, self.breaks.into_iter()))
    }
}


/// An iterator over borrowed slices delimited by Standard hyphenation
/// opportunities.
#[derive(Clone, Debug)]
pub struct Segments<'t, I> {
    text : &'t str,
    breaks : I,
    start : Option<usize>
}

impl<'t, I> Segments<'t, I> {
    pub fn new(text : &'t str, breaks : I) -> Self {
        Segments {
            text,
            breaks,
            start : Some(0)
        }
    }
}

impl<'t, I> Iterator for Segments<'t, I> where I : Iterator<Item = usize> {
    type Item = &'t str;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.breaks.next() {
            None => self.start.take().map(|i| &self.text[i ..]),
            Some(index) => {
                let (start, end) = (self.start.unwrap(), index);
                let segment = &self.text[start .. end];

                self.start = Some(end);
                Some(segment)
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.breaks.size_hint();
        let stagger = self.start.iter().len();
        (lower + stagger, upper.map(|n| n + stagger))
    }
}

impl<'t, I> ExactSizeIterator for Segments<'t, I>
where I : Iterator<Item = usize> + ExactSizeIterator {}


/// An iterator over string segments delimited by Extended hyphenation
/// opportunities. A segment may be borrowed or owned, depending on whether
/// the break requires changes to neighboring letters.
#[derive(Clone, Debug)]
pub struct SegmentsExt<'t, I> {
    text : &'t str,
    breaks : I,
    start : Option<usize>,
    queued : Option<(usize, &'t str)>
}

impl<'t, I> SegmentsExt<'t, I> {
    pub fn new(text : &'t str, breaks : I) -> Self {
        SegmentsExt {
            text,
            breaks,
            start : Some(0),
            queued : None
        }
    }

    fn substitute(&mut self, text : &'t str) -> Cow<'t, str> {
        match self.queued.take() {
            None => Cow::Borrowed(text),
            Some((skip, ref subst)) => Cow::Owned([subst, &text[skip ..]].concat())
        }
    }
}



impl<'t, I>  Iterator for SegmentsExt<'t, I>
where I : Iterator<Item = (usize, Option<&'t Subregion>)> {
    type Item = Cow<'t, str>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.breaks.next() {
            None => self.start.take().map(|start| self.substitute(&self.text[start ..])),
            Some((index, None)) => {
                let start = self.start.unwrap();
                self.start = Some(index);
                Some(self.substitute(&self.text[start .. index]))
            },
            Some((index, Some(ref subr))) => {
                let (start, end) = (self.start.unwrap(), index);
                self.start = Some(index);

                let (segment_start, fore) = self.queued.take().unwrap_or((start, ""));
                let (segment_end, aft) = {
                    let (subst, queued) = subr.substitution.split_at(subr.breakpoint);
                    if queued.len() > 0 {
                        self.queued = Some((subr.right, queued));
                    }
                    (end - subr.left, subst)
                };

                let segment = [fore, &self.text[segment_start .. segment_end], aft].concat();
                Some(Cow::Owned(segment))
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.breaks.size_hint();
        let stagger = self.start.iter().len();
        (lower + stagger, upper.map(|n| n + stagger))
    }
}

impl<'t, I> ExactSizeIterator for SegmentsExt<'t, I>
where I : Iterator<Item = (usize, Option<&'t Subregion>)>
        + ExactSizeIterator {}
