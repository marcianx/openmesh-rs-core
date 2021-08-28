//! Handles for mesh item and item property handles.
use crate::mesh::item_handle::{EdgeHandle, FaceHandle, HalfedgeHandle, MeshHandle, VertexHandle};
use crate::property::{Index, INVALID_INDEX};

/// Trait for handle types that wrap `HandleBase` which wraps an index.
/// The `Default` implementation must initialize the handle to an invalid index.
pub trait Handle:
    ::std::any::Any + Copy + Clone + Default + ::std::fmt::Debug + Eq + 'static
{
    /// Initialize a handle with an invalid index.
    fn new() -> Self {
        Default::default()
    }

    /// Construct from index.
    fn from_index(idx: Index) -> Self;

    /// Gets the index.
    fn index(self) -> Index;

    /// Sets the index.
    fn set_index(&mut self, idx: Index);

    // Automatic implementations.

    /// Gets the index as a usize for indexing into standard subcontainer.
    #[inline(always)]
    fn index_us(self) -> usize {
        self.index() as usize
    }

    /// Whether the handle is valid.
    #[inline(always)]
    fn is_valid(self) -> bool {
        self.index() != INVALID_INDEX
    }

    /// Invalidates the underlying index.
    #[inline(always)]
    fn invalidate(&mut self) {
        self.set_index(INVALID_INDEX);
    }

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
        if self.is_valid() {
            Some(self)
        } else {
            None
        }
    }
}

#[macro_export]
/// Usage examples (`S` and `T` below must be generic.
/// - `def_handle!(MyHandle, "Documentation for `MyHandle`");`
/// - `def_handle!(MyHandleT<S, T>, "Documentation for `MyHandleT<S, T>`");`
/// Note: This adds the `::std::any::Any` trait constraint on all the type parameters for
/// implementing `property::Handle`.
macro_rules! def_handle {
    (@def $Handle:ident ( $($Types:ident),* ), $doc:expr) => {
        #[doc=$doc]
        pub struct $Handle<$($Types),*>(
            $crate::property::Index,
            ::std::marker::PhantomData<($($Types),*)>);

        impl<$($Types),*> ::std::default::Default for $Handle<$($Types),*> {
            fn default() -> Self {
                $Handle($crate::property::INVALID_INDEX, ::std::marker::PhantomData::<_>)
            }
        }

        impl<$($Types: ::std::any::Any),*> $crate::property::Handle for $Handle<$($Types),*> {
            #[inline(always)]
            fn from_index(idx: $crate::property::Index) -> Self {
                assert!(idx != $crate::property::INVALID_INDEX);
                $Handle(idx, ::std::marker::PhantomData::<_>)
            }
            #[inline(always)]
            fn index(self) -> $crate::property::Index { self.0 }
            #[inline(always)]
            fn set_index(&mut self, idx: $crate::property::Index) { self.0 = idx; }
        }

        // Because of the type parameters, these cannot be auto-derived.
        impl<$($Types),*> Copy for $Handle<$($Types),*> {}

        impl<$($Types),*> Clone for $Handle<$($Types),*> { fn clone(&self) -> Self { *self } }

        impl<$($Types),*> PartialEq for $Handle<$($Types),*> {
            fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
        }

        impl<$($Types),*> Eq for $Handle<$($Types),*> {}

        impl<$($Types),*> ::std::fmt::Debug for $Handle<$($Types),*> {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                stringify!($Handle).fmt(fmt)?;
                "(".fmt(fmt)?;
                self.0.fmt(fmt)?;
                ")".fmt(fmt)
            }
        }

        impl<$($Types),*> ::std::hash::Hash for $Handle<$($Types),*> {
            fn hash<HH>(&self, state: &mut HH) where HH: ::std::hash::Hasher { self.0.hash(state) }
        }

        impl<$($Types),*> ::std::fmt::Display for $Handle<$($Types),*> {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                self.0.fmt(formatter)
            }
        }
    };

    ($Handle:ident < $($Types:ident),* >, $doc:expr) => {
        def_handle!(@def $Handle ( $($Types),* ), $doc);
    };

    ($Handle:ident, $doc:expr) => { def_handle!(@def $Handle (), $doc); };
}

/// Marker trait for handles to mesh items (vertex, halfedge, edge, face).
pub trait ItemHandle: Handle {}

def_handle!(
    PropHandle<H, T>,
    "Mesh property handle, parametrized by mesh item handle type (handles to vertex, halfedge, \
     edge, face, mesh), and the property item type `T`");

/// Handle for a specific vertex property.
pub type VPropHandle<T> = PropHandle<VertexHandle, T>;

/// Handle for a specific halfedge property.
pub type HPropHandle<T> = PropHandle<HalfedgeHandle, T>;

/// Handle for a specific edge property.
pub type EPropHandle<T> = PropHandle<EdgeHandle, T>;

/// Handle for a specific face property.
pub type FPropHandle<T> = PropHandle<FaceHandle, T>;

/// Handle for a specific mesh property.
pub type MPropHandle<T> = PropHandle<MeshHandle, T>;

#[cfg(test)]
mod test {
    def_handle!(MyHandle, "Test Handle Trait.");
    def_handle!(MyHandleT<T>, "Test HandleT<T> Trait.");
    def_handle!(MyHandleST<S, T>, "Test HandleST<S, T> Trait.");
}
