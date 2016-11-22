use std::io::{Read, Write};

use downcast_rs::Downcast;

use io::binary::{Endian, UNKNOWN_SIZE};
use io::result::Result;
use property::size::Size;
use property::traits;

/// All mesh types are stored in Properties which implement this trait. We distinguish between
/// standard properties, which can be defined at compile time using the Attributes in the traits
/// definition and at runtime using the request property functions defined in one of the kernels.
///
/// If the property should be stored along with the default properties in the OM-format one must
/// name the property and enable the persistant flag with set_persistent().
pub trait Property: Downcast + ::std::fmt::Debug
{
    type Handle: traits::Handle;

    ////////////////////////////////////////////////////////////////////////////////
    // synchronized array interface

    /// Swaps two elements.
    fn swap(&mut self, i0: Self::Handle, i1: Self::Handle);

    /// Copy one element from index `i_src` to index `i_dst`.
    fn copy(&mut self, i_src: Self::Handle, i_dst: Self::Handle);

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
/// elements equally-sized. Excludes methods that would allow `ResizeableProperty` from being used
/// as a trait object.
pub trait ResizeableProperty: Property {
    /// A deep copy of `self` as a trait object. Used to implement the `Clone` trait.
    fn clone_as_trait(&self) -> Box<ResizeableProperty<Handle=Self::Handle>>;

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
    fn as_property(&self) -> &Property<Handle=Self::Handle>;

    /// Convert to a mutable `Property` trait object.
    fn as_property_mut(&mut self) -> &mut Property<Handle=Self::Handle>;
}


/// Includes `ResizeableProperty` and methods that are disallowed for trait objects.
pub trait ConstructableProperty: ResizeableProperty {
    /// Instantiate a property with the given `name` of length `size`.
    fn new(name: String, size: Size) -> Self;
}


/// Allows picking the optimal property container for implemented type.
pub trait PropertyFor<H: traits::Handle> {
    type Property: ConstructableProperty<Handle=H>;
}


// Support down-casting from `Property` to a struct implementing it.
impl_downcast!(Property assoc Handle where Handle: traits::Handle);


impl<H> Clone for Box<ResizeableProperty<Handle=H>>
{
    fn clone(&self) -> Self {
        self.clone_as_trait()
    }
}
