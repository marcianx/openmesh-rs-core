//! Handle trait for mesh item and item property handles.

use property::size::{Index, INVALID_INDEX};

/// Trait for handle types that wrap `HandleBase` which wraps an index.
/// The `Default` implementation must initialize the handle to an invalid index.
pub trait Handle: ::std::any::Any + Copy + Clone + Default + ::std::fmt::Debug + Eq + 'static
{
    /// Initialize a handle with an invalid index.
    fn new() -> Self { Default::default() }
    /// Construct from index.
    fn from_index(idx: Index) -> Self;

    /// Gets the index.
    fn index(self) -> Index;

    /// Sets the index.
    fn set_index(&mut self, idx: Index);

    // Automatic implementations.

    /// Gets the index as a usize for indexing into standard subcontainer.
    #[inline(always)]
    fn index_us(self) -> usize { self.index() as usize }

    /// Whether the handle is valid.
    #[inline(always)]
    fn is_valid(self) -> bool { self.index() != INVALID_INDEX }

    /// Invalidates the underlying index.
    #[inline(always)]
    fn invalidate(&mut self) { self.set_index(INVALID_INDEX); }

    /// Increments the handle's underlying index.
    ///
    /// It should be used only by iterators which must ensure that it is not
    /// called if there is a danger of wrapping.
    #[inline(always)]
    fn __increment(&mut self) {
        let index = self.index() + (1 as Index);
        self.set_index(index);
    }

    /// Decrements the handle's underlying index, rolling over from 0 to
    /// `Index::max_size()`, which is `INVALID_INDEX`.
    ///
    /// It should only be used by iterators, which may rely on this roll-over
    /// behavior.
    #[inline(always)]
    fn __decrement(&mut self) {
        let index = self.index().wrapping_sub(1 as Index);
        self.set_index(index);
    }

    /// Converts the handle to `Some(self)` if valid, else `None`.
    #[inline(always)]
    fn to_option(self) -> Option<Self> {
        if self.is_valid() { Some(self) } else { None }
    }
}

#[macro_export]
macro_rules! def_handle {
    ($handle:ident, $doc:expr) => {
        #[doc=$doc]
        #[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
        pub struct $handle($crate::property::size::Index);

        impl ::std::default::Default for $handle {
            fn default() -> Self {
                $handle($crate::property::size::INVALID_INDEX)
            }
        }

        impl $crate::property::traits::Handle for $handle {
            #[inline(always)]
            fn from_index(idx: $crate::property::size::Index) -> Self {
                assert!(idx != $crate::property::size::INVALID_INDEX);
                $handle(idx)
            }
            #[inline(always)]
            fn index(self) -> $crate::property::size::Index { self.0 }
            #[inline(always)]
            fn set_index(&mut self, idx: $crate::property::size::Index) { self.0 = idx; }
        }

        impl ::std::fmt::Display for $handle {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.0.fmt(formatter)
            }
        }
    }
}


def_handle!(
    BasePropHandle,
    "`PropertyContainer` handle to a contained `Property` (which is a list of `Value`s).");

/// Handle to a `Property` within a `PropertyContainer`. Each `Property` represents a list of items
/// of type `Value`. The `Default` implementation must initialize the handle to an invalid index.
pub trait PropHandle: Copy + Default + Eq {
    /// The value type stored in the property list into which `self` is a handle.
    type Value;
    /// Handle type corresponding to the mesh item type (vertex, halfedge, edge, face, mesh) for
    /// which `Value` is being stored.
    type Handle: Handle;

    /// Create an invalidated handle.
    fn new() -> Self { Default::default() }
    /// Create from `BasePropHandle`.
    fn from_base(h: BasePropHandle) -> Self;
    /// Get `BasePropHandle` form.
    fn to_base(self) -> BasePropHandle;
    /// Set the handle from the given `BasePropHandle`.
    fn set_base(&mut self, h: BasePropHandle);

    /// Whether the handle is valid.
    #[inline(always)]
    fn is_valid(self) -> bool { self.to_base().index() != INVALID_INDEX }
    /// Invalidates the handle.
    #[inline(always)]
    fn invalidate(&mut self) { self.set_base(BasePropHandle::new()); }
    /// Converts the handle to `Some(self)` if valid, else `None`.
    #[inline(always)]
    fn to_option(self) -> Option<Self> {
        if self.is_valid() { Some(self) } else { None }
    }
}

#[cfg(test)]
mod test {
    def_handle!(MyHandle, "Test Handle Trait.");
}
