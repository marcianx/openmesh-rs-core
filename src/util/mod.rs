//! General-purpose utilities.

pub mod index;

/// Re-export for `bitvec_rs::BitVec` from the `bitvec-rs` crate.
pub mod bitvec {
    pub use bitvec_rs::BitVec;
}
