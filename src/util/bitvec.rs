/// Implementation of a bit vector with the ability to get safe immutable and mutable views into
/// its internal vector for easy I/O.
///
/// Slices into the bit vector are guaranteed to have the unused bits on the last set to 0 in case
/// that is necessary and for the ease of writing unit tests.

// TODO: Flesh out docs.

use std::fmt;
use std::num::Wrapping;

#[derive(Eq)]
pub struct BitVec {
    nbits: usize,
    vec: Vec<u8>
}

fn bytes_in_bits(nbits: usize) -> usize {
    // #bytes = #ceil(nbits / 8)
    nbits / 8 +
        if nbits % 8 != 0 { 1 } else { 0 }
}

fn byte_from_bool(bit: bool) -> u8 {
    if bit { !0u8 } else { 0u8 }
}

impl BitVec {
    ////////////////////////////////////////
    // Constructors

    /// Constructs an empty `BitVec`.
    pub fn new() -> BitVec {
        BitVec { vec: Vec::new(), nbits: 0 }
    }

    /// Constructs a `BitVec` from bytes.
    pub fn from_bytes(bytes: &[u8]) -> BitVec {
        let mut vec = BitVec { vec: Vec::from(bytes), nbits: bytes.len() * 8 };
        vec.set_unused_zero();
        vec
    }

    /// Constructs a `BitVec` from a repeating bit value.
    pub fn from_elem(len: usize, value: bool) -> BitVec {
        let mut vec = BitVec {
            vec: vec![byte_from_bool(value); bytes_in_bits(len)],
            nbits: len
        };
        vec.set_unused_zero();
        vec
    }

    ////////////////////////////////////////
    // Converters/views

    /// Returns a byte slice view of the data.
    pub fn as_bytes(&self) -> &[u8] { &self.vec }

    /// Invokes the given function on a mut byte slice view of the data. After `f` completes, the
    /// trailing unused bits of the last byte are automatically set to 0.
    pub fn with_bytes_mut<U, F: FnOnce(&mut [u8]) -> U>(&mut self, f: F) -> U {
        let val = f(&mut self.vec);
        self.set_unused_zero();
        val
    }

    /// Consumes the `self` and returns the underlying `Vec<u8>` of length `ceil(self.len()/8)`.
    /// The values of the bits in the last byte of `Vec<u8>` beyond the length of the `BitVec` are
    /// unspecified.
    pub fn into_vec(self) -> Vec<u8> { self.vec }

    ////////////////////////////////////////
    // Getters/setters

    /// Returns the length of the bit vector.
    pub fn len(&self) -> usize { self.nbits }

    /// Validates the index for validity or panics.
    fn validate_index(&self, index: usize) {
        assert!(self.nbits <= self.vec.len() * 8,
                "Expected #bits {} <= 8 x (#bytes {} in vec).", self.nbits, self.vec.len());
        if index >= self.nbits { panic!("Index {} out of bounds [0, {})", index, self.nbits); }
    }

    /// Gets the bit at the given `index`. Panics if `index` exceeds length.
    pub fn get(&self, index: usize) -> bool {
        self.validate_index(index);
        unsafe { self.get_unchecked(index) }
    }

    /// Sets the bit at the given `index`. Panics if `index` exceeds length.
    pub fn set(&mut self, index: usize, value: bool) {
        self.validate_index(index);
        unsafe { self.set_unchecked(index, value) };
    }

    /// Gets the bit at the given `index` without bounds checking.
    pub unsafe fn get_unchecked(&self, index: usize) -> bool {
        let byte = self.vec.get_unchecked(index / 8);
        let pattern = 1u8 << (index % 8);
        (*byte & pattern) != 0u8
    }

    /// Sets the bit at the given `index` without bounds checking.
    pub unsafe fn set_unchecked(&mut self, index: usize, value: bool) {
        let byte = self.vec.get_unchecked_mut(index / 8);
        let pattern = 1u8 << (index % 8);
        *byte = if value { *byte |  pattern }
                else     { *byte & !pattern };
    }

    ////////////////////////////////////////
    // Adding/removing items

    /// Pushes a boolean to the end of the `BitVec`.
    pub fn push(&mut self, value: bool) {
        let nbits = self.nbits; // avoid mutable borrow error
        if nbits % 8 == 0 {
            self.vec.push(if value { 1u8 } else { 0u8 });
        } else {
            unsafe { self.set_unchecked(nbits, value) };
        }
        self.nbits += 1;
    }

