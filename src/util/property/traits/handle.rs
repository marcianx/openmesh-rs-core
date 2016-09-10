use util::property::size::{Index, INVALID_INDEX};

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
        pub struct $handle($crate::util::property::size::Index);

        impl $crate::util::property::traits::Handle for $handle {
            #[inline(always)]
            fn new() -> Self {
                $handle($crate::util::property::size::INVALID_INDEX)
            }
            #[inline(always)]
            fn from_index(idx: $crate::util::property::size::Index) -> Self {
                assert!(idx != $crate::util::property::size::INVALID_INDEX);
                $handle(idx)
            }
            #[inline(always)]
            fn index(self) -> $crate::util::property::size::Index { self.0 }
            #[inline(always)]
            fn set_index(&mut self, idx: $crate::util::property::size::Index) { self.0 = idx; }
        }

        impl ::std::fmt::Display for $handle {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.0.fmt(formatter)
            }
        }
    }
}

/// Property handle - one that is tied to a `Property` enumerating a specific `Value` type.
pub trait PropHandle: Handle {
    type Value;
}

/// Defines a handle implementing `Handle` `PropHandle` with `Value = T` via
///  `def_prop_handle!(Handle<T>)`
/// `T` must be a generic type. This macro currently also requires `T: std::any::Any` on the
/// definitions.
#[macro_export]
macro_rules! def_prop_handle {
    ($handle:ident < $arg:ident >) => {
        #[derive(Hash)]
        pub struct $handle<$arg: ::std::any::Any>($crate::util::property::size::Index, ::std::marker::PhantomData<$arg>);
        impl<$arg: ::std::any::Any> Copy for $handle<$arg> {}
        impl<$arg: ::std::any::Any> Clone for $handle<$arg> { fn clone(&self) -> Self { *self } }

        impl<$arg: ::std::any::Any> PartialEq for $handle<$arg> {
            fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
        }
        impl<$arg: ::std::any::Any> Eq for $handle<$arg> {}

        impl<$arg: ::std::any::Any> $crate::util::property::traits::Handle for $handle<$arg> {
            #[inline(always)]
            fn new() -> Self {
                $handle($crate::util::property::size::INVALID_INDEX, ::std::marker::PhantomData::<$arg>)
            }
            #[inline(always)]
            fn from_index(idx: $crate::util::property::size::Index) -> Self {
                assert!(idx != $crate::util::property::size::INVALID_INDEX);
                $handle(idx, ::std::marker::PhantomData::<$arg>)
            }
            #[inline(always)]
            fn index(self) -> $crate::util::property::size::Index { self.0 }
            #[inline(always)]
            fn set_index(&mut self, idx: $crate::util::property::size::Index) { self.0 = idx; }
        }

        impl<$arg: ::std::any::Any> ::std::fmt::Debug for $handle<$arg> {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl<$arg: ::std::any::Any> ::std::fmt::Display for $handle<$arg> {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl<$arg: ::std::any::Any> $crate::util::property::traits::PropHandle for $handle<$arg> {
            type Value = $arg;
        }
    };
}

#[cfg(test)]
mod test {
    def_handle!(MyHandle1);
    def_prop_handle!(MyHandle3<T>);
}
