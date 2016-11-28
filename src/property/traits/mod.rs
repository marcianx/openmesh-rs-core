//! Traits for `Property` lists, handles, and types that can be stored in property lists.

#[macro_use]
mod handle;

mod property;
mod value;

pub use self::handle::*;
pub use self::property::*;
pub use self::value::*;
