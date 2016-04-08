use std::io::{Read, Write};
use std::ops::Deref;

use io::binary::Binary;
use io::binary::UNKNOWN_SIZE;
use io::result::Result;
use util::bitvec::BitVec;
use util::index::{IndexUnchecked, IndexSetUnchecked, IndexSet};
use util::property::handle::Handle;
use util::property::size::{Size, INVALID_INDEX};
use util::property::traits;
use util::property::traits::Handle as HandleTrait; // to allow index_us().

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
/// For type safety, it is parametrized by the Handle type `H` which differentiates whether this is
/// a vertex, halfedge, edge, or face property.
#[derive(Clone)]
pub struct Property<T, H> {
    name: String,
    persistent: bool,
    vec: Vec<T>,
    _m: ::std::marker::PhantomData<H>
}

impl<T, H> Property<T, H> {
    pub fn new(name: String) -> Property<T, H> {
        Property {
            name: name,
            persistent: false,
            vec: Vec::new(),
            _m: ::std::marker::PhantomData
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Index impls (pass through to vec).

impl<T, H: Copy + Deref<Target=Handle>> ::std::ops::Index<H> for Property<T, H> {
    type Output = T;
    fn index(&self, index: H) -> &Self::Output {
        self.vec.index(index.index_us())
    }
}

impl<T, H: Copy + Deref<Target=Handle>> IndexUnchecked<H> for Property<T, H> {
    unsafe fn index_unchecked(&self, index: H) -> &Self::Output {
        self.vec.index_unchecked(index.index_us())
    }
}

impl<T, H: Copy + Deref<Target=Handle>> IndexSetUnchecked<H> for Property<T, H> {
    unsafe fn index_set_unchecked(&mut self, index: H, value: T) {
        self.vec.index_set_unchecked(index.index_us(), value);
    }
}

impl<T, H: Copy + Deref<Target=Handle>> IndexSet<H> for Property<T, H> {
    fn index_set(&mut self, index: H, value: T) {
        self.vec.index_set(index.index_us(), value);
    }
}

////////////////////////////////////////////////////////////////////////////////
// impl `std::fmt::Debug`

impl<T, H> ::std::fmt::Debug for Property<T, H> {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        writeln!(formatter, "  {}{}", self.name, if self.persistent { ", persistent" } else { "" })
    }
}

////////////////////////////////////////////////////////////////////////////////
// impl `traits::Property`

impl<T, H> traits::Property<H> for Property<T, H>
    where T: Clone + Binary + Default + 'static,
          H: ::std::any::Any + Copy + Deref<Target=Handle> + 'static,
          Property<T, H>: ::std::any::Any,
          Vec<T>: Binary
{
    impl_property_accessors!(<T as Binary>::is_streamable());

    ////////////////////////////////////////
    // synchronized array interface

    fn reserve(&mut self, n: Size) {
        if n >= INVALID_INDEX {
            panic!("Reserve dimensions {} exceeded bounds {}-1", n, INVALID_INDEX);
        }
        let n = n as usize;
        let len = self.vec.len();
        if n > len {
            self.vec.reserve(n - len);
        }
    }
    fn resize(&mut self, n: Size) {
        if n >= INVALID_INDEX {
            panic!("Resize dimensions {} exceeded bounds {}-1", n, INVALID_INDEX);
        }
        self.vec.resize(n as usize, Default::default());
    }
    fn clear(&mut self) { self.vec.clear(); }
    fn push(&mut self) { self.vec.push(Default::default()); }
    fn swap(&mut self, i0: H, i1: H) {
        self.vec.swap(i0.index_us(), i1.index_us());
    }
    fn copy(&mut self, i_src: H, i_dst: H) {
        self.vec[i_dst.index_us()] = self.vec[i_src.index_us()].clone();
    }
    fn clone_as_trait(&self) -> Box<traits::Property<H>> { Box::new(self.clone()) }

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
/// For type safety, it is parametrized by the Handle type which differentiates whether this is a
/// vertex, halfedge, edge, or face property.
#[derive(Clone)]
pub struct PropertyBits<H> {
    name: String,
    persistent: bool,
    vec: BitVec,
    _m: ::std::marker::PhantomData<H>
}

impl<H> PropertyBits<H> {
    pub fn new(name: String) -> PropertyBits<H> {
        PropertyBits {
            name: name,
            persistent: false,
            vec: BitVec::new(),
            _m: ::std::marker::PhantomData
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Index impls (pass through to vec).

impl<H: Copy + Deref<Target=Handle>> ::std::ops::Index<H> for PropertyBits<H> {
    type Output = bool;
    fn index(&self, index: H) -> &Self::Output {
        self.vec.index(index.index_us())
    }
}

impl<H: Copy + Deref<Target=Handle>> IndexUnchecked<H> for PropertyBits<H> {
    unsafe fn index_unchecked(&self, index: H) -> &Self::Output {
        self.vec.index_unchecked(index.index_us())
    }
}

impl<H: Copy + Deref<Target=Handle>> IndexSetUnchecked<H> for PropertyBits<H> {
    unsafe fn index_set_unchecked(&mut self, index: H, value: bool) {
        self.vec.index_set_unchecked(index.index_us(), value);
    }
}

impl<H: Copy + Deref<Target=Handle>> IndexSet<H> for PropertyBits<H> {
    fn index_set(&mut self, index: H, value: bool) {
        self.vec.index_set(index.index_us(), value);
    }
}

////////////////////////////////////////////////////////////////////////////////
// impl `std::fmt::Debug`

impl<H> ::std::fmt::Debug for PropertyBits<H> {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        writeln!(formatter, "  {}{}", self.name, if self.persistent { ", persistent" } else { "" })
    }
}

////////////////////////////////////////////////////////////////////////////////
// impl `traits::Property`

impl<H> traits::Property<H> for PropertyBits<H>
    where H: ::std::any::Any + Copy + Deref<Target=Handle> + 'static
{
    impl_property_accessors!(true); // is_streamable = true

    ////////////////////////////////////////
    // synchronized array interface

    fn reserve(&mut self, n: Size) {
        if n >= INVALID_INDEX {
            panic!("Reserve dimensions {} exceeded bounds {}-1", n, INVALID_INDEX);
        }
        let n = n as usize;
        let len = self.vec.len();
        if n > len {
            self.vec.reserve(n - len);
        }
    }
    fn resize(&mut self, n: Size) {
        if n >= INVALID_INDEX {
            panic!("Resize dimensions {} exceeded bounds {}-1", n, INVALID_INDEX);
        }
        self.vec.resize(n as usize, Default::default());
    }
    fn clear(&mut self) { self.vec.clear(); }
    fn push(&mut self) { self.vec.push(Default::default()); }
    fn swap(&mut self, i0: H, i1: H) {
        self.vec.swap(i0.index_us(), i1.index_us());
    }
    fn copy(&mut self, i_src: H, i_dst: H) {
        let value = self.vec[i_src.index_us()];
        self.vec.set(i_dst.index_us(), value);
    }
    fn clone_as_trait(&self) -> Box<traits::Property<H>> { Box::new(self.clone()) }

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

