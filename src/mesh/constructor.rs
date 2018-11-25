//! Mesh constructors

use geometry::vector::Vec3d;
use mesh::mesh::Mesh;
use mesh::item_handle::{VertexHandle, HalfedgeHandle, FaceHandle};
use mesh::items::{Vertex, Halfedge, Edge, Face};
use property::traits::Handle; // For handle construction methods.

impl Mesh {
    /// Creates a new empty mesh. Same as Default:;default().
    pub fn new() -> Mesh {
        Default::default()
    }

    /// Creates a mesh from the given parts. This is a low-level crate-internal function.
    pub(crate) fn from_parts(vertices: Vec<Vertex>, edges: Vec<Edge>, faces: Vec<Face>) -> Mesh {
        Mesh {
            vertices,
            edges,
            faces,
            ..Default::default()
        }
    }

    /// Returns a mesh representing this triangle.
    pub fn triangle(_p1: Vec3d, _p2: Vec3d, _p3: Vec3d) -> Mesh {
        let vh = VertexHandle::from_index;
        let hh = HalfedgeHandle::from_index;
        let fh = FaceHandle::from_index;
        let inval = FaceHandle::new();
        //      _   0
        //      / / |\ \
        //     3 2    0 1
        //    / /      \ \
        //   /|/___4___\\ \|
        //  1 <----5----- 2
        let vertices = vec![
            Vertex { hh: hh(0) },
            Vertex { hh: hh(2) },
            Vertex { hh: hh(4) },
        ];
        let faces = vec![Face { hh: hh(0) }];
        let edges = vec![
            Edge([
                Halfedge { fh: fh(0), vh: vh(0), hnext: hh(2), hprev: hh(4) }, // hh 0
                Halfedge { fh: inval, vh: vh(2), hnext: hh(5), hprev: hh(3) }, // hh 1
            ]),
            Edge([
                Halfedge { fh: fh(0), vh: vh(1), hnext: hh(4), hprev: hh(0) }, // hh 2
                Halfedge { fh: inval, vh: vh(1), hnext: hh(1), hprev: hh(5) }, // hh 3
            ]),
            Edge([
                Halfedge { fh: fh(0), vh: vh(2), hnext: hh(0), hprev: hh(2) }, // hh 4
                Halfedge { fh: inval, vh: vh(0), hnext: hh(3), hprev: hh(1) }, // hh 5
            ]),
        ];
        // TODO: Actually use the positions by adding a position property.
        Mesh::from_parts(vertices, edges, faces)
    }
}

#[cfg(test)]
mod test {
    use mesh::mesh::Mesh;
    use geometry::vector::Vec3d;

    #[test]
    fn empty_mesh() {
        let mesh = Mesh::new();
        assert_eq!(mesh.vertices().len(), 0);
        assert_eq!(mesh.halfedges().len(), 0);
        assert_eq!(mesh.edges().len(), 0);
        assert_eq!(mesh.faces().len(), 0);
    }

    #[test]
    fn triangle() {
        let zero = Vec3d::new(0.0, 0.0, 0.0);
        let mesh = Mesh::triangle(zero, zero, zero);
        assert_eq!(mesh.vertices().len(), 3);
        assert_eq!(mesh.halfedges().len(), 6);
        assert_eq!(mesh.edges().len(), 3);
        assert_eq!(mesh.faces().len(), 1);
    }
}
