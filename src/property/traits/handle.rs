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
/// Usage examples (`S` and `T` below must be generic.
/// - `def_handle!(MyHandle, "Documentation for `MyHandle`");`
/// - `def_handle!(MyHandleT<S, T>, "Documentation for `MyHandleT<S, T>`");`
/// Note: This adds the `::std::any::Any` trait constraint on all the type parameters for
/// implementing `property::traits::Handle`.
macro_rules! def_handle {
    (@def $Handle:ident ( $($Types:ident),* ), $doc:expr) => {
        #[doc=$doc]
        pub struct $Handle<$($Types),*>(
            $crate::property::size::Index,
            ::std::marker::PhantomData<($($Types),*)>);

        impl<$($Types),*> ::std::default::Default for $Handle<$($Types),*> {
            fn default() -> Self {
                $Handle($crate::property::size::INVALID_INDEX, ::std::marker::PhantomData::<_>)
            }
        }

        impl<$($Types: ::std::any::Any),*> $crate::property::traits::Handle for $Handle<$($Types),*> {
            #[inline(always)]
            fn from_index(idx: $crate::property::size::Index) -> Self {
                assert!(idx != $crate::property::size::INVALID_INDEX);
                $Handle(idx, ::std::marker::PhantomData::<_>)
            }
            #[inline(always)]
            fn index(self) -> $crate::property::size::Index { self.0 }
            #[inline(always)]
            fn set_index(&mut self, idx: $crate::property::size::Index) { self.0 = idx; }
        }

        // Because of the type parameters, these cannot be auto-derived.
        impl<$($Types),*> Copy for $Handle<$($Types),*> {}

        impl<$($Types),*> Clone for $Handle<$($Types),*> { fn clone(&self) -> Self { *self } }

        impl<$($Types),*> PartialEq for $Handle<$($Types),*> {
            fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
        }

        impl<$($Types),*> Eq for $Handle<$($Types),*> {}

        impl<$($Types),*> ::std::fmt::Debug for $Handle<$($Types),*> {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                stringify!($Handle).fmt(fmt)?;
                "(".fmt(fmt)?;
                self.0.fmt(fmt)?;
                ")".fmt(fmt)
            }
        }

        impl<$($Types),*> ::std::hash::Hash for $Handle<$($Types),*> {
            fn hash<H>(&self, state: &mut H) where H: ::std::hash::Hasher { self.0.hash(state) }
        }

        impl<$($Types),*> ::std::fmt::Display for $Handle<$($Types),*> {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.0.fmt(formatter)
            }
        }
    };

    ($Handle:ident < $($Types:ident),* >, $doc:expr) => {
        def_handle!(@def $Handle ( $($Types),* ), $doc);
    };

    ($Handle:ident, $doc:expr) => { def_handle!(@def $Handle (), $doc); };
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
    def_handle!(MyHandleT<T>, "Test HandleT<T> Trait.");
    def_handle!(MyHandleST<S, T>, "Test HandleST<S, T> Trait.");
}
