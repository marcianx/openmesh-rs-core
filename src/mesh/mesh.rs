//! 2-manifold surface mesh represented as a halfedge data structure.

use mesh::handles::{
    VertexHandle, HalfedgeHandle, EdgeHandle, FaceHandle, MeshHandle,
};
use mesh::items::{
    Vertex, Halfedge, Edge, Face,
    MeshItem, MeshItemFor, MeshHandleFor, ContainerItem,
    ItemsWithProps, VItems, HItems, EItems, FItems,
    ItemsWithPropsMut, VItemsMut, HItemsMut, EItemsMut, FItemsMut,
};
use mesh::prop::{
    ItemProps, VProps, HProps, EProps, FProps, MProps,
    ItemPropsMut, VPropsMut, HPropsMut, EPropsMut, FPropsMut, MPropsMut,
};
use mesh::rc::{
    RcVPropHandle, RcHPropHandle, RcEPropHandle, RcFPropHandle,
};
use mesh::status::Status;
use property::PropertyContainer;
use property::size::Size;
use property::traits;

////////////////////////////////////////////////////////////////////////////////
// Module-private

/// Mesh implementation detail.
/// TODO: Make all fields `pub(crate)` and put them directly in `Mesh`, removing `_Mesh`. This
/// requires `#![feature(pub_restricted)]`.
#[derive(Clone)]
pub struct _Mesh {
    // Item connectivity and properties.
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,

    // Properties by item type.
    pub v_props: PropertyContainer<VertexHandle>,
    pub h_props: PropertyContainer<HalfedgeHandle>,
    pub e_props: PropertyContainer<EdgeHandle>,
    pub f_props: PropertyContainer<FaceHandle>,
    pub m_props: PropertyContainer<MeshHandle>,

    // Handles for mesh status. 
    pub v_status: RcVPropHandle<Status>,
    pub h_status: RcHPropHandle<Status>,
    pub e_status: RcEPropHandle<Status>,
    pub f_status: RcFPropHandle<Status>,
}

////////////////////////////////////////////////////////////
// General property accessors and methods

#[doc(hidden)]
/// For getting the right mesh item vectors and properties based on the handle type.
/// This is useful for implementing helper structs parametrized by item.
pub trait _ToItems
    where Self: traits::Handle + MeshItemFor,
{
    fn items_props(m: &_Mesh) -> (&Vec<ContainerItem<Self::Item>>, &PropertyContainer<Self>);
    fn items_props_mut(m: &mut _Mesh) -> (&mut Vec<ContainerItem<Self::Item>>, &mut PropertyContainer<Self>);
}

macro_rules! impl_to_items {
    ($Item:ty, $Handle:ty, $item_field:ident, $prop_field:ident) => {
        impl _ToItems for $Handle {
            fn items_props(m: &_Mesh) ->
                (&Vec<ContainerItem<$Item>>, &PropertyContainer<Self>)
            {
                (&m.$item_field, &m.$prop_field)
            }
            fn items_props_mut(m: &mut _Mesh) ->
                (&mut Vec<ContainerItem<$Item>>, &mut PropertyContainer<Self>)
            {
                (&mut m.$item_field, &mut m.$prop_field)
            }
        }
    }
}

impl_to_items!(  Vertex,   VertexHandle, vertices, v_props);
impl_to_items!(Halfedge, HalfedgeHandle,    edges, h_props); // Halfedges are stored within edges.
impl_to_items!(    Edge,     EdgeHandle,    edges, e_props);
impl_to_items!(    Face,     FaceHandle,    faces, f_props);

#[doc(hidden)]
/// For getting the right mesh item properties based on the handle type.
/// This is useful for implementing helper structs parametrized by handle, like `RcPropHandle`.
pub trait _ToProps where Self: traits::Handle {
    const PREFIX: &'static str;
    fn with_prefix(name: &str) -> String {
        format!("{}{}", Self::PREFIX, name)
    }
    fn len(m: &_Mesh) -> Size;
    fn props(m: &_Mesh) -> &PropertyContainer<Self>;
    fn props_mut(m: &mut _Mesh) -> &mut PropertyContainer<Self>;
}

macro_rules! impl_to_props {
    ($Handle:ty, $field:ident, $prefix:expr, $len_fn:expr) => {
        impl _ToProps for $Handle {
            const PREFIX: &'static str = $prefix;
            fn len(m: &_Mesh) -> Size { $len_fn(m) }
            fn props(m: &_Mesh) -> &PropertyContainer<Self> { &m.$field }
            fn props_mut(m: &mut _Mesh) -> &mut PropertyContainer<Self> { &mut m.$field }
        }
    }
}

impl_to_props!(  VertexHandle, v_props, &"v:", | m: &_Mesh| { m.vertices.len() as Size });
impl_to_props!(HalfedgeHandle, h_props, &"h:", | m: &_Mesh| { (m.edges.len() * 2) as Size });
impl_to_props!(    EdgeHandle, e_props, &"e:", | m: &_Mesh| { m.edges.len() as Size });
impl_to_props!(    FaceHandle, f_props, &"f:", | m: &_Mesh| { m.faces.len() as Size });
impl_to_props!(    MeshHandle, m_props, &"m:", |_m: &_Mesh| { 1 });

