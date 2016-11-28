//! Defines the `Binary` trait which must be defined for all types storable in mesh property lists.
//! Implements this trait for various common data types.

// Trait definition
mod traits;
pub use self::traits::*;

// Trait implementations
mod bitvec;
mod primitives;
mod string;
mod status;
mod geometry;
pub use self::bitvec::*;
pub use self::primitives::*;
pub use self::string::*;
pub use self::status::*;
pub use self::geometry::*;

