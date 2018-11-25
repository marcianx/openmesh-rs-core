//! Handles to a property lists for an item type.

use mesh::item_handle::{VertexHandle, HalfedgeHandle, EdgeHandle, FaceHandle, MeshHandle};

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


