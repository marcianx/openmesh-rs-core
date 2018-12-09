use std::io::{Read, Write};
use crate::io::binary::{Binary, Endian};
use crate::io::binary::UNKNOWN_SIZE;
use crate::io::result::Result;
use crate::util::bitvec::BitVec;
use crate::util::index::{IndexUnchecked, IndexSetUnchecked, IndexSet};
use crate::property::size::{Size, INVALID_INDEX};
use crate::property::traits::{self, ItemHandle, PropertyFor};
use crate::property::traits::{ConstructableProperty, ResizeableProperty};

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
///
/// Note that for the reflection-based implementation to work, the user-composed type `T` that is
/// stored in `Property` **must** satisfy the bound `T: traits::Value`.
///
/// The bound is not placed on this struct to avoid replicating it on most of the impls, which
/// don't require this bound.
#[derive(Clone)]
pub struct Property<T, H> {
    name: String,
    persistent: bool,
    vec: Vec<T>,
    _m: ::std::marker::PhantomData<H>
}

////////////////////////////////////////////////////////////////////////////////
// Index impls (pass through to vec).

impl<T, H: ItemHandle> ::std::ops::Index<H> for Property<T, H> {
    type Output = T;
    fn index(&self, index: H) -> &Self::Output {
        self.vec.index(index.index_us())
    }
}

impl<T, H: ItemHandle> ::std::ops::IndexMut<H> for Property<T, H> {
    fn index_mut(&mut self, index: H) -> &mut Self::Output {
        self.vec.index_mut(index.index_us())
    }
}

impl<T, H: ItemHandle> IndexUnchecked<H> for Property<T, H> {
    unsafe fn index_unchecked(&self, index: H) -> &Self::Output {
        self.vec.index_unchecked(index.index_us())
    }
}

impl<T, H: ItemHandle> IndexSetUnchecked<H> for Property<T, H> {
    unsafe fn index_set_unchecked(&mut self, index: H, value: T) {
        self.vec.index_set_unchecked(index.index_us(), value);
    }
}

