//! Crate-local helpers to relate handles to mesh items (Vertex, Halfedge, Edge, Face).

use mesh::handles::{VertexHandle, HalfedgeHandle, EdgeHandle, FaceHandle};
use mesh::items::MeshMeta;
use mesh::mesh::Mesh;
use property::PropertyContainer;
use property::traits;

/// For getting the right mesh item vectors and properties based on the handle type.
/// This is useful for implementing helper structs parametrized by item.
pub(crate) trait ItemHandleMeta: traits::ItemHandle + MeshMeta { // explicit `traits::ItemHandle` for documentation
    // Default property name prefix.
    const PREFIX: &'static str;
    fn with_prefix(name: &str) -> String { format!("{}{}", Self::PREFIX, name) }
    // For getting containers underlying the mesh item types out of the mesh.
    fn items_props(m: &Mesh) -> (&Vec<Self::ContainerItem>, &PropertyContainer<Self>);
    fn items_props_mut(m: &mut Mesh) -> (&mut Vec<Self::ContainerItem>, &mut PropertyContainer<Self>);
}

macro_rules! impl_to_items {
    ($Item:ty, $Handle:ty, $item_field:ident, $prop_field:ident, $prefix:expr) => {
        impl ItemHandleMeta for $Handle {
            const PREFIX: &'static str = $prefix;
            fn items_props(m: &Mesh) ->
                (&Vec<Self::ContainerItem>, &PropertyContainer<Self>)
            {
                (&m.$item_field, &m.$prop_field)
            }
            fn items_props_mut(m: &mut Mesh) ->
                (&mut Vec<Self::ContainerItem>, &mut PropertyContainer<Self>)
            {
                (&mut m.$item_field, &mut m.$prop_field)
            }
        }
    }
}

impl_to_items!(  Vertex,   VertexHandle, vertices, v_props, &"v:");
impl_to_items!(Halfedge, HalfedgeHandle,    edges, h_props, &"h:"); // Halfedges are stored within edges.
impl_to_items!(    Edge,     EdgeHandle,    edges, e_props, &"e:");
impl_to_items!(    Face,     FaceHandle,    faces, f_props, &"f:");

