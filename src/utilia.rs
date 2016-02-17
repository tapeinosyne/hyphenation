//! Nonspecific utilities and helpers for matters tangential to hyphenation.

use std::iter::Fuse;


pub struct Intersperse<I> where I: Iterator {
    inner: Fuse<I>,
    element: I::Item,
    sequent: Option<I::Item>
}

impl<I> Iterator for Intersperse<I> where
    I: Iterator,
    I::Item: Clone
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        match self.sequent {
            Some(_) => self.sequent.take(),
            None => self.inner.next().and_then(|e| {
                self.sequent = Some(e);
                Some(self.element.clone())
            })
        }
    }
}


pub trait Interspersable : Iterator {
    fn intersperse(self, e: Self::Item) -> Intersperse<Self> where Self: Sized;
}

impl<I> Interspersable for I where I: Iterator {
    fn intersperse(self: I, e: I::Item) -> Intersperse<I> {
        let mut i = self.fuse();
        let s = i.next();

        Intersperse {
            inner: i,
            element: e,
            sequent: s
        }
    }
}
