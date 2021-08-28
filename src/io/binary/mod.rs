//! Defines the `Binary` trait which must be defined for all types storable in mesh property lists.
//! Implements this trait for various common data types.

// Trait definition
mod traits;
pub use self::traits::*;

// Trait implementations
mod bitvec;
mod geometry;
mod primitives;
mod status;
mod string;
pub use self::bitvec::*;
pub use self::geometry::*;
pub use self::primitives::*;
pub use self::status::*;
pub use self::string::*;