// Private to `mesh` module.
// These property accessor methods are generic and useful for all helper objects parametrized by
// item handle type.
impl _Mesh {
    /// Returns the property container associated with the mesh item type identified by `Handle`.
    pub fn items<Item, Handle>(&self) -> ItemsWithProps<Item, Handle>
        where Item: MeshItem + MeshHandleFor<Handle=Handle>,
              Handle: traits::Handle + MeshItemFor<Item=Item> + _ToItems,
    {
        let (items, props) = <Handle as _ToItems>::items_props(self);
        ItemsWithProps::new(items, props)
    }

    /// Returns the property container associated with the mesh item type identified by `Handle`.
    pub fn items_mut<Item, Handle>(&mut self) -> ItemsWithPropsMut<Item, Handle>
        where Item: MeshItem + MeshHandleFor<Handle=Handle>,
              Handle: traits::Handle + MeshItemFor<Item=Item> + _ToItems,
    {
        let (items, props) = <Handle as _ToItems>::items_props_mut(self);
        ItemsWithPropsMut::new(items, props)
    }

    // TODO:
    // - Remove after re-writing these in terms of the items_* methods above.
    // - Then, make a special-case version for `Mesh` properties and remove `MeshHandle`.
    /// Returns the property container associated with the mesh item type identified by `Handle`.
    pub fn props<Handle: _ToProps>(&self) -> ItemProps<Handle> {
        let len = <Handle as _ToProps>::len(self);
        ItemProps::new(<Handle as _ToProps>::props(self), len)
    }

    /// Returns the property container associated with the mesh item type identified by `Handle`.
    pub fn props_mut<Handle: _ToProps>(&mut self) -> ItemPropsMut<Handle> {
        let len = <Handle as _ToProps>::len(self);
        ItemPropsMut::new(<Handle as _ToProps>::props_mut(self), len)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Public interface

/// Halfedge data structure.
#[derive(Clone)]
pub struct Mesh(_Mesh);

////////////////////////////////////////////////////////////
// Connectivity ("Item") constructors and accessors

// Public accessor methods
macro_rules! item_accessors {
    ($Handle:ty, $method:ident, $method_mut:ident, $Struct:ident, $StructMut:ident, $item:expr) => {
        #[doc="Returns a struct to access "] #[doc=$item] #[doc=" mesh items and properties."]
        pub fn $method(&self) -> $Struct { self.0.items() }
        #[doc="Returns a struct to mutably access "] #[doc=$item] #[doc=" mesh items and properties."]
        pub fn $method_mut(&mut self) -> $StructMut { self.0.items_mut() }
    }
}

impl Mesh {
    // Property accessors
    item_accessors!(  VertexHandle,  vertices,  vertices_mut, VItems, VItemsMut,   "vertex");
    item_accessors!(HalfedgeHandle, halfedges, halfedges_mut, HItems, HItemsMut, "halfedge");
    item_accessors!(    EdgeHandle,     edges,     edges_mut, EItems, EItemsMut,     "edge");
    item_accessors!(    FaceHandle,     faces,     faces_mut, FItems, FItemsMut,     "face");
}

////////////////////////////////////////////////////////////
// General property accessors and methods

// Public accessor methods
macro_rules! prop_accessors {
    ($Handle:ty, $method:ident, $method_mut:ident, $Struct:ident, $StructMut:ident, $item:expr) => {
        #[doc="Returns a struct to access "] #[doc=$item] #[doc=" properties."]
        pub fn $method(&self) -> $Struct { self.0.props() }
        #[doc="Returns a struct to mutably access "] #[doc=$item] #[doc=" properties."]
        pub fn $method_mut(&mut self) -> $StructMut { self.0.props_mut() }
    }
}

impl Mesh {
    // Property accessors
    prop_accessors!(  VertexHandle, v_props, v_props_mut, VProps, VPropsMut,   "vertex");
    prop_accessors!(HalfedgeHandle, h_props, h_props_mut, HProps, HPropsMut, "halfedge");
    prop_accessors!(    EdgeHandle, e_props, e_props_mut, EProps, EPropsMut,     "edge");
    prop_accessors!(    FaceHandle, f_props, f_props_mut, FProps, FPropsMut,     "face");
    prop_accessors!(    MeshHandle, m_props, m_props_mut, MProps, MPropsMut,     "mesh");

    /// Struct implementing `std::fmt::Debug`, which outputs property list stats.
    pub fn prop_stats(&self) -> FormattedPropStats { FormattedPropStats(self) }
}

/// Generated by `Mesh::prop_stats()` for outputting property list stats.
pub struct FormattedPropStats<'a>(&'a Mesh);

impl<'a> ::std::fmt::Debug for FormattedPropStats<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        "vertex props:\n".fmt(f)?;
        self.0.v_props().fmt(f)?;
        "halfedge props:\n".fmt(f)?;
        self.0.h_props().fmt(f)?;
        "edge props:\n".fmt(f)?;
        self.0.e_props().fmt(f)?;
        "face props:\n".fmt(f)?;
        self.0.f_props().fmt(f)?;
        "mesh props:\n".fmt(f)?;
        self.0.m_props().fmt(f)?;
        Ok(())
    }
}


