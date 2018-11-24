//! Mesh item handle types.

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