impl<T, H: ItemHandle> IndexSet<H> for Property<T, H> {
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

impl<T, H> traits::Property for Property<T, H>
    where T: traits::Value,
          H: ItemHandle
{
    type Handle = H;

    impl_property_accessors!(<T as Binary>::is_streamable());

    ////////////////////////////////////////
    // synchronized array interface

    fn swap(&mut self, i0: H, i1: H) {
        self.vec.swap(i0.index_us(), i1.index_us());
    }
    fn copy(&mut self, i_src: H, i_dst: H) {
        self.vec[i_dst.index_us()] = self.vec[i_src.index_us()].clone();
    }

    ////////////////////////////////////////
    // I/O support

    fn n_elements(&self) -> usize { self.vec.len() }
    fn element_size(&self) -> usize { <T as Binary>::size_of_type() }
    fn size_of(&self) -> usize { <Vec<T> as Binary>::size_of_value(&self.vec) }
    fn store(&self, writer: &mut Write, endian: Endian) -> Result<usize> {
        <Vec<T> as Binary>::store(&self.vec, writer, endian)
    }
    fn restore(&mut self, reader: &mut Read, endian: Endian) -> Result<usize> {
        <Vec<T> as Binary>::restore(&mut self.vec, reader, endian)
    }
}

impl<T, H> ResizeableProperty for Property<T, H>
    where T: traits::Value,
          H: ItemHandle
{
    fn reserve(&mut self, n: Size) {
        let n = n as usize;
        let len = self.vec.len();
        if n > len {
            self.vec.reserve(n - len);
        }
    }
    fn resize(&mut self, n: Size) {
        self.vec.resize(n as usize, Default::default());
    }
    fn clear(&mut self) { ::std::mem::swap(&mut self.vec, &mut Vec::new()); }
    fn push(&mut self) { self.vec.push(Default::default()); }
    fn clone_as_trait(&self) -> Box<ResizeableProperty<Handle=H>> { Box::new(self.clone()) }
    fn as_property(&self) -> &traits::Property<Handle=H> { self }
    fn as_property_mut(&mut self) -> &mut traits::Property<Handle=H> { self }
}

impl<T, H> ConstructableProperty for Property<T, H>
    where T: traits::Value,
          H: ItemHandle
{
    fn new(name: String, size: Size) -> Self {
        let mut prop = Property {
            name,
            persistent: false,
            vec: Vec::new(),
            _m: ::std::marker::PhantomData
        };
        prop.resize(size);
        prop
    }
}

impl<T, H> PropertyFor<H> for T
    where T: traits::Value,
          H: ItemHandle
{
    default type Property = Property<T, H>;
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

////////////////////////////////////////////////////////////////////////////////
// Index impls (pass through to vec).

impl<H: ItemHandle> ::std::ops::Index<H> for PropertyBits<H> {
    type Output = bool;
    fn index(&self, index: H) -> &Self::Output {
        self.vec.index(index.index_us())
    }
}

impl<H: ItemHandle> IndexUnchecked<H> for PropertyBits<H> {
    unsafe fn index_unchecked(&self, index: H) -> &Self::Output {
        self.vec.index_unchecked(index.index_us())
    }
}

impl<H: ItemHandle> IndexSetUnchecked<H> for PropertyBits<H> {
    unsafe fn index_set_unchecked(&mut self, index: H, value: bool) {
        self.vec.index_set_unchecked(index.index_us(), value);
    }
}

impl<H: ItemHandle> IndexSet<H> for PropertyBits<H> {
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

impl<H> traits::Property for PropertyBits<H>
    where H: ItemHandle
{
    type Handle = H;

    impl_property_accessors!(true); // is_streamable = true

    ////////////////////////////////////////
    // synchronized array interface

    fn swap(&mut self, i0: H, i1: H) {
        self.vec.swap(i0.index_us(), i1.index_us());
    }
    fn copy(&mut self, i_src: H, i_dst: H) {
        let value = self.vec[i_src.index_us()];
        self.vec.set(i_dst.index_us(), value);
    }

    ////////////////////////////////////////
    // I/O support

    fn n_elements(&self) -> usize { self.vec.len() }
    fn element_size(&self) -> usize { UNKNOWN_SIZE }
    fn size_of(&self) -> usize { <BitVec as Binary>::size_of_value(&self.vec) }
    fn store(&self, writer: &mut Write, endian: Endian) -> Result<usize> {
        <BitVec as Binary>::store(&self.vec, writer, endian)
    }
    fn restore(&mut self, reader: &mut Read, endian: Endian) -> Result<usize> {
        <BitVec as Binary>::restore(&mut self.vec, reader, endian)
    }
}

impl<H> ResizeableProperty for PropertyBits<H>
    where H: ItemHandle
{
    fn reserve(&mut self, n: Size) {
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
    fn clear(&mut self) { ::std::mem::swap(&mut self.vec, &mut BitVec::new()); }
    fn push(&mut self) { self.vec.push(Default::default()); }
    fn clone_as_trait(&self) -> Box<ResizeableProperty<Handle=H>> { Box::new(self.clone()) }
    fn as_property(&self) -> &traits::Property<Handle=H> { self }
    fn as_property_mut(&mut self) -> &mut traits::Property<Handle=H> { self }
}

impl<H> ConstructableProperty for PropertyBits<H>
    where H: ItemHandle
{
    fn new(name: String, size: Size) -> PropertyBits<H> {
        let mut prop = PropertyBits {
            name,
            persistent: false,
            vec: BitVec::new(),
            _m: ::std::marker::PhantomData
        };
        prop.resize(size);
        prop
    }
}

impl<H> PropertyFor<H> for bool
    where H: ItemHandle
{
    type Property = PropertyBits<H>;
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use crate::property::Property;
    use crate::property::traits::{self, ConstructableProperty, ItemHandle};

    fn _assert_any<P: ::std::any::Any>(_p: P) {}

    // This method isn't used anywhere. Instead, it serves as a compile-time assertion that the
    // constraints `T: traits::Value` and `H: ItemHandle` imply `Property: ::std::any::Any`.
    // Test compilation will fail here if this fact is violated.
    fn _assert_property_any<T: traits::Value, H: ItemHandle>() {
        _assert_any(Property::<T, H>::new("test".into(), 10));
    }
}
