extern crate num;
use self::num::Zero;

use std::io::{Read, Write};
use std::mem;

use io::binary::traits::*;
use io::result::Result;
use mesh::status::{FlagBits, Status};

////////////////////////////////////////////////////////////////////////////////
// Implementations for mesh status.

impl Binary for Status {
    fn is_streamable() -> bool { true }
    fn size_of_type() -> usize { mem::size_of::<Self>() }

    fn store(&self, writer: &mut Write, swap: bool) -> Result<usize> {
        self.bits().store(writer, swap)
    }

    fn restore(&mut self, reader: &mut Read, swap: bool) -> Result<usize> {
        let mut bits: FlagBits = Zero::zero();
        let len = try!(bits.restore(reader, swap));
        *self = Status::from_bits_truncate(bits);
        Ok(len)
    }
}


#[cfg(test)]
mod test_status {
    use io::binary::test;
    use mesh::status;
    
    #[test]
    fn test_store() {
        let flags1 = status::DELETED | status::LOCKED | status::HIDDEN | status::TAGGED | status::TAGGED2;
        test::test_store(false, &flags1, &[0x6b, 0x00, 0x00, 0x00]);
        test::test_store(true , &flags1, &[0x00, 0x00, 0x00, 0x6b]);
        let flags2 = status::SELECTED | status::FEATURE | status::FIXED_NONMANIFOLD;
        test::test_store(false, &flags2, &[0x94, 0x00, 0x00, 0x00]);
        test::test_store(true , &flags2, &[0x00, 0x00, 0x00, 0x94]);
    }

    #[test]
    fn test_restore() {
        let flags1 = status::DELETED | status::LOCKED | status::HIDDEN | status::TAGGED | status::TAGGED2;
        test::test_restore(false, &[0x6b, 0xff, 0xff, 0xff], status::Status::empty, &flags1);
        test::test_restore(true , &[0xff, 0xff, 0xff, 0x6b], status::Status::empty, &flags1);
        let flags2 = status::SELECTED | status::FEATURE | status::FIXED_NONMANIFOLD;
        test::test_restore(false, &[0x94, 0xff, 0xff, 0xff], status::Status::empty, &flags2);
        test::test_restore(true , &[0xff, 0xff, 0xff, 0x94], status::Status::empty, &flags2);
    }
}

