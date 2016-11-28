//! 2-manifold surface mesh represented as a halfedge data structure.

// TODO: Determine carefully what ought to be reexported.
pub mod handles;
pub mod iter;
pub mod status;

pub mod items;
pub mod prop;

mod mesh;
pub use self::mesh::Mesh;
