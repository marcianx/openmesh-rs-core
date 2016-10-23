use mesh::handles::{
    VertexHandle, HalfedgeHandle, EdgeHandle, FaceHandle, MeshHandle,
};
use mesh::items::{Vertex, Edge, Face};
use mesh::prop::{
    VProps, HProps, EProps, FProps, MProps,
    VPropsMut, HPropsMut, EPropsMut, FPropsMut, MPropsMut,
};
use property::PropertyContainer;

////////////////////////////////////////////////////////////
// Mesh

/// Mesh implementation detail.
pub struct _Mesh {
    // Item connectivity and properties.
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,

    // Properties by item type.
    pub vprops: PropertyContainer<VertexHandle>,
    pub hprops: PropertyContainer<HalfedgeHandle>,
    pub eprops: PropertyContainer<EdgeHandle>,
    pub fprops: PropertyContainer<FaceHandle>,
    pub mprops: PropertyContainer<MeshHandle>,
}

/// Halfedge data structure.
pub struct Mesh(_Mesh);

////////////////////////////////////////////////////////////////////////////////
// Properties

macro_rules! prop_accessors {
    ($method:ident, $method_mut:ident, $Struct:ident, $StructMut:ident, $field:ident, $item:expr, $len_fn:expr) => {
        #[doc="Returns a struct to access "] #[doc=$item] #[doc=" properties."]
        pub fn $method(&self) -> $Struct {
            let len = $len_fn(&self.0);
            $Struct::new(&(self.0).$field, len)
        }
        #[doc="Returns a struct to mutably access "] #[doc=$item] #[doc=" properties."]
        pub fn $method_mut(&mut self) -> $StructMut {
            let len = $len_fn(&self.0);
            $StructMut::new(&mut (self.0).$field, len)
        }
    }
}

impl Mesh {
    // Property accessors.
    prop_accessors!( vertices,  vertices_mut, VProps, VPropsMut, vprops,   "vertex", | m: &_Mesh| { m.vertices.len() });
    prop_accessors!(halfedges, halfedges_mut, HProps, HPropsMut, hprops, "halfedge", | m: &_Mesh| { m.edges.len() * 2 });
    prop_accessors!(    edges,     edges_mut, EProps, EPropsMut, eprops,     "edge", | m: &_Mesh| { m.edges.len() });
    prop_accessors!(    faces,     faces_mut, FProps, FPropsMut, fprops,     "face", | m: &_Mesh| { m.faces.len() });
    prop_accessors!(     mesh,      mesh_mut, MProps, MPropsMut, mprops,     "mesh", |_m: &_Mesh| { 1 });
}
