#![crate_name = "openmesh"]

#[macro_use(bitflags)]
extern crate bitflags;
#[macro_use(impl_downcast)]
extern crate downcast_rs;

// Include macro-bearing modules earlier.
#[macro_use]
pub mod system;
#[macro_use]
pub mod property;

pub mod geometry;
pub mod io;
pub mod mesh;
pub mod util;
