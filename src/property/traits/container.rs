use property::{BasePropHandle, Property};
use property::size;
use property::traits;

// TODO: Should there be a `RefCell` here to allow getting multiple properties mutably?
pub type Properties<H> = Vec<Option<Box<traits::Property<H>>>>;

pub trait PropertyContainer<H: traits::Handle>
{
    ////////////////////////////////////////////////////////////////////////////////
    // Addition/getting/removal of properties.

    /// Returns the length of the `Property`-holder. The number of stored properties does not
    /// exceed this length.
    fn len(&self) -> size::Size;
    /// The underlying property vector.
    fn vec(&self) -> &Properties<H>;
    /// Adds a property whose elements are of type `T`.
    /// Panics in the unlikely case that the number of properties reaches `size::INVALID_INDEX`.
    fn add<T>(&mut self, name: Option<String>) -> BasePropHandle
        where T: traits::Value;
    /// Returns the property at the given handle if any exists and if the return type matches.
    fn get<T>(&mut self, prop_handle: BasePropHandle) -> Option<&Property<T, H>>
        where T: traits::Value;
    /// Returns the property at the given handle if any exists and if the return type matches.
    fn get_mut<T>(&mut self, prop_handle: BasePropHandle) -> Option<&mut Property<T, H>>
        where T: traits::Value;
    /// Returns the property at the given handle if any exists and if the return type matches.
    fn remove(&mut self, prop_handle: BasePropHandle);
    /// Removes all properties.
    fn clear(&mut self);

    ////////////////////////////////////////////////////////////////////////////////
    // Collectively managing active property lists.

    /// Clears the contents of each active property list.
    fn clear_all(&mut self);

    /// Reserves space for `n` items in each active property list.
    fn reserve_all(&mut self, n: size::Size);

    /// Resizes each active property list.
    fn resize_all(&mut self, n: size::Size);

    /// Swaps a pair of items in each active property list.
    /// TODO: Return an error if the indices were out of bounds.
    fn swap_all(&mut self, i0: H, i1: H);

    // TODO: Add methods for bit-vectors (`PropertyBits`) as well.
}