    /// Pops a boolean from the end of the `BitVec`.
    pub fn pop(&mut self) -> Option<bool> {
        if self.nbits == 0 { return None }
        self.nbits -= 1;

        // Get the popped bit value to return.
        let nbits = self.nbits; // avoid mutable borrow error
        let value = unsafe { self.get_unchecked(nbits) };
        // Set the popped bit value to 0.
        unsafe { self.set_unchecked(nbits, false); }
        // Pop off the last byte from the underlying vector if it has no active bits.
        if self.nbits % 8 == 0 {
            assert!(self.nbits == (self.vec.len() - 1) * 8,
                "Expected #bits {} == 8 x (#bytes {} in vec - 1) after bit pop and before vec pop.",
                self.nbits, self.vec.len());
            self.vec.pop();
        }

        Some(value)
    }

    /// Clears the `BitVec`, removing all values.
    pub fn clear(&mut self) {
        self.vec.clear();
        self.nbits = 0;
    }

    ////////////////////////////////////////
    // Iterators

    /// Returns an iterator for the booleans in the array.
    pub fn iter(&self) -> Iter {
        Iter { vec: self, index: 0 }
    }

    ////////////////////////////////////////
    // Helpers

    /// Sets the extra unused bits in the bitvector to 0.
    fn set_unused_zero(&mut self) {
        if self.nbits % 8 == 0 { return }
        let len = self.vec.len(); // avoid mutable borrow error
        assert!(len > 0);

        let byte = unsafe { self.vec.get_unchecked_mut(len - 1) };
        // Pattern with all 1's in the used bits only, avoiding overflow check in debug.
        let pattern = (Wrapping(1u8 << (self.nbits % 8)) - Wrapping(1u8)).0;
        *byte &= pattern;
    }
}

impl PartialEq<BitVec> for BitVec {
    fn eq(&self, other: &BitVec) -> bool {
        self.nbits == other.nbits && self.vec == other.vec
    }
}

impl Clone for BitVec {
    fn clone(&self) -> BitVec {
        BitVec { vec: self.vec.clone(), nbits: self.nbits }
    }

    fn clone_from(&mut self, other: &BitVec) {
        self.nbits = other.nbits;
        self.vec.clone_from(&other.vec);
    }
}

impl fmt::Debug for BitVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BitVec{{{:?}: {}}}", self.nbits, &self)
    }
}

impl fmt::Display for BitVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (val, index) in self.iter().zip(0..usize::max_value()) {
            if index > 0 && index % 8 == 0 {
                try!(write!(f, " "));
            }
            try!(write!(f, "{}", if val { "1" } else { "." }));
        }
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Iterators

pub struct Iter<'a> {
    vec: &'a BitVec,
    index: usize
}

impl<'a> Iterator for Iter<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.vec.nbits {
            None
        } else {
            let val = unsafe { self.vec.get_unchecked(self.index) };
            self.index += 1;
            Some(val)
        }
    }
}


#[cfg(test)]
mod test {
    use util::bitvec::BitVec;

    #[test]
    fn test_constructors() {
        let vec = BitVec::new();
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.as_bytes(), &[]);

        let vec = BitVec::from_bytes(&[0xab, 0xcd]);
        assert_eq!(vec.len(), 16);
        assert_eq!(vec.as_bytes(), &[0xab, 0xcd]);

        let vec = BitVec::from_elem(4, true);
        assert_eq!(vec.len(), 4);
        assert_eq!(vec.as_bytes(), &[0x0f]);

        let vec = BitVec::from_elem(31, true);
        assert_eq!(vec.len(), 31);
        assert_eq!(vec.as_bytes(), &[0xff, 0xff, 0xff, 0x7f]);

        let vec = BitVec::from_elem(4, false);
        assert_eq!(vec.len(), 4);
        assert_eq!(vec.as_bytes(), &[0]);

