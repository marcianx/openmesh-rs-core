extern crate num;
use std::fmt::{Display, Error, Formatter};
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
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> { self.idx.fmt(formatter) }
}

/// Handle provider trait for supporting distinct types to wrap Handle.
trait HandleProvider<Index> {
    /// Returns the underlying handle.
    fn get_handle(&self) -> &Handle<Index>;
}

impl<Index> HandleProvider<Index> for Handle<Index> {
    fn get_handle(&self) -> &Handle<Index> { &self }
}


/// Vertex handle
#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct VertexHandle<Index> { h: Handle<Index> }
impl<Index> HandleProvider<Index> for VertexHandle<Index> {
    fn get_handle(&self) -> &Handle<Index> { &self.h }
}
impl<Index: Display> Display for VertexHandle<Index> {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        self.h.fmt(formatter)
    }
}

/// Halfedge handle
#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct HalfedgeHandle<Index> { h: Handle<Index> }
impl<Index> HandleProvider<Index> for HalfedgeHandle<Index> {
    fn get_handle(&self) -> &Handle<Index> { &self.h }
}
impl<Index: Display> Display for HalfedgeHandle<Index> {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        self.h.fmt(formatter)
    }
}

/// Edge handle
#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct EdgeHandle<Index> { h: Handle<Index> }
impl<Index> HandleProvider<Index> for EdgeHandle<Index> {
    fn get_handle(&self) -> &Handle<Index> { &self.h }
}
impl<Index: Display> Display for EdgeHandle<Index> {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        self.h.fmt(formatter)
    }
}

/// Face handle
#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct FaceHandle<Index> { h: Handle<Index> }
impl<Index> HandleProvider<Index> for FaceHandle<Index> {
    fn get_handle(&self) -> &Handle<Index> { &self.h }
}
impl<Index: Display> Display for FaceHandle<Index> {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        self.h.fmt(formatter)
    }
}

