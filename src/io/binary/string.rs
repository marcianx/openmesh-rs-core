use std::io::{Read, Write};

use crate::io::binary::traits::{Binary, ByteOrder};
use crate::io::result::{Error, Result};

////////////////////////////////////////////////////////////////////////////////
// Implementation for string.

impl Binary for String {
    fn is_streamable() -> bool { true }
    fn size_of_value(&self) -> usize { self.len() + <u16 as Binary>::size_of_type() }

    fn store_endian<B: ByteOrder>(&self, writer: &mut dyn Write) -> Result<usize> {
        let len = self.len();
        if len > u16::max_value() as usize {
            return Err(Error::StringExceeds64k)
        }
        // TODO: OpenMesh has a bug where len is double-swapped.
        // Reproduce the bug for backward-compatibility with OpenMesh files?
        let len_size = (len as u16).store_endian::<B>(writer)?;
        writer.write_all(self.as_bytes())?;
        Ok(len + len_size)
    }

    fn restore_endian<B: ByteOrder>(&mut self, reader: &mut dyn Read) -> Result<usize> {
        let mut len = 0u16;
        // TODO: OpenMesh has a bug where len is double-swapped.
        // Reproduce the bug for backward-compatibility with OpenMesh files?
        let len_size = len.restore_endian::<B>(reader)?;
        let len = len as usize;

        self.clear();
        self.reserve_exact(len);
        reader.take(len as u64).read_to_string(self)?;
        Ok(len + len_size)
    }
}


#[cfg(test)]
mod test {
    use crate::io::binary::test;
    use crate::io::binary::traits::Endian::{Big, Little};

    #[test]
    fn test_store() {
        test::test_store(Little, &String::new(), &[0, 0]);
        test::test_store(Big   , &String::new(), &[0, 0]);

        let s = String::from("hello");
        test::test_store(Little, &s, &[5, 0, 104, 101, 108, 108, 111]);
        test::test_store(Big   , &s, &[0, 5, 104, 101, 108, 108, 111]);
    }

    #[test]
    fn test_restore() {
        test::test_restore(Little, &[0, 0], String::new, &String::new());
        test::test_restore(Big   , &[0, 0], String::new, &String::new());
        test::test_restore(Little, &[5, 0, 104, 101, 108, 108, 111], || String::from("prev-content"), &String::from("hello"));
        test::test_restore(Big   , &[0, 5, 104, 101, 108, 108, 111], || String::from("prev-content"), &String::from("hello"));
    }
}


#[cfg(test)]
mod test_vec {
    use crate::io::binary::test;
    use crate::io::binary::traits::Endian::{Big, Little};

    #[test]
    fn test_store() {
        test::test_store(Little, &Vec::<String>::new(), &[]);
        test::test_store(Big   , &Vec::<String>::new(), &[]);

        let vec = vec![String::from("hello"), String::from(" world")];
        test::test_store(Little, &vec, &[5, 0, 104, 101, 108, 108, 111, 6, 0, 32, 119, 111, 114, 108, 100]);
        test::test_store(Big   , &vec, &[0, 5, 104, 101, 108, 108, 111, 0, 6, 32, 119, 111, 114, 108, 100]);
    }

    #[test]
    fn test_restore() {
        test::test_restore(Little, &[], Vec::<String>::new, &Vec::<String>::new());
        test::test_restore(Big   , &[], Vec::<String>::new, &Vec::<String>::new());

        let expected = vec![String::from("hello"), String::from(" world")];
        let to_fill = vec![String::from("prev-content"), String::from("prev-content")];
        test::test_restore(Little, &[5, 0, 104, 101, 108, 108, 111, 6, 0, 32, 119, 111, 114, 108, 100], || to_fill.clone(), &expected);
        test::test_restore(Big   , &[0, 5, 104, 101, 108, 108, 111, 0, 6, 32, 119, 111, 114, 108, 100], || to_fill.clone(), &expected);
    }
}

