use std::io::{Read, Write};

use io::binary::Binary;
use io::binary::UNKNOWN_SIZE;
use io::result::Result;
use util::bitvec::BitVec;
use util::index::{IndexUnchecked, IndexSetUnchecked, IndexSet};
use util::property::traits;

/// Implements getter/setters for the `name` and `persistent` properties.
/// `$is_streamable` indicates whether the property is streamable, and thus, whether `persistent`
/// can ever be set to `true`.
#[macro_export]
macro_rules! impl_property_accessors {
    ($is_streamable: expr) => {
        fn name(&self) -> &str { &self.name }
        fn persistent(&self) -> bool { self.persistent }
        fn set_persistent(&mut self, persistent: bool) {
            if persistent && $is_streamable {
                omerr!("Warning! Type of property value is not binary storable!");
            }
            self.persistent = $is_streamable && persistent;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Named property encapsulating a `Vec` of some type.
#[derive(Clone)]
pub struct Property<T> {
    name: String,
    persistent: bool,
    vec: Vec<T>
}

impl<T> Property<T> {
    pub fn new(name: String) -> Property<T> {
        Property {
            name: name,
            persistent: false,
            vec: Vec::new()
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Index impls (pass through to vec).

impl<T> ::std::ops::Index<usize> for Property<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.vec.index(index)
    }
}

impl<T> IndexUnchecked<usize> for Property<T> {
    unsafe fn index_unchecked(&self, index: usize) -> &Self::Output {
        self.vec.index_unchecked(index)
    }
}

impl<T> IndexSetUnchecked<usize> for Property<T> {
    unsafe fn index_set_unchecked(&mut self, index: usize, value: T) {
        self.vec.index_set_unchecked(index, value);
    }
}

impl<T> IndexSet<usize> for Property<T> {
    fn index_set(&mut self, index: usize, value: T) {
        self.vec.index_set(index, value);
    }
}

////////////////////////////////////////////////////////////////////////////////
// impl `std::fmt::Debug`

impl<T> ::std::fmt::Debug for Property<T> {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        writeln!(formatter, "  {}{}", self.name, if self.persistent { ", persistent" } else { "" })
    }
}

////////////////////////////////////////////////////////////////////////////////
// impl `traits::Property`

impl<T: Clone + Binary + Default + 'static> traits::Property for Property<T>
    where Property<T>: ::std::any::Any,
          Vec<T>: Binary
{
    impl_property_accessors!(<T as Binary>::is_streamable());

    ////////////////////////////////////////
    // synchronized array interface

    fn reserve(&mut self, n: usize) {
        let len = self.vec.len();
        if n > len {
            self.vec.reserve(n - len);
        }
    }
    fn resize(&mut self, n: usize) { self.vec.resize(n, Default::default()); }
    fn clear(&mut self) { self.vec.clear(); }
    fn push(&mut self) { self.vec.push(Default::default()); }
    fn swap(&mut self, i0: usize, i1: usize) { self.vec.swap(i0, i1); }
    fn copy(&mut self, i_src: usize, i_dst: usize) {
        self.vec[i_dst] = self.vec[i_src].clone();
    }
    fn clone_as_trait(&self) -> Box<traits::Property> { Box::new(self.clone()) }

    ////////////////////////////////////////
    // I/O support

    fn n_elements(&self) -> usize { self.vec.len() }
    fn element_size(&self) -> usize { <T as Binary>::size_of_type() }
    fn size_of(&self) -> usize { <Vec<T> as Binary>::size_of_value(&self.vec) }
    fn store(&self, writer: &mut Write, swap: bool) -> Result<usize> {
        <Vec<T> as Binary>::store(&self.vec, writer, swap)
    }
    fn restore(&mut self, reader: &mut Read, swap: bool) -> Result<usize> {
        <Vec<T> as Binary>::restore(&mut self.vec, reader, swap)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

/// Named property encapsulating a `BitVec`.
#[derive(Clone)]
pub struct PropertyBits {
    name: String,
    persistent: bool,
    vec: BitVec
}

impl PropertyBits {
    pub fn new(name: String) -> PropertyBits {
        PropertyBits {
            name: name,
            persistent: false,
            vec: BitVec::new()
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Index impls (pass through to vec).

impl ::std::ops::Index<usize> for PropertyBits {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {
        self.vec.index(index)
    }
}

impl IndexUnchecked<usize> for PropertyBits {
    unsafe fn index_unchecked(&self, index: usize) -> &Self::Output {
        self.vec.index_unchecked(index)
    }
}

impl IndexSetUnchecked<usize> for PropertyBits {
    unsafe fn index_set_unchecked(&mut self, index: usize, value: bool) {
        self.vec.index_set_unchecked(index, value);
    }
}

impl IndexSet<usize> for PropertyBits {
    fn index_set(&mut self, index: usize, value: bool) {
        self.vec.index_set(index, value);
    }
}

////////////////////////////////////////////////////////////////////////////////
// impl `std::fmt::Debug`

impl ::std::fmt::Debug for PropertyBits {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        writeln!(formatter, "  {}{}", self.name, if self.persistent { ", persistent" } else { "" })
    }
}

////////////////////////////////////////////////////////////////////////////////
// impl `traits::Property`

impl traits::Property for PropertyBits {
    impl_property_accessors!(true); // is_streamable = true

    ////////////////////////////////////////
    // synchronized array interface

    fn reserve(&mut self, n: usize) {
        let len = self.vec.len();
        if n > len {
            self.vec.reserve(n - len);
        }
    }
    fn resize(&mut self, n: usize) { self.vec.resize(n, Default::default()); }
    fn clear(&mut self) { self.vec.clear(); }
    fn push(&mut self) { self.vec.push(Default::default()); }
    fn swap(&mut self, i0: usize, i1: usize) { self.vec.swap(i0, i1); }
    fn copy(&mut self, i_src: usize, i_dst: usize) {
        let value = self.vec[i_src];
        self.vec.set(i_dst, value);
    }
    fn clone_as_trait(&self) -> Box<traits::Property> { Box::new(self.clone()) }

    ////////////////////////////////////////
    // I/O support

    fn n_elements(&self) -> usize { self.vec.len() }
    fn element_size(&self) -> usize { UNKNOWN_SIZE }
    fn size_of(&self) -> usize { <BitVec as Binary>::size_of_value(&self.vec) }
    fn store(&self, writer: &mut Write, swap: bool) -> Result<usize> {
        <BitVec as Binary>::store(&self.vec, writer, swap)
    }
    fn restore(&mut self, reader: &mut Read, swap: bool) -> Result<usize> {
        <BitVec as Binary>::restore(&mut self.vec, reader, swap)
    }
}

////////////////////////////////////////////////////////////////////////////////

