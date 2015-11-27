use std::io::{Read, Write};

use io::binary::traits::*;
use io::result::{Error, Result};

////////////////////////////////////////////////////////////////////////////////
// Implementation for string.

impl Binary for String {
    fn is_streamable() -> bool { true }
    fn size_of_value(&self) -> usize { self.len() }

    fn store(&self, writer: &mut Write, swap: bool) -> Result<usize> {
        let len = self.len();
        if len > u16::max_value() as usize {
            return Err(Error::StringExceeds64k)
        }
        // TODO: OpenMesh has a bug where len is double-swapped.
        // Reproduce the bug for backward-compatibility with OpenMesh files?
        let len_size =try!((len as u16).store(writer, swap));
        try!(writer.write_all(self.as_bytes()));
        Ok(len + len_size)
    }

    fn restore(&mut self, reader: &mut Read, swap: bool) -> Result<usize> {
        let mut len = 0u16;
        // TODO: OpenMesh has a bug where len is double-swapped.
        // Reproduce the bug for backward-compatibility with OpenMesh files?
        let len_size = try!(len.restore(reader, swap));
        let len = len as usize;

        self.clear();
        self.reserve_exact(len);
        try!(reader.take(len as u64).read_to_string(self));
        Ok(len + len_size)
    }
}


#[cfg(test)]
mod test_string {
    use ::io::binary::test;

    #[test]
    fn test_store() {
        let s = String::from("hello");
        test::test_store(false, &s, &[5, 0, 104, 101, 108, 108, 111]);
        test::test_store(true , &s, &[0, 5, 104, 101, 108, 108, 111]);
    }

    #[test]
    fn test_restore() {
        test::test_restore(false, &[5, 0, 104, 101, 108, 108, 111], || String::from("prev-content"), &String::from("hello"));
        test::test_restore(true , &[0, 5, 104, 101, 108, 108, 111], || String::from("prev-content"), &String::from("hello"));
    }
}

////////////////////////////////////////////////////////////////////////////////
// Implementation for exact-size iterators of strings.

// TODO

