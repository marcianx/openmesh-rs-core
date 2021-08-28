use crate::io::binary::{Binary, Endian};
use crate::io::result::Result;
use crate::property::Storage;
use crate::property::{ConstructableProperty, Property, ResizeableProperty, StorageFor};
use crate::property::{ItemHandle, Size, Value, INVALID_INDEX};
use crate::util::index::{IndexSet, IndexSetUnchecked, IndexUnchecked};
use std::io::{Read, Write}; // For methods.

/// Implements getter/setters for the `name` and `persistent` properties.
/// `$is_streamable` indicates whether the property is streamable, and thus, whether `persistent`
/// can ever be set to `true`.
macro_rules! impl_property_accessors {
    ($is_streamable: expr) => {
        fn name(&self) -> &str {
            &self.name
        }
        fn persistent(&self) -> bool {
            self.persistent
        }
        fn set_persistent(&mut self, persistent: bool) {
            if persistent && $is_streamable {
                omerr!("Warning! Type of property value is not binary storable!");
            }
            self.persistent = $is_streamable && persistent;
        }
    };
}

////////////////////////////////////////////////////////////////////////////////

/// Named property encapsulating a `Vec` of some type.
/// For type safety, it is parametrized by the Handle type `H` which differentiates whether this is
/// a vertex, halfedge, edge, or face property.
///
/// Note that for the reflection-based implementation to work, the user-composed type `T` that is
/// stored in `PropertyList` **must** satisfy the bound `T: Value`.
///
/// The bound is not placed on this struct to avoid replicating it on most of the impls, which
/// don't require this bound.
#[derive(Clone)]
pub struct PropertyList<T: Value, H> {
    name: String,
    persistent: bool,
    pub(crate) storage: <T as StorageFor>::Storage, // exposed for tests only
    _m: ::std::marker::PhantomData<H>,
}

type StorageForValue<T> = <T as StorageFor>::Storage;

////////////////////////////////////////////////////////////////////////////////
// Index impls (pass through to vec).

impl<T: Value, H: ItemHandle> ::std::ops::Index<H> for PropertyList<T, H> {
    type Output = T;
    fn index(&self, index: H) -> &Self::Output {
        self.storage.get(index.index_us())
    }
}

// This one is only for `Vec<T>` since IndexMut cannot be implmented for `BitVec`.
impl<T: Value, H: ItemHandle> ::std::ops::IndexMut<H> for PropertyList<T, H>
where
    T: StorageFor<Storage = Vec<T>>,
{
    fn index_mut(&mut self, index: H) -> &mut Self::Output {
        &mut self.storage[index.index_us()]
    }
}

impl<T: Value, H: ItemHandle> IndexUnchecked<H> for PropertyList<T, H> {
    unsafe fn index_unchecked(&self, index: H) -> &Self::Output {
        self.storage.get_unchecked(index.index_us())
    }
}

impl<T: Value, H: ItemHandle> IndexSetUnchecked<H> for PropertyList<T, H> {
    unsafe fn index_set_unchecked(&mut self, index: H, value: Self::Output) {
        self.storage.set_unchecked(index.index_us(), value);
    }
}

impl<T: Value, H: ItemHandle> IndexSet<H> for PropertyList<T, H> {
    fn index_set(&mut self, index: H, value: Self::Output) {
        self.storage.set(index.index_us(), value);
    }
}

////////////////////////////////////////////////////////////////////////////////
// impl `std::fmt::Debug`

impl<T: Value, H> ::std::fmt::Debug for PropertyList<T, H> {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        writeln!(
            formatter,
            "  {}{}",
            self.name,
            if self.persistent { ", persistent" } else { "" }
        )
    }
}

////////////////////////////////////////////////////////////////////////////////
// impl `Property`

impl<T, H> Property for PropertyList<T, H>
where
    T: Value,
    H: ItemHandle,
{
    type Handle = H;

    impl_property_accessors!(<T as Binary>::is_streamable());

    ////////////////////////////////////////
    // synchronized array interface

    fn swap(&mut self, i0: H, i1: H) {
        self.storage.swap(i0.index_us(), i1.index_us());
    }
    fn copy(&mut self, i_src: H, i_dst: H) {
        let value = self.storage.get(i_src.index_us()).clone();
        self.storage.set(i_dst.index_us(), value);
    }

    ////////////////////////////////////////
    // I/O support

    fn len(&self) -> usize {
        self.storage.len()
    }
    fn element_size(&self) -> usize {
        <T as Binary>::size_of_type()
    }
    fn size_of(&self) -> usize {
        <StorageForValue<T> as Binary>::size_of_value(&self.storage)
    }
    fn store(&self, writer: &mut dyn Write, endian: Endian) -> Result<usize> {
        <StorageForValue<T> as Binary>::store(&self.storage, writer, endian)
    }
    fn restore(&mut self, reader: &mut dyn Read, endian: Endian) -> Result<usize> {
        <StorageForValue<T> as Binary>::restore(&mut self.storage, reader, endian)
    }
}

impl<T, H> ResizeableProperty for PropertyList<T, H>
where
    T: Value,
    H: ItemHandle,
{
    fn reserve(&mut self, n: Size) {
        let n = n as usize;
        let len = self.storage.len();
        if n > len {
            self.storage.reserve_more(n - len);
        }
    }
    #[allow(clippy::absurd_extreme_comparisons)]
    fn resize(&mut self, n: Size) {
        if n >= INVALID_INDEX {
            panic!(
                "Resize dimensions {} exceeded bounds {}-1",
                n, INVALID_INDEX
            );
        }
        self.storage.resize(n as usize);
    }
    fn clear(&mut self) {
        ::std::mem::swap(&mut self.storage, &mut <T as StorageFor>::Storage::new());
    }
    fn push(&mut self) {
        self.storage.push();
    }
    fn clone_as_trait(&self) -> Box<dyn ResizeableProperty<Handle = H>> {
        Box::new(self.clone())
    }
    fn as_property(&self) -> &dyn Property<Handle = H> {
        self
    }
    fn as_property_mut(&mut self) -> &mut dyn Property<Handle = H> {
        self
    }
}

impl<T, H> ConstructableProperty for PropertyList<T, H>
where
    T: Value,
    H: ItemHandle,
{
    fn new(name: String, size: Size) -> Self {
        let mut prop = PropertyList {
            name,
            persistent: false,
            storage: <T as StorageFor>::Storage::new(),
            _m: ::std::marker::PhantomData,
        };
        prop.resize(size);
        prop
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use crate::property::{ConstructableProperty, ItemHandle, PropertyList, Value};

    fn _assert_any<P: ::std::any::Any>(_p: P) {}

    // This method isn't used anywhere. Instead, it serves as a compile-time assertion that the
    // constraints `T: Value` and `H: ItemHandle` imply `PropertyList: ::std::any::Any`.
    // Test compilation will fail here if this fact is violated.
    fn _assert_property_any<T: Value, H: ItemHandle>() {
        _assert_any(PropertyList::<T, H>::new("test".into(), 10));
    }
}
