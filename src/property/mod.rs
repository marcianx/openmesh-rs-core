//! Defines mesh `Property` lists and related traits and data types.

pub mod traits;
#[macro_use]
mod handle;
pub use self::handle::*;

#[macro_use]
mod property;
pub use self::property::{Property, PropertyBits};
mod value;
pub use self::value::*;

mod container;
pub use self::container::PropertyContainer;
mod size;
pub use self::size::*;
