//! These types and constants are to be used for indices and sizes of property lists, but **not**
//! for binary I/O size (trait `io::trait::Binary`), which may be larger than `Size` defined here.

/// The underlying (primitive) type representation to index into a property container.
pub type Index = u32;
/// The underlying (primitive) type representation to represent property container sizes.
pub type Size = u32;
/// An invalid `Index` value used to indicate an uninitialized index.
pub const INVALID_INDEX: Index = !(0 as Index);
