use std::io::{Read, Write};
use io::result::{Error, Result};

pub const UNKNOWN_SIZE: usize = !0usize;

/// A trait for describing whether an object can be serialized to file.
///
/// The default implementation assumes a non-serializable type.
pub trait Binary {
    /// Whether values of type T are streamable.
    fn is_streamable() -> bool { false }

    /// Size of all values of type T, if determinable.
    fn size_of_type() -> usize { UNKNOWN_SIZE }

    /// Size of a specific value of type T, if determinable.
    fn size_of_value(&self) -> usize { Self::size_of_type() }

    /// Stores `self` into `writer`, `swap`ping byte order if specified. Returns the number of
    /// bytes written on success.
    fn store(&self, _writer: &mut Write, _swap: bool) -> Result<usize> {
        Err(Error::Unsupported)
    }

    /// Loads `self` from `reader`, `swap`ping byte order if specified. Returns the number of
    /// bytes read on success.
    fn restore(&mut self, _reader: &mut Read, _swap: bool) -> Result<usize> {
        Err(Error::Unsupported)
    }
}


#[cfg(test)]
pub mod test {
    use std::fmt::Debug;
    use std::io::Cursor;
    use io::binary::Binary;

    pub fn test_store<T: Binary>(swap: bool, value: &T, expected_bytes: &[u8]) {
        let mut buf = Vec::<u8>::new();
        assert_eq!(value.store(&mut buf, swap).unwrap(), expected_bytes.len());
        assert_eq!(buf, expected_bytes);
    }

    pub fn test_restore<F, T>(swap: bool, bytes: &[u8], new: F, expected_value: &T)
        where T: Binary + Debug + PartialEq<T>, F: FnOnce() -> T
    {
        let mut buf = Cursor::new(Vec::from(bytes));
        let mut value = new();
        assert_eq!(value.restore(&mut buf, swap).unwrap(), bytes.len());
        assert_eq!(&value, expected_value);
    }
}

