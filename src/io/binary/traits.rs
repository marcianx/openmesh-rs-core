use std::io::{Read, Write};
use io::result::{Error, Result};

extern crate byteorder;
// TODO: Reexporting byteorder's traits and types. Consider not leaking this implementation detail
// by creating a wrapper trait/class around this.
/// Trait for staticly-dispatched endianness.
pub use self::byteorder::ByteOrder;
/// Types implementing `ByteOrder` for staticly-dispatched endianness.
pub use self::byteorder::{BigEndian, LittleEndian};

pub const UNKNOWN_SIZE: usize = !0usize;

/// Endianness for storing/restoring.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Endian {
    Big,
    Little,
}

/// A trait for describing whether an object can be serialized to file.
///
/// See also the related `impl_binary_notstreamable` and `impl_binary_streamablevec` macros to
/// auto-generate implementations.
pub trait Binary {
    /// Whether values of type T are streamable.
    fn is_streamable() -> bool { false }

    /// Size of all values of type T, if determinable.
    fn size_of_type() -> usize { UNKNOWN_SIZE }

    /// Size of a specific value of type T, if determinable.
    fn size_of_value(&self) -> usize { Self::size_of_type() }

    /// Stores `self` into `writer` with statically-specified byte order. Returns the number of
    /// bytes written on success.
    #[inline(always)]
    fn store_endian<B: ByteOrder>(&self, _writer: &mut Write) -> Result<usize> {
        Err(Error::Unsupported)
    }

    /// Loads `self` from `reader` with statically-specified byte order. Returns the number of
    /// bytes read on success.
    #[inline(always)]
    fn restore_endian<B: ByteOrder>(&mut self, _reader: &mut Read) -> Result<usize> {
        Err(Error::Unsupported)
    }

    /// Stores `self` into `writer` with the specified byte order. Returns the number of bytes
    /// written on success.
    fn store(&self, writer: &mut Write, endian: Endian) -> Result<usize> {
        match endian {
            Endian::Big    => self.store_endian::<BigEndian>(writer),
            Endian::Little => self.store_endian::<LittleEndian>(writer),
        }
    }

    /// Loads `self` from `reader` with the specified byte order. Returns the number of bytes read
    /// on success.
    fn restore(&mut self, reader: &mut Read, endian: Endian) -> Result<usize> {
        match endian {
            Endian::Big    => self.restore_endian::<BigEndian>(reader),
            Endian::Little => self.restore_endian::<LittleEndian>(reader),
        }
    }
}

// NOTE:
// Unfortunately, a strategy that uses, say, `NonStreamable` and `StreamableBinaryVec` marker
// traits for auto-generation of implementations does not work due to current limitations in rust.
//
// Specifically, the last line in
//
// ```rust
// /// Marker trait providing a non-streamable `Binary` implementation for `Vec<T: NonStreamable>` and
// /// (recursively) `Vec<...Vec<T: NonStreamable>...>` types.
// trait NonStreamable {}
// // Vectors of non-streamable objects are non-streamable.
// impl<T: NonStreamable> NonStreamable for Vec<T> {}
// // Non-streamable objects have `Binary` trait implementations.
// impl<T: NonStreamable> Binary for T {}
// ```
//
// conflicts with `impl<T: StreamableBinaryVec> Binary for Vec<T>` since there is no guarantee that
// some type won't implement both traits resulting in ambiguity. Reconsider this approach since it
// it better enforces types once rust supports trait impl specialization.

/// Provides a non-streamable implementation of `Binary` for the provided type and arrays of that
/// type.
#[macro_export]
macro_rules! impl_binary_notstreamable {
    ($ty: ty) => {
        // Vectors of non-streamable objects are non-streamable.
        impl Binary for Vec<$ty> {}
        // Non-streamable objects have `Binary` trait implementations.
        impl Binary for $ty {}
    }
}

/// Provides a streamable implementation of `Binary` for vector of the provided streamable type
/// which is required to have a streamable `Binary` implementation for correct implementation.
#[macro_export]
macro_rules! impl_binary_streamablevec {
    ($ty: ty) => {
        impl Binary for Vec<$ty> {
            fn is_streamable() -> bool { true }
            fn size_of_value(&self) -> usize {
                self.iter().map(|s| s.size_of_value()).fold(0, |a, b| a + b)
            }

            fn store_endian<B: ByteOrder>(&self, writer: &mut Write) -> Result<usize> {
                let mut size = 0;
                for s in self.iter() {
                    size += try!(s.store_endian::<B>(writer));
                }
                Ok(size)
            }

            /// Note: This reads exactly as many items as the existing length of self.
            fn restore_endian<B: ByteOrder>(&mut self, reader: &mut Read) -> Result<usize> {
                let mut size = 0;
                for s in self.iter_mut() {
                    size += try!(s.restore_endian::<B>(reader));
                }
                Ok(size)
            }
        }
    }
}


#[cfg(test)]
pub mod test {
    use std::fmt::Debug;
    use std::io::Cursor;
    use io::binary::{Binary, UNKNOWN_SIZE};
    use io::binary::traits::Endian;

    pub fn test_store<T: Binary>(endian: Endian, value: &T, expected_bytes: &[u8]) {
        let mut buf = Vec::<u8>::new();
        assert_eq!(value.size_of_value(), expected_bytes.len());
        if <T as Binary>::size_of_type() != UNKNOWN_SIZE {
            assert_eq!(value.size_of_value(), <T as Binary>::size_of_type());
        }
        assert_eq!(value.store(&mut buf, endian).unwrap(), expected_bytes.len());
        assert_eq!(buf, expected_bytes);
    }

    pub fn test_restore<F, T>(endian: Endian, bytes: &[u8], new: F, expected_value: &T)
        where T: Binary + Debug + PartialEq<T>, F: FnOnce() -> T
    {
        let mut buf = Cursor::new(Vec::from(bytes));
        let mut value = new();
        assert_eq!(value.restore(&mut buf, endian).unwrap(), bytes.len());
        assert_eq!(&value, expected_value);
    }
}