        let vec = BitVec::from_elem(31, false);
        assert_eq!(vec.len(), 31);
        assert_eq!(vec.as_bytes(), &[0, 0, 0, 0]);
    }

    #[test]
    fn test_with_bytes_mut() {
        let mut vec = BitVec::from_elem(28, false);
        assert_eq!(vec.len(), 28);
        assert_eq!(vec.as_bytes(), &[0, 0, 0, 0]);

        // Fill the underlying buffers with all 1s.
        vec.with_bytes_mut(|slice| {
            assert_eq!(slice.len(), 4);
            for i in 0..4 { slice[i] = 0xff; }
        });
        // Expect the unused bytes to be zeroed out.
        assert_eq!(vec.as_bytes(), &[0xff, 0xff, 0xff, 0x0f]);
    }

    #[test]
    fn test_into_vec() {
        let mut vec = BitVec::from_bytes(&[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0xe3]);
        vec.pop(); vec.pop();
        assert_eq!(vec.len(), 54);
        let vec = vec.into_vec();
        assert_eq!(vec, &[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23]);
    }

    #[test]
    fn test_get_set() {
        let mut vec = BitVec::from_bytes(&[0xef, 0xa5, 0x71]);
        assert_eq!(vec.as_bytes(), &[0xef, 0xa5, 0x71]);
        assert_eq!(true, vec.get(8));

        vec.set(8, true);
        assert_eq!(true, vec.get(8));
        assert_eq!(vec.as_bytes(), &[0xef, 0xa5, 0x71]);

        vec.set(8, false);
        assert_eq!(false, vec.get(8));
        assert_eq!(vec.as_bytes(), &[0xef, 0xa4, 0x71]);

        vec.set(7, false);
        assert_eq!(false, vec.get(7));
        assert_eq!(vec.as_bytes(), &[0x6f, 0xa4, 0x71]);
    }

    #[test]
    fn test_pop_to_empty() {
        let mut vec = BitVec::new();
        assert_eq!(vec.pop(), None);
        assert_eq!(vec.pop(), None);

        let mut vec = BitVec::from_bytes(&[0b01111111]);
        assert_eq!(vec.pop(), Some(false));
        assert_eq!(vec.len(), 7);
        for _ in 0..7 {
            assert_eq!(vec.pop(), Some(true));
        }
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.pop(), None);
        assert_eq!(vec.pop(), None);
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn test_pop_push() {
        let mut vec = BitVec::from_bytes(&[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0b11100011]);
        assert_eq!(vec.len(), 56);

        // Pop 2 bits and expect the slice view to show zeros for them.
        assert_eq!(vec.pop(), Some(true));
        assert_eq!(vec.pop(), Some(true));
        assert_eq!(vec.len(), 54);
        assert_eq!(vec.as_bytes(), &[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0b00100011]);

        // Finish popping the byte and expect the slice to be one byte shorter.
        assert_eq!(vec.pop(), Some(true));
        assert_eq!(vec.pop(), Some(false));
        assert_eq!(vec.pop(), Some(false));
        assert_eq!(vec.pop(), Some(false));
        assert_eq!(vec.pop(), Some(true));
        assert_eq!(vec.pop(), Some(true));
        assert_eq!(vec.len(), 48);
        assert_eq!(vec.as_bytes(), &[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45]);

        // Push another byte in.
        for _ in 0..4 {
            vec.push(true);
            vec.push(false);
        }
        assert_eq!(vec.len(), 56);
        assert_eq!(vec.as_bytes(), &[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0b01010101]);
    }

    #[test]
    fn test_clear() {
        let mut vec = BitVec::from_bytes(&[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0xe3]);
        assert_eq!(vec.len(), 56);
        vec.clear();
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.as_bytes(), &[]);
    }

    fn assert_iter_eq(vec: &BitVec, expected: &Vec<bool>) {
        let actual: Vec<bool> = vec.iter().collect();
        assert_eq!(&actual, expected);
    }

    #[test]
    fn test_iter() {
        let l = true;
        let o = false;

        assert_iter_eq(&BitVec::new(), &Vec::new());

        let mut vec = BitVec::from_bytes(&[0xef, 0xa5, 0x71]);
        // low bit to high bit:       f       e        5       a        1       7
        assert_iter_eq(&vec, &vec![l,l,l,l,o,l,l,l, l,o,l,o,o,l,o,l, l,o,o,o,l,l,l,o]);
        vec.pop(); vec.pop();
        
        // low bit to high bit:       f       e        5       a        1     3
        assert_iter_eq(&vec, &vec![l,l,l,l,o,l,l,l, l,o,l,o,o,l,o,l, l,o,o,o,l,l]);
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn test_get_validation() {
        &BitVec::from_bytes(&[0xef, 0xa5, 0x71]).get(24);
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn test_set_validation() {
        &BitVec::from_bytes(&[0xef, 0xa5, 0x71]).set(24, true);
    }

    #[test]
    fn test_eq() {
        let vec1 = BitVec::from_bytes(&[0xef, 0xa5, 0x71]);
        let mut vec2 = BitVec::from_bytes(&[0xef, 0xa5, 0x71]);
        assert!(vec1 == vec2);
        vec2.push(true);
        assert!(vec1 != vec2);
        vec2.pop();
        assert!(vec1 == vec2);
        vec2.set(3, false);
        assert!(vec1 != vec2);
    }

    #[test]
    fn test_clone() {
        let mut vec = BitVec::from_bytes(&[0xef, 0xa5, 0x71]);
        assert_eq!(vec, vec.clone());
        vec.pop(); vec.pop();
        assert_eq!(vec, vec.clone());
    }

    #[test]
    fn test_debug() {
        assert_eq!(
            format!("{:?}", &BitVec::from_bytes(&[0xef, 0xa5, 0x71])),
            "BitVec{24: 1111.111 1.1..1.1 1...111.}"
        )
    }
}
