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


#[cfg(test)]
mod test {
    use super::{IndexUnchecked, IndexSetUnchecked, IndexSet};

    #[test]
    fn test_index() {
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
}
