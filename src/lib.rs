#![crate_name = "openmesh"]

// Remove when Read#read_exact() is stabilized (https://github.com/rust-lang/rust/issues/27585).
#![feature(read_exact)]

#[macro_use(bitflags)]
extern crate bitflags;

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
