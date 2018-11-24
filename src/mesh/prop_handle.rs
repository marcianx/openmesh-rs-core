//! Handles to a property lists for an item type.

use std::any::Any;
use std::marker::PhantomData;
use mesh::item_handle::{VertexHandle, HalfedgeHandle, EdgeHandle, FaceHandle, MeshHandle};
use property::BasePropHandle;
use property::traits;

/// Mesh property handle, parametrized by mesh item handle type (handles to vertex, halfedge,
/// edge, face, mesh), and the property item type `T`.
#[derive(Eq, Hash)]
pub struct PropHandle<H, T>(BasePropHandle<T>, PhantomData<H>);

impl<H, T> Copy for PropHandle<H, T> {}
impl<H, T> Clone for PropHandle<H, T> { fn clone(&self) -> Self { *self } }

impl<H, T> PartialEq for PropHandle<H, T> {
    fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
}
impl<H, T> Eq for PropHandle<H, T> {}

impl<H, T> ::std::fmt::Debug for PropHandle<H, T> {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl<H, T> ::std::fmt::Display for PropHandle<H, T> {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl<H: traits::ItemHandle, T: Any> Default for PropHandle<H, T> {
    fn default() -> Self {
        <Self as traits::PropHandle<T>>::from_base(traits::Handle::new())
    }
}

impl<H: traits::ItemHandle, T: Any> traits::PropHandle<T> for PropHandle<H, T> {
    type Value = T;
    type Handle = H;

    fn from_base(h: BasePropHandle<T>) -> Self {
        PropHandle(h, ::std::marker::PhantomData::<H>)
    }
    fn to_base(self) -> BasePropHandle<T> { self.0 }
    fn set_base(&mut self, h: BasePropHandle<T>) { self.0 = h }
}

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

