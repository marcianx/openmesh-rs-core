//! 2-manifold surface mesh represented as a halfedge data structure.

// TODO: Determine carefully what ought to be reexported.
pub mod item_handle;
pub mod iter;
pub mod status;

pub mod items;
pub mod prop;
pub mod rc;

mod mesh;
mod constructor;
pub use self::mesh::Mesh;
