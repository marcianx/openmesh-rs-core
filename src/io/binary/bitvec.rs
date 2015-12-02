use std::io::{Read, Write};

use io::binary::traits::*;
use io::result::Result;
use util::bitvec::BitVec;

////////////////////////////////////////////////////////////////////////////////
// Implementation for a BitVec.

impl Binary for BitVec {
    fn is_streamable() -> bool { true }
    fn size_of_value(&self) -> usize { self.as_bytes().len() }

    fn store(&self, writer: &mut Write, _swap: bool) -> Result<usize> {
        try!(writer.write_all(self.as_bytes()));
        Ok(self.size_of_value())
    }

    fn restore(&mut self, reader: &mut Read, _swap: bool) -> Result<usize> {
        try!(self.with_bytes_mut(|vec| reader.read_exact(vec)));
        Ok(self.size_of_value())
    }
}


#[cfg(test)]
mod test_status {
    use io::binary::test;
    use util::bitvec::BitVec;

    #[test]
    fn test_store() {
        test::test_store(false, &BitVec::new(), &[]);
        test::test_store(true , &BitVec::new(), &[]);

        let mut vec = BitVec::from_bytes(&[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0xe3]);
        vec.pop(); vec.pop();
        assert_eq!(vec.len(), 54);
        test::test_store(false, &vec, &[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23]);
        test::test_store(true , &vec, &[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23]);

        let vec = BitVec::from_bytes(&[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01]);

        assert_eq!(vec.len(), 64);
        test::test_store(false, &vec, &[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01]);
        test::test_store(true , &vec, &[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01]);
    }

    #[test]
    fn test_restore() {
        test::test_restore(false, &[], BitVec::new, &BitVec::new());
        test::test_restore(true , &[], BitVec::new, &BitVec::new());

        let input_stream = [0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23];
        let mut expected_vec = BitVec::from_bytes(&input_stream);
        test::test_restore(false, &input_stream, || BitVec::from_elem(56, false), &expected_vec);
        test::test_restore(true , &input_stream, || BitVec::from_elem(56, false), &expected_vec);

        expected_vec.pop(); expected_vec.pop();
        test::test_restore(false, &input_stream, || BitVec::from_elem(54, false), &expected_vec);
        test::test_restore(true , &input_stream, || BitVec::from_elem(54, false), &expected_vec);
    }
}

