//! Mesh item handle types.

use mesh::mesh::Mesh;
use mesh::items::{Vertex, Halfedge, Edge, Face};
use mesh::status::Status;
use property::{Property, PropertyContainer};
use property::size::Size;
use property::traits::{self, Handle, PropertyFor}; // import methods of Handle

def_handle!(VertexHandle, "Vertex handle.");
def_handle!(HalfedgeHandle, "Halfedge handle.");
def_handle!(EdgeHandle, "Edge handle.");
def_handle!(FaceHandle, "Face handle.");
def_handle!(MeshHandle, "Mesh handle (only needed for parametrizing PropertyContainer).");

// Define the marker trait (used to define the property containers).
impl traits::ItemHandle for VertexHandle {}
impl traits::ItemHandle for HalfedgeHandle {}
impl traits::ItemHandle for EdgeHandle {}
impl traits::ItemHandle for FaceHandle {}
impl traits::ItemHandle for MeshHandle {}


/// Relates each `ItemHandle` type to corresponding structures within the Mesh itself.
/// The methods and types within this trait **are implementation details** and should not be used
/// outside of this framework.
pub trait MeshItemHandle: traits::ItemHandle {
    /// Mesh item type corresponding to `Self` which is one of `Vertex`, `Halfedge`, `Edge`, or
    /// `Face`.
    type Item: Clone + Default;

    /// Storage item type containing `Self::Item`. Specifically, `Vertex`/`Edge`/`Face` is stored
    /// as itself, but each `Halfedge` is stored in an `Edge`.
    type ContainerItem: Clone + Default;

    // Mesh property lists.

    /// Default property name prefix.
    const PREFIX: &'static str;

    /// Prepends a name with the canonical prefix for this item type.
    fn with_prefix(name: &str) -> String { format!("{}{}", Self::PREFIX, name) }

    /// Gets container underlying the mesh item type out of the mesh.
    fn items_props(m: &Mesh) -> (&Vec<Self::ContainerItem>, &PropertyContainer<Self>);

    /// Gets container underlying the mesh item type out of the mesh mutably.
    fn items_props_mut(m: &mut Mesh) -> (&mut Vec<Self::ContainerItem>, &mut PropertyContainer<Self>);

    // Mesh items.

    /// Number of items of type `Self` in the underlying storage vector.
    fn num_items(vec: &Vec<Self::ContainerItem>) -> usize;

    /// Gets the number of items of the given type.
    fn len(mesh: &Mesh) -> Size {
        Self::num_items(Self::items_props(mesh).0) as Size
    }

    /// Gets item of type `Self` from the underlying storage vector.
    fn get(vec: &Vec<Self::ContainerItem>, handle: Self) -> Option<&Self::Item>;

    /// Gets item of type `Self` mutably from the underlying storage vector.
    fn get_mut(vec: &mut Vec<Self::ContainerItem>, handle: Self) -> Option<&mut Self::Item>;

    // Mesh iteration.

    /// Gets the status property.
    fn status_prop(mesh: &Mesh) -> Option<&Property<Status, Self>>;
}

macro_rules! impl_to_items {
    ($Item:ty, $ContainerItem:ty, $Handle:ty, $prefix:expr, $item_field:ident, $prop_field:ident,
     $get_status:ident,
     ($vec:ident, $handle:ident) -> {
         fn num_items: $num_items:expr,
         fn get: $get:expr,
         fn get_mut: $get_mut:expr,
     }) => {
        impl MeshItemHandle for $Handle {
            type Item = $Item;
            type ContainerItem = $ContainerItem;
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

            fn num_items($vec: &Vec<Self::ContainerItem>) -> usize {
                $num_items
            }

            fn get($vec: &Vec<Self::ContainerItem>, $handle: Self) -> Option<&Self::Item> {
                $get
            }

            fn get_mut($vec: &mut Vec<Self::ContainerItem>, $handle: Self) -> Option<&mut Self::Item> {
                $get_mut
            }

            fn status_prop(mesh: &Mesh) -> Option<&Property<Status, Self>> {
                mesh.$get_status()
            }
        }
    };

    ($Item:ty, $ContainerItem:ty, $Handle:ty, $prefix:expr, $item_field:ident, $prop_field:ident,
     $get_status:ident) => {
        impl_to_items!(
            $Item, $ContainerItem, $Handle, $prefix, $item_field, $prop_field, $get_status,
            (vec, handle) -> {
                fn num_items: vec.len(),
                fn get:       vec.get(handle.index_us()),
                fn get_mut:   vec.get_mut(handle.index_us()),
            }
        );
    };
}


// TODO: This is required for some incomprehensible reason for `status_prop()` above to work.
// Figure out why and whether there's a bug in the compiler's (unstable) associated type support.
impl<H> PropertyFor<H> for Status
    where H: traits::ItemHandle
{
    type Property = Property<Status, H>;
}


impl_to_items!(  Vertex, Vertex,   VertexHandle, "v:", vertices, v_props, get_vertex_status);
impl_to_items!(    Edge,   Edge,     EdgeHandle, "e:",    edges, e_props, get_edge_status);
impl_to_items!(    Face,   Face,     FaceHandle, "f:",    faces, f_props, get_face_status);
impl_to_items!(Halfedge,   Edge, HalfedgeHandle, "h:",    edges, h_props, get_halfedge_status,
               // Halfedges are stored within edges.
               (vec, handle) -> {
                   fn num_items: {
                       debug_assert!(vec.len() <= usize::max_value() / 2);
                       vec.len() * 2
                   },
                   fn get: {
                       let index = handle.index_us();
                       vec.get(index / 2).map(|edge| &edge.0[index % 2])
                   },
                   fn get_mut: {
                       let index = handle.index_us();
                       vec.get_mut(index / 2).map(|edge| &mut edge.0[index % 2])
                   },
               });
