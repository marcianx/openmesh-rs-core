use property::traits::{Handle, PropHandle};

/// Ref-counted property handle.
pub struct RCPropHandle<H: PropHandle> {
    handle: H,
    ref_count: usize,
}

impl<H: PropHandle> RCPropHandle<H> {
    /// Returns a `RCPropHandle` with an invalid handle an 0 ref count.
    pub fn new() -> RCPropHandle<H> {
        RCPropHandle {
            handle: H::new(),
            ref_count: 0,
        }
    }
}

