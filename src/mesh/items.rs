//! Defines the core primitives (`Vertex`, `Halfedge`, `Edge`, `Face`) encoding mesh connectivity.

use mesh::handles::{
    VertexHandle, HalfedgeHandle, FaceHandle,
};

////////////////////////////////////////////////////////////
// Half-edge data structure connectivity fields

/// Vertex fields for HDS topology.
#[derive(Clone, Default)]
pub struct Vertex {
    /// An outgoing halfedge, if any, from this vertex.
    pub hh: HalfedgeHandle,
}

/// Halfedge fields for HDS topology.
#[derive(Clone, Default)]
pub struct Halfedge {
    /// The face, if any, to which this halfedge belongs.
    pub fh: FaceHandle,
    /// The vertex this halfedge points to.
    pub vh: VertexHandle,
    /// The next halfedge going counter-clockwise around the face.
    pub hnext: HalfedgeHandle,
    /// The previous halfedge - i.e. the next one going clockwise around the face.
    pub hprev: HalfedgeHandle,
}

/// Edge fields for HDS topology.
#[derive(Default)]
pub struct Edge {
    /// The pair of halfedges constituting the edge.
    pub halfedges: [Halfedge; 2],
}
// Explicitly implement `Clone` to avoid requiring `Copy` on `Halfedge`.
impl Clone for Edge {
    fn clone(&self) -> Self {
        Edge {
            halfedges: [self.halfedges[0].clone(), self.halfedges[1].clone()]
        }
    }
}

/// Face fields for HDS topology.
#[derive(Clone, Default)]
pub struct Face {
    /// A halfedge bounding this face.
    pub hh: HalfedgeHandle,
}

