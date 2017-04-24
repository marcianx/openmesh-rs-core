//! 2-manifold surface mesh represented as a halfedge data structure.

use mesh::handles::{
    VertexHandle, HalfedgeHandle, EdgeHandle, FaceHandle, MeshHandle,
};
use mesh::items::{
    Vertex, Edge, Face,
    MeshMeta,
    Items, VItems, HItems, EItems, FItems,
    ItemsMut, VItemsMut, HItemsMut, EItemsMut, FItemsMut,
};
use mesh::prop::{
    Props, VProps, HProps, EProps, FProps, MProps,
    PropsMut, VPropsMut, HPropsMut, EPropsMut, FPropsMut, MPropsMut,
};
use mesh::rc::{
    RcVPropHandle, RcHPropHandle, RcEPropHandle, RcFPropHandle,
};
use mesh::status::Status;
use property::PropertyContainer;
use property::traits;

/// Halfedge data structure.
#[derive(Clone)]
pub struct Mesh {
    // Item connectivity and properties.
    pub(crate) vertices: Vec<Vertex>,
    pub(crate) edges: Vec<Edge>,
    pub(crate) faces: Vec<Face>,

    // Properties by item type.
    pub(crate) v_props: PropertyContainer<VertexHandle>,
    pub(crate) h_props: PropertyContainer<HalfedgeHandle>,
    pub(crate) e_props: PropertyContainer<EdgeHandle>,
    pub(crate) f_props: PropertyContainer<FaceHandle>,
    pub(crate) m_props: PropertyContainer<MeshHandle>,

    // Handles for mesh status. 
    pub(crate) v_status: RcVPropHandle<Status>,
    pub(crate) h_status: RcHPropHandle<Status>,
    pub(crate) e_status: RcEPropHandle<Status>,
    pub(crate) f_status: RcFPropHandle<Status>,
}

////////////////////////////////////////////////////////////////////////////////
// Module-private

////////////////////////////////////////////////////////////
// General property accessors and methods

#[doc(hidden)]
/// For getting the right mesh item vectors and properties based on the handle type.
/// This is useful for implementing helper structs parametrized by item.
pub trait _ToItems: traits::Handle + MeshMeta { // explicit `traits::Handle` for documentation
    // Default property name prefix.
    const PREFIX: &'static str;
    fn with_prefix(name: &str) -> String { format!("{}{}", Self::PREFIX, name) }
    // For getting containers underlying the mesh item types out fo the mesh.
    fn items_props(m: &Mesh) -> (&Vec<Self::ContainerItem>, &PropertyContainer<Self>);
    fn items_props_mut(m: &mut Mesh) -> (&mut Vec<Self::ContainerItem>, &mut PropertyContainer<Self>);
}

macro_rules! impl_to_items {
    ($Item:ty, $Handle:ty, $item_field:ident, $prop_field:ident, $prefix:expr) => {
        impl _ToItems for $Handle {
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

// Private to `mesh` module.
// These property accessor methods are generic and useful for all helper objects parametrized by
// item handle type.
impl Mesh {
    /// Returns the property container associated with the mesh item type identified by `Handle`.
    pub(crate) fn items<Handle: _ToItems>(&self) -> Items<Handle> {
        let (items, props) = <Handle as _ToItems>::items_props(self);
        Items::new(items, props)
    }

    /// Returns the property container associated with the mesh item type identified by `Handle`.
    pub(crate) fn items_mut<Handle: _ToItems>(&mut self) -> ItemsMut<Handle> {
        let (items, props) = <Handle as _ToItems>::items_props_mut(self);
        ItemsMut::new(items, props)
    }

    /// Returns the property container associated with the mesh item type identified by `Handle`.
    pub(crate) fn props<Handle: _ToItems>(&self) -> Props<Handle> {
        Self::items(self).into_props()
    }

    /// Returns the property container associated with the mesh item type identified by `Handle`.
    pub(crate) fn props_mut<Handle: _ToItems>(&mut self) -> PropsMut<Handle> {
        Self::items_mut(self).into_props_mut()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Public interface

////////////////////////////////////////////////////////////
// Connectivity ("Item") constructors and accessors

// Public accessor methods
macro_rules! item_accessors {
    ($Handle:ty, $method:ident, $method_mut:ident, $Struct:ident, $StructMut:ident, $item:expr) => {
        #[doc="Returns a struct to access "] #[doc=$item] #[doc=" mesh items and properties."]
        pub fn $method(&self) -> $Struct { self.items() }
        #[doc="Returns a struct to mutably access "] #[doc=$item] #[doc=" mesh items and properties."]
        pub fn $method_mut(&mut self) -> $StructMut { self.items_mut() }
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
        pub fn $method(&self) -> $Struct { self.props() }
        #[doc="Returns a struct to mutably access "] #[doc=$item] #[doc=" properties."]
        pub fn $method_mut(&mut self) -> $StructMut { self.props_mut() }
    }
}

impl Mesh {
    // Property accessors
    prop_accessors!(  VertexHandle, v_props, v_props_mut, VProps, VPropsMut,   "vertex");
    prop_accessors!(HalfedgeHandle, h_props, h_props_mut, HProps, HPropsMut, "halfedge");
    prop_accessors!(    EdgeHandle, e_props, e_props_mut, EProps, EPropsMut,     "edge");
    prop_accessors!(    FaceHandle, f_props, f_props_mut, FProps, FPropsMut,     "face");

    #[doc="Returns a struct to access mesh properties."]
    pub fn m_props(&self) -> MProps {
        Props::new(&self.m_props, 1)
    }
    #[doc="Returns a struct to mutably access mesh properties."]
    pub fn m_props_mut(&mut self) -> MPropsMut {
        PropsMut::new(&mut self.m_props, 1)
    }

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

////////////////////////////////////////////////////////////
// Connectivity object constructors and accessors

impl Mesh {
    /// ArrayKernel uses the default copy constructor and assignment operator, which means
    /// that the connectivity and all properties are copied, including reference
    /// counters, allocated bit status masks, etc.. In contrast assign_connectivity
    /// copies only the connectivity, i.e. vertices, edges, faces and their status fields.
    /// NOTE: The geometry (the points property) is NOT copied. Poly/TriConnectivity
    /// override(and hide) that function to provide connectivity consistence.
    pub fn assign_connectivity<M>(&mut self, _mesh: &M) {
        // TODO: Note that Poly/Tri connectivity cannot "override" this. So this has to be
        // implemented with those taken into account.
        unimplemented!()
    }

    // TODO
    // - garbage_collection()
    // - resize(_, _, _)
    // - reserve(_, _, _)
}

