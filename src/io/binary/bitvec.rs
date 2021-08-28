use std::io::{Read, Write};

use crate::io::binary::traits::{Binary, ByteOrder};
use crate::io::result::Result;
use crate::util::bitvec::BitVec;

////////////////////////////////////////////////////////////////////////////////
// Implementation for a BitVec.

impl Binary for BitVec {
    fn is_streamable() -> bool {
        true
    }

    fn size_of_value(&self) -> usize {
        self.as_bytes().len()
    }

    fn store_endian<B: ByteOrder>(&self, writer: &mut dyn Write) -> Result<usize> {
        writer.write_all(self.as_bytes())?;
        Ok(self.size_of_value())
    }

    fn restore_endian<B: ByteOrder>(&mut self, reader: &mut dyn Read) -> Result<usize> {
        self.with_bytes_mut(|vec| reader.read_exact(vec))?;
        Ok(self.size_of_value())
    }
}

#[cfg(test)]
mod test {
    use crate::io::binary::test;
    use crate::io::binary::traits::Endian::{Big, Little};
    use crate::util::bitvec::BitVec;

    #[test]
    fn test_store() {
        test::test_store(Little, &BitVec::new(), &[]);
        test::test_store(Big, &BitVec::new(), &[]);

        let with_full_bytes = [0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0xe3];
        let with_partial_byte = [0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23];
        let mut vec = BitVec::from_bytes(&with_full_bytes);
        vec.pop();
        vec.pop();
        assert_eq!(vec.len(), 54);
        test::test_store(Little, &vec, &with_partial_byte);
        test::test_store(Big, &vec, &with_partial_byte);

        let with_full_bytes = [0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01];
        let vec = BitVec::from_bytes(&with_full_bytes);
        assert_eq!(vec.len(), 64);
        test::test_store(Little, &vec, &with_full_bytes);
        test::test_store(Big, &vec, &with_full_bytes);
    }

    #[rustfmt::skip]
    #[test]
    fn test_restore() {
        test::test_restore(Little, &[], BitVec::new, &BitVec::new());
        test::test_restore(Big   , &[], BitVec::new, &BitVec::new());

        let input_stream = [0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23];
        let mut expected_vec = BitVec::from_bytes(&input_stream);
        test::test_restore(Little, &input_stream, || BitVec::from_elem(56, false), &expected_vec);
        test::test_restore(Big   , &input_stream, || BitVec::from_elem(56, true ), &expected_vec);

        expected_vec.pop();
        expected_vec.pop();
        test::test_restore(Little, &input_stream, || BitVec::from_elem(54, false), &expected_vec);
        test::test_restore(Big   , &input_stream, || BitVec::from_elem(54, true ), &expected_vec);
    }
}
