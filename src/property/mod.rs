//! Defines mesh `Property` lists and related traits and data types.

#[macro_use]
mod handle;
pub use self::handle::*;
mod property;
pub use self::property::*;

mod list;
pub use self::list::*;
mod value;
pub use self::value::*;

mod container;
pub use self::container::*;
mod size;
pub use self::size::*;
