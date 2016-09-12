use property::size::{Index, INVALID_INDEX};

/// Trait for handle types that wrap `HandleBase` which wraps an index.
pub trait Handle: ::std::any::Any + Copy + Clone + ::std::fmt::Debug + 'static
{
    /// Initialize a handle with an invalid index.
    fn new() -> Self;
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
    ($handle: ident) => {
        #[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
        pub struct $handle($crate::property::size::Index);

        impl $crate::property::traits::Handle for $handle {
            #[inline(always)]
            fn new() -> Self {
                $handle($crate::property::size::INVALID_INDEX)
            }
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

/// `PropertyContainer` handle to a contained `Property` (which is a list of `Value`s).
def_handle!(BasePropHandle);

/// Handle to a `Property` within a `PropertyContainer`. Each `Property` represents a list of items
/// of type `Value`.
pub trait PropHandle: Copy {
    type Value;

    /// Create an invalidated handle.
    fn new() -> Self;
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

/// Defines a handle implementing `Handle` `PropHandle` with `Value = T` via
///  `def_prop_handle!(Handle<T>)`
/// `T` must be a generic type. This macro currently also requires `T: std::any::Any` on the
/// definitions.
#[macro_export]
macro_rules! def_prop_handle {
    ($prop_handle:ident < $arg:ident >) => {
        #[derive(Hash)]
        pub struct $prop_handle<$arg: ::std::any::Any>($crate::property::BasePropHandle, ::std::marker::PhantomData<$arg>);
        impl<$arg: ::std::any::Any> Copy for $prop_handle<$arg> {}
        impl<$arg: ::std::any::Any> Clone for $prop_handle<$arg> { fn clone(&self) -> Self { *self } }

        impl<$arg: ::std::any::Any> PartialEq for $prop_handle<$arg> {
            fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
        }
        impl<$arg: ::std::any::Any> Eq for $prop_handle<$arg> {}

        impl<$arg: ::std::any::Any> ::std::fmt::Debug for $prop_handle<$arg> {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl<$arg: ::std::any::Any> ::std::fmt::Display for $prop_handle<$arg> {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl<$arg: ::std::any::Any> $crate::property::traits::PropHandle for $prop_handle<$arg> {
            type Value = $arg;

            fn new() -> Self {
                Self::from_base($crate::property::traits::Handle::new())
            }
            fn from_base(h: $crate::property::BasePropHandle) -> Self {
                $prop_handle(h, ::std::marker::PhantomData::<$arg>)
            }
            fn to_base(self) -> $crate::property::BasePropHandle { self.0 }
            fn set_base(&mut self, h: $crate::property::BasePropHandle) { self.0 = h }
        }
    };
}

#[cfg(test)]
mod test {
    def_handle!(MyHandle1);
    def_prop_handle!(MyHandle3<T>);
}
