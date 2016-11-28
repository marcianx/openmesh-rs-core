//! General-purpose utilities.

extern crate bitvec_rs;

pub mod index;

/// Re-export for `bitvec_rs::BitVec` from the `bitvec-rs` crate.
pub mod bitvec {
    pub use super::bitvec_rs::BitVec;
}
