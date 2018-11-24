//! Mesh item handle types.

use mesh::items::MeshMeta;
use mesh::mesh::Mesh;
use property::PropertyContainer;
use property::traits;

def_handle!(VertexHandle, "Vertex handle.");
def_handle!(HalfedgeHandle, "Halfedge handle.");
def_handle!(EdgeHandle, "Edge handle.");
def_handle!(FaceHandle, "Face handle.");
def_handle!(MeshHandle, "Mesh handle (only needed for parametrizing PropertyContainer).");

impl traits::ItemHandle for VertexHandle {}
impl traits::ItemHandle for HalfedgeHandle {}
impl traits::ItemHandle for EdgeHandle {}
impl traits::ItemHandle for FaceHandle {}
impl traits::ItemHandle for MeshHandle {}


/// Relates each `ItemHandle` type to corresponding structures within the Mesh itself.
/// The methods and types within this trait **are implementation details** and should not be used
/// outside of this framework.
pub(crate) trait MeshItemHandle: traits::ItemHandle + MeshMeta {
    // Default property name prefix.
    const PREFIX: &'static str;
    fn with_prefix(name: &str) -> String { format!("{}{}", Self::PREFIX, name) }
    // For getting containers underlying the mesh item types out of the mesh.
    fn items_props(m: &Mesh) -> (&Vec<Self::ContainerItem>, &PropertyContainer<Self>);
    fn items_props_mut(m: &mut Mesh) -> (&mut Vec<Self::ContainerItem>, &mut PropertyContainer<Self>);
}

macro_rules! impl_to_items {
    ($Item:ty, $Handle:ty, $item_field:ident, $prop_field:ident, $prefix:expr) => {
        impl MeshItemHandle for $Handle {
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

