extern crate num;
use std::fmt::{Display, Formatter};
use std::ops::Not;
use self::num::traits::Zero;

/// This is the default index type.
pub type IndexType = u32;

/// Trait for forward and backward traversal.
pub trait Ordinal {
    /// Return the successor value.
    fn succ(&self) -> Self;
    /// Returns the predecessor value.
    fn pred(&self) -> Self;
}

impl Ordinal for u32 {
    fn succ(&self) -> Self { self + 1u32 }
    fn pred(&self) -> Self { self - 1u32 }
}

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
/// BaseHandle for all index types.
pub struct Handle<Index> { idx: Index }

impl<Index: Copy + Ordinal + Eq + Not<Output=Index> + Zero> Handle<Index> {
    /// Returns the value corresponding to an invalid index.
    pub fn invalid_index() -> Index { !Index::zero() }

    /// Initialize a handle with an invalid index.
    pub fn new() -> Self { Handle { idx: Self::invalid_index() } }
    /// Construct from index.
    pub fn from_index(idx: Index) -> Self { Handle { idx: idx } }

    /// Gets the index.
    pub fn idx(&self) -> Index { self.idx }
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
impl<Index: Display> Display for Handle<Index> {
    fn fmt(&self, formatter: &mut Formatter) -> ::std::fmt::Result { self.idx.fmt(formatter) }
}

/// Handle provider trait for supporting distinct types to wrap Handle.
pub trait HandleProvider<Index> {
    /// Returns the underlying handle.
    fn get_handle(&self) -> &Handle<Index>;
}

impl<Index> HandleProvider<Index> for Handle<Index> {
    fn get_handle(&self) -> &Handle<Index> { &self }
}

#[macro_export]
macro_rules! def_index {
    ($handle: ident) => {
        #[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
        pub struct $handle<Index>($crate::util::property::Handle<Index>);
        impl<Index> $crate::util::property::HandleProvider<Index> for $handle<Index> {
            fn get_handle(&self) -> &$crate::util::property::Handle<Index> { &self.0 }
        }
        impl<Index: ::std::fmt::Display> ::std::fmt::Display for $handle<Index> {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.0.fmt(formatter)
            }
        }
    }
}

