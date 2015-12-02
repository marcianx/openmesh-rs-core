#![crate_name = "openmesh"]

// Remove when Read#read_exact() is stabilized (https://github.com/rust-lang/rust/issues/27585).
#![feature(read_exact)]

#[macro_use(bitflags)]
extern crate bitflags;

pub mod geometry;
pub mod mesh;
pub mod io;
pub mod util;
