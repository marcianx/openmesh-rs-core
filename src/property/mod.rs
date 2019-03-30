//! Defines mesh `Property` lists and related traits and data types.

#[macro_use]
pub mod traits;
pub mod handle;

#[macro_use]
mod property;
pub use self::property::{Property, PropertyBits};
mod value;
pub use self::value::*;

mod container;
pub use self::container::PropertyContainer;
pub mod size;
