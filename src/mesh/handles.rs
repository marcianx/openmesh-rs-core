//! Mesh item and item property handle types.

use std::any::Any;
use std::marker::PhantomData;
use property::BasePropHandle;
use property::traits;

////////////////////////////////////////////////////////////////////////////////
// Handle to an item.

def_handle!(VertexHandle, "Vertex handle.");
def_handle!(HalfedgeHandle, "Halfedge handle.");
def_handle!(EdgeHandle, "Edge handle.");
def_handle!(FaceHandle, "Face handle.");
def_handle!(MeshHandle, "Mesh handle (only needed for parametrizing PropertyContainer).");

////////////////////////////////////////////////////////////////////////////////
// Handle to a property lists for an item type.

/// Mesh property handle, parametrized byto mesh item handle type (handles to vertex, halfedge,
/// edge, face, mesh), and the property item type `T`.
#[derive(Eq, Hash)]
pub struct PropHandle<H, T>(BasePropHandle, PhantomData<H>, PhantomData<T>);

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

impl<H: traits::Handle, T: Any> Default for PropHandle<H, T> {
    fn default() -> Self {
        <Self as traits::PropHandle>::from_base(traits::Handle::new())
    }
}

impl<H: traits::Handle, T: Any> traits::PropHandle for PropHandle<H, T> {
    type Value = T;
    type Handle = H;

    fn from_base(h: BasePropHandle) -> Self {
        PropHandle(h, ::std::marker::PhantomData::<H>, ::std::marker::PhantomData::<T>)
    }
    fn to_base(self) -> BasePropHandle { self.0 }
    fn set_base(&mut self, h: BasePropHandle) { self.0 = h }
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

