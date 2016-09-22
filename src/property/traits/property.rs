use std::io::{Read, Write};

use downcast_rs::Downcast;

use io::binary::{Endian, UNKNOWN_SIZE};
use io::result::Result;
use property::size::Size;
use property::traits::Handle;

/// All mesh types are stored in Properties which implement this trait. We distinguish between
/// standard properties, which can be defined at compile time using the Attributes in the traits
/// definition and at runtime using the request property functions defined in one of the kernels.
///
/// If the property should be stored along with the default properties in the OM-format one must
/// name the property and enable the persistant flag with set_persistent().
pub trait Property<H: Handle>: Downcast + ::std::fmt::Debug
{
    ////////////////////////////////////////////////////////////////////////////////
    // synchronized array interface

    /// Swaps two elements.
    fn swap(&mut self, i0: H, i1: H);

    /// Copy one element from index `i_src` to index `i_dst`.
    fn copy(&mut self, i_src: H, i_dst: H);

    ////////////////////////////////////////////////////////////////////////////////
    // named property interface

    /// The name of the property.
    fn name(&self) -> &str;

    ////////////////////////////////////////////////////////////////////////////////
    // I/O support

    /// Whether this object property should be persisted.
    fn persistent(&self) -> bool;

    /// Enables or disables persistency. `self` must be a named property to enable persistency.
    fn set_persistent(&mut self, enable: bool);

    /// Number of elements in property
    fn n_elements(&self) -> usize;

    /// Size of one element in bytes or `openmesh::io::binary::UNKNOWN_SIZE` if not known.
    fn element_size(&self) -> usize;

    /// Size of property in bytes.
    fn size_of(&self) -> usize { self.size_of_len(self.n_elements()) }

    /// Size of property if it has `n_elem` elements, or `openmesh::io::binary::UNKNOWN_SIZE`
    /// if the size cannot be estimated.
    fn size_of_len(&self, n_elem: usize) -> usize {
        if self.element_size() != UNKNOWN_SIZE {
            n_elem * self.element_size()
        } else {
            UNKNOWN_SIZE
        }
    }

    /// Store self as one binary block.
    fn store(&self, writer: &mut Write, endian: Endian) -> Result<usize>;

    /// Restore self from a binary block. Uses reserve() to set the size of self before restoring.
    fn restore(&mut self, reader: &mut Read, endian: Endian) -> Result<usize>;
}


/// Trait for methods to be used only by `PropertyContainer` since it keeps all its comprising
/// elements equally-sized.
pub trait ResizeableProperty<H: Handle>: Property<H> {
    /// A deep copy of `self` as a trait object. Used to implement the `Clone` trait.
    fn clone_as_trait(&self) -> Box<ResizeableProperty<H>>;

    /// Reserve memory for `n` elements.
    /// Panics if `n >= property::size::INVALID_INDEX`.
    ///
    /// NOTE that this is different from rust standard library (eg `Vec`) where reserve takes the
    /// additional number of items that can be added before reallocation is necessary.
    fn reserve(&mut self, n: Size);

    /// Resize storage to hold `n` elements.
    fn resize(&mut self, n: Size);

    /// Clear all elements and free memory.
    fn clear(&mut self);

    /// Extend the number of elements by one.
    fn push(&mut self);

    /// Convert to a mutable `Property` trait object.
    fn as_property(&self) -> &Property<H>;

    /// Convert to a mutable `Property` trait object.
    fn as_property_mut(&mut self) -> &mut Property<H>;
}


// Support down-casting from `Property` to a struct implementing it.
impl_downcast!(Property<H> where H: Handle);


impl<H: Handle> Clone for Box<ResizeableProperty<H>>
{
    fn clone(&self) -> Self {
        self.clone_as_trait()
    }
}
