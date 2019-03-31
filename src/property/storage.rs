use crate::io::binary::{Binary, UNKNOWN_SIZE};
use crate::property::Value;
use crate::util::bitvec::BitVec;
use crate::util::index::{IndexUnchecked, IndexSetUnchecked, IndexSet};

/// Abstraction to support multiple underlying storage types for property lists.
pub trait Storage:
    Clone + Default +
    ::std::ops::Index<usize> +
    IndexUnchecked<usize> +
    IndexSetUnchecked<usize> +
    IndexSet<usize>
{
    /// Type of the items stored in the storage.
    type Value: Clone + Default;

    /// Create object with given storage size.
    fn new() -> Self { Default::default() }
    /// Element size w.r.t. storing the full storage.
    fn element_size() -> usize;
    /// Number of elements.
    fn len(&self) -> usize;
    /// Whether the storage is empty.
    fn is_empty(&self) -> bool;
    /// Get an element at the given location.
    fn get(&self, i: usize) -> &Self::Value;
    /// Get an element at the given location without bounds checks.
    unsafe fn get_unchecked(&self, i: usize) -> &Self::Value;
    /// Set an element at the given location.
    fn set(&mut self, i: usize, value: Self::Value);
    /// Set an element at the given location without bounds checks.
    unsafe fn set_unchecked(&mut self, i: usize, value: Self::Value);
    /// Swap the elements at the given indices.
    fn swap(&mut self, i: usize, j: usize);
    /// Resize to a certain size.
    fn resize(&mut self, n: usize);
    /// Reserve to be able to push the given number of elements without further reallocations.
    fn reserve_more(&mut self, n: usize);
    /// Push an additional default value.
    fn push(&mut self);
}

/// Allows picking the optimal storage container for implemented type.
pub trait StorageFor: Clone + Default {
    /// Storage container type to be used to store objects of type `Self`.
    type Storage: Storage<Value=Self> + Binary;
}

impl<T: Value> Storage for Vec<T> {
    type Value = T;
    fn len(&self) -> usize { Vec::len(self) }
    fn is_empty(&self) -> bool { Vec::is_empty(self) }
    fn element_size() -> usize { <T as Binary>::size_of_type() }
    fn get(&self, i: usize) -> &Self::Value { &self[i] }
    unsafe fn get_unchecked(&self, i: usize) -> &Self::Value { &self[i] }
    fn set(&mut self, i: usize, value: T) { self[i] = value; }
    unsafe fn set_unchecked(&mut self, i: usize, value: T) { *self.get_unchecked_mut(i) = value; }
    fn swap(&mut self, i: usize, j: usize) { <[T]>::swap(self, i, j) }
    fn resize(&mut self, n: usize) { Vec::resize(self, n, Default::default()) }
    fn reserve_more(&mut self, n: usize) { self.reserve(n) }
    fn push(&mut self) { Vec::push(self, Default::default()) }
}

impl<T: Value> StorageFor for T {
    default type Storage = Vec<T>;
}

impl Storage for BitVec {
    type Value = bool;
    fn len(&self) -> usize { BitVec::len(self) }
    fn is_empty(&self) -> bool { BitVec::is_empty(self) }
    fn element_size() -> usize { UNKNOWN_SIZE }
    fn get(&self, i: usize) -> &Self::Value { &self[i] }
    unsafe fn get_unchecked(&self, i: usize) -> &Self::Value {
        <Self as IndexUnchecked<usize>>::index_unchecked(&self, i)
    }
    fn set(&mut self, i: usize, value: bool) { BitVec::set(self, i, value); }
    unsafe fn set_unchecked(&mut self, i: usize, value: bool) {
        BitVec::set_unchecked(self, i, value);
    }
    fn swap(&mut self, i: usize, j: usize) { BitVec::swap(self, i, j) }
    fn resize(&mut self, n: usize) { BitVec::resize(self, n, Default::default()) }
    fn reserve_more(&mut self, n: usize) { self.reserve(n) }
    fn push(&mut self) { BitVec::push(self, Default::default()) }
}

impl StorageFor for bool {
    type Storage = BitVec;
}
