//! Additional indexing traits analagous to `std::ops::Index` and `std::ops::IndexMut`, including
//! unsafe indexing variants and a setter trait. Includes implementations for the `std::vec::Vec`
//! and `util::vec::BitVec` types.

/// An unsafe alternative to `std::ops::Index` for getting the value at an index.
pub trait IndexUnchecked<Idx>: ::std::ops::Index<Idx> {
    /// Gets a value at index `index` without bounds checking.
    unsafe fn index_unchecked(&self, index: Idx) -> &Self::Output;
}

/// An unsafe alternative to `std::ops::IndexMut` for setting the value at an index.
pub trait IndexSetUnchecked<Idx>: ::std::ops::Index<Idx> {
    /// Moves the provided value into index `index` without bounds checking.
    unsafe fn index_set_unchecked(&mut self, index: Idx, value: Self::Output);
}

/// An alternative to `std::ops::IndexMut` for setting the value at an index.
pub trait IndexSet<Idx: ?Sized>: ::std::ops::Index<Idx> {
    /// Moves the provided value into index `index`. Panics if `index` is out of bounds.
    fn index_set(&mut self, index: Idx, value: Self::Output);
}

////////////////////////////////////////////////////////////////////////////////
// Impls for `Vec`

impl<T> IndexUnchecked<usize> for Vec<T> {
    unsafe fn index_unchecked(&self, index: usize) -> &Self::Output {
        assert!(index < self.len());
        self.get_unchecked(index)
    }
}

impl<T> IndexSetUnchecked<usize> for Vec<T> {
    unsafe fn index_set_unchecked(&mut self, index: usize, value: T) {
        assert!(index < self.len());
        *self.get_unchecked_mut(index) = value;
    }
}

impl<T> IndexSet<usize> for Vec<T> {
    fn index_set(&mut self, index: usize, value: T) {
        *self.get_mut(index).unwrap() = value;
    }
}

////////////////////////////////////////////////////////////////////////////////
// Impls for `BitVec`

use util::bitvec::BitVec;

static TRUE: bool = true;
static FALSE: bool = false;

impl IndexUnchecked<usize> for BitVec {
    unsafe fn index_unchecked(&self, index: usize) -> &Self::Output {
        assert!(index < self.len());
        let value = self.get_unchecked(index);
        if value { &TRUE } else { &FALSE }
    }
}

impl IndexSetUnchecked<usize> for BitVec {
    unsafe fn index_set_unchecked(&mut self, index: usize, value: bool) {
        assert!(index < self.len());
        self.set_unchecked(index, value);
    }
}

impl IndexSet<usize> for BitVec {
    fn index_set(&mut self, index: usize, value: bool) {
        self.set(index, value);
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use super::{IndexUnchecked, IndexSetUnchecked, IndexSet};
    use util::bitvec::BitVec;

    #[test]
    fn test_index_vec() {
        let mut vec = vec![1, 2, 3];
        unsafe {
            assert_eq!(*vec.index_unchecked(0), 1);
            assert_eq!(*vec.index_unchecked(1), 2);
            assert_eq!(*vec.index_unchecked(2), 3);
        }

        unsafe {
            vec.index_set_unchecked(0, 5);
            assert_eq!(*vec.index_unchecked(0), 5);
            vec.index_set_unchecked(2, 7);
            assert_eq!(*vec.index_unchecked(2), 7);
        }
        assert_eq!(vec, vec![5, 2, 7]);

        vec.index_set(0, 1);
        assert_eq!(vec[0], 1);
        vec.index_set(2, 3);
        assert_eq!(vec[2], 3);
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_index_bitvec() {
        let mut vec = BitVec::from_bytes(&[0xef, 0xa5, 0x71]);
        assert_eq!(vec[0], true);
        assert_eq!(vec[4], false);
        assert_eq!(vec[15], true);
        unsafe {
            assert_eq!(*vec.index_unchecked(0), true);
            assert_eq!(*vec.index_unchecked(4), false);
            assert_eq!(*vec.index_unchecked(15), true);
        }

        unsafe {
            vec.index_set_unchecked(0, false);
            assert_eq!(*vec.index_unchecked(0), false);
            vec.index_set_unchecked(15, false);
            assert_eq!(*vec.index_unchecked(15), false);
        }
        assert_eq!(vec.as_bytes(), &[0xee, 0x25, 0x71]);

        vec.index_set(0, true);
        assert_eq!(vec[0], true);
        vec.index_set(15, true);
        assert_eq!(vec[15], true);
        assert_eq!(vec.as_bytes(), &[0xef, 0xa5, 0x71]);
    }
}
