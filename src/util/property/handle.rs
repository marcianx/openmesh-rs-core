extern crate num;
use std::fmt::{Display, Formatter};

/// This is the default index type.
pub type Index = u32;

/// Trait for forward and backward traversal.
pub trait Ordinal {
    /// Return the successor value.
    fn succ(&self) -> Self;
    /// Returns the predecessor value.
    fn pred(&self) -> Self;
}

impl Ordinal for Index {
    fn succ(&self) -> Self { self + (1 as Index) }
    fn pred(&self) -> Self { self - (1 as Index) }
}

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
/// BaseHandle for all index types.
pub struct Handle { idx: Index }

impl Handle {
    /// Returns the value corresponding to an invalid index.
    pub fn invalid_index() -> Index { Index::max_value() }

    /// Initialize a handle with an invalid index.
    pub fn new() -> Self { Handle { idx: Self::invalid_index() } }
    /// Construct from index.
    pub fn from_index(idx: usize) -> Self {
        assert!(::std::mem::size_of::<Index>() <= ::std::mem::size_of::<usize>());
        // Note that `Index::max_value()` is `invalid_index()`.
        assert!(idx < Index::max_value() as usize);
        Handle { idx: idx as Index }
    }

    /// Gets the index.
    pub fn index(&self) -> Index { self.idx }
    /// Whether the handle is valid.
    pub fn is_valid(&self) -> bool { self.idx == Self::invalid_index() }
    /// Invalidates the underlying index.
    pub fn invalidate(&mut self) { self.idx = Self::invalid_index(); }

    /// To be used only by iterators to increment the handle.
    pub fn __increment(&mut self) {
        self.idx = self.idx.succ();
    }
    /// To be used only by iterators to decrement the handle.
    pub fn __decrement(&mut self) {
        self.idx = self.idx.pred();
    }
}

// Display trait implementation.
impl Display for Handle {
    fn fmt(&self, formatter: &mut Formatter) -> ::std::fmt::Result { self.idx.fmt(formatter) }
}

/// Handle provider trait for supporting distinct types to wrap Handle.
pub trait HandleProvider {
    /// Returns the underlying handle.
    fn handle(&self) -> &Handle;
}

impl HandleProvider for Handle {
    fn handle(&self) -> &Handle { &self }
}

#[macro_export]
macro_rules! def_handle {
    ($handle: ident) => {
        #[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
        pub struct $handle($crate::util::property::Handle);
        impl $crate::util::property::HandleProvider for $handle {
            fn handle(&self) -> &$crate::util::property::Handle { &self.0 }
        }
        impl ::std::fmt::Display for $handle {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.0.fmt(formatter)
            }
        }
    }
}

