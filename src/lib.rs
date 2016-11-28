//! This is a port of OpenMesh 5.1 Core to Rust.
//! The original [OpenMesh](http://www.openmesh.org/) is a C++ implementation of "A
//! generic and efficient polygon mesh data structure".
//! 
//! # Dependencies exposed in the API
//! 
//! * nalgebra-rs is used for geometric primitives like vectors.

#![feature(specialization)]

#![warn(missing_docs)]

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
