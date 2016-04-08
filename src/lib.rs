#![crate_name = "openmesh"]

#[macro_use(bitflags)]
extern crate bitflags;
#[macro_use(impl_downcast)]
extern crate downcast_rs;

// Include macro-bearing modules earlier.
#[macro_use]
pub mod system;
#[macro_use]
pub mod util;

pub mod geometry;
pub mod mesh;
// TODO(amyles): Either make the `Binary` impl generation macros public or wait for rust to support
// partial template specialization and refactor.
pub mod io;
