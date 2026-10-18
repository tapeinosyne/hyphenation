use fst::raw;
use fst::Map;
use rkyv::rancor::{Fallible, Source as RkyvSource};
use rkyv::ser::{Allocator, Writer};
use rkyv::vec::{ArchivedVec, VecResolver};
use rkyv::with::{ArchiveWith, DeserializeWith, SerializeWith};
use rkyv::Place;

use std::convert::From;
use std::error;
use std::fmt;
use std::slice;


#[derive(Clone, Debug, Default)]
pub struct Trie(Map<Vec<u8>>);

impl Trie {
    pub fn as_bytes(&self) -> &[u8] { self.0.as_fst().as_bytes() }

    pub fn from_bytes(bs : Vec<u8>) -> Result<Self, Error> {
        let map = Map::new(bs)?;
        Ok(Trie(map))
    }

    pub fn from_iter<I>(iter : I) -> Result<Self, Error>
        where I : Iterator<Item = (String, u64)>
    {
        let m = fst::Map::from_iter(iter)?;
        Ok(Trie(m))
    }

    pub fn get_prefixes<'f, 'q>(&'f self, query : &'q [u8]) -> PrefixMatches<'f, 'q> {
        let fst = self.0.as_fst();
        PrefixMatches { fst,
                        node : fst.root(),
                        output : raw::Output::zero(),
                        query : query.iter() }
    }
}

#[derive(Clone)]
pub struct PrefixMatches<'f, 'q> {
    fst :    &'f raw::Fst<Vec<u8>>,
    node :   raw::Node<'f>,
    output : raw::Output,
    query :  slice::Iter<'q, u8>,
}

impl<'f, 'q> Iterator for PrefixMatches<'f, 'q> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let prefix = &mut self.query;
        for &b in prefix {
            match self.node.find_input(b) {
                None => return None,
                Some(i) => {
                    let t = self.node.transition(i);
                    self.output = self.output.cat(t.out);
                    self.node = self.fst.node(t.addr);
                    if self.node.is_final() {
                        let final_output = self.output.cat(self.node.final_output());
                        return Some(final_output.value());
                    }
                }
            }
        }

        None
    }
}


impl AsRef<fst::Map<Vec<u8>>> for Trie {
    fn as_ref(&self) -> &fst::Map<Vec<u8>> { &self.0 }
}

impl AsMut<fst::Map<Vec<u8>>> for Trie {
    fn as_mut(&mut self) -> &mut fst::Map<Vec<u8>> { &mut self.0 }
}

impl From<fst::Map<Vec<u8>>> for Trie {
    fn from(m : fst::Map<Vec<u8>>) -> Self { Trie(m) }
}


/*
Serialization
*/

/// Archives a [`Trie`] as the raw bytes of its underlying FST.
pub struct TrieAsBytes;

impl ArchiveWith<Trie> for TrieAsBytes {
    type Archived = ArchivedVec<u8>;
    type Resolver = VecResolver;

    fn resolve_with(field : &Trie, resolver : Self::Resolver, out : Place<Self::Archived>) {
        ArchivedVec::resolve_from_slice(field.as_bytes(), resolver, out);
    }
}

impl<S> SerializeWith<Trie, S> for TrieAsBytes
    where S : Fallible + Allocator + Writer + ?Sized,
          S::Error : RkyvSource
{
    fn serialize_with(field : &Trie, serializer : &mut S) -> Result<Self::Resolver, S::Error> {
        ArchivedVec::serialize_from_slice(field.as_bytes(), serializer)
    }
}

impl<D> DeserializeWith<ArchivedVec<u8>, Trie, D> for TrieAsBytes
    where D : Fallible + ?Sized,
          D::Error : RkyvSource
{
    fn deserialize_with(field : &ArchivedVec<u8>, _ : &mut D) -> Result<Trie, D::Error> {
        Trie::from_bytes(field.to_vec()).map_err(D::Error::new)
    }
}


#[derive(Debug)]
pub struct Error(pub fst::Error);

impl fmt::Display for Error {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = format!("The dictionary's internal trie could not be built:\n{}",
                              &self.0);

        f.write_str(&message)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> { Some(&self.0) }
}

impl From<fst::Error> for Error {
    fn from(err : fst::Error) -> Self { Error(err) }
}
