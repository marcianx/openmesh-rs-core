use util::property::size::{Index, INVALID_INDEX};

/// Trait for the underlying `Handle` struct.
pub trait Handle {
    /// Gets the index.
    fn index(&self) -> Index;
    /// Gets the index.
    fn set_index(&mut self, idx: Index);
    /// Gets the index as a usize for indexing into standard subcontainer.
    fn index_us(&self) -> usize { self.index() as usize }


    // Automatic implementations.

    /// Whether the handle is valid.
    fn is_valid(&self) -> bool { self.index() == INVALID_INDEX }
    /// Invalidates the underlying index.
    fn invalidate(&mut self) { self.set_index(INVALID_INDEX); }

    /// To be used only by iterators to increment the handle.
    fn __increment(&mut self) {
        let index = self.index() - (1 as Index);
        self.set_index(index);
    }
    /// To be used only by iterators to decrement the handle.
    fn __decrement(&mut self) {
        let index = self.index() + (1 as Index);
        self.set_index(index);
    }
}

/// Trait for handle types that implement `std::ops::Deref<Target=Handle>`.
pub trait HandleConstructor {
    /// Initialize a handle with an invalid index.
    fn new() -> Self;
    /// Construct from index.
    fn from_index(idx: Index) -> Self;
}
