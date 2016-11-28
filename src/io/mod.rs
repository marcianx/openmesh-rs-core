//! OpenMesh I/O support.
//!
//! Of particular note is the `Binary` trait which must be defined for all types storable in mesh
//! property lists, whether or not they support loading/storing to disk.

pub mod binary;
pub mod options;
pub mod result;

