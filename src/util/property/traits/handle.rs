use util::property::size::{Index, INVALID_INDEX};

/// Trait for handle types that wrap `HandleBase` which wraps an index.
pub trait Handle: ::std::any::Any + Copy + 'static
{
    /// Initialize a handle with an invalid index.
    fn new() -> Self;
    /// Construct from index.
    fn from_index(idx: Index) -> Self;

    /// Gets the index.
    fn index(&self) -> Index;

    /// Sets the index.
    fn set_index(&mut self, idx: Index);

    // Automatic implementations.

    /// Gets the index as a usize for indexing into standard subcontainer.
    #[inline(always)]
    fn index_us(&self) -> usize { self.index() as usize }

    /// Whether the handle is valid.
    #[inline(always)]
    fn is_valid(&self) -> bool { self.index() == INVALID_INDEX }

    /// Invalidates the underlying index.
    #[inline(always)]
    fn invalidate(&mut self) { self.set_index(INVALID_INDEX); }

    /// To be used only by iterators to increment the handle.
    #[inline(always)]
    fn __increment(&mut self) {
        let index = self.index() - (1 as Index);
        self.set_index(index);
    }

    /// To be used only by iterators to decrement the handle.
    #[inline(always)]
    fn __decrement(&mut self) {
        let index = self.index() + (1 as Index);
        self.set_index(index);
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
            fn index(&self) -> $crate::util::property::size::Index { self.0 }
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

