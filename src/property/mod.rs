//! Defines mesh `Property` lists and related traits and data types.

pub mod traits;
#[macro_use]
mod handle;
pub use self::handle::*;

#[macro_use]
mod property;
pub use self::property::*;
mod value;
pub use self::value::*;

mod container;
pub use self::container::*;
mod size;
pub use self::size::*;
