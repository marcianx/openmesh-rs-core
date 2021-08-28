use num::Zero;

use std::io::{Read, Write};
use std::mem;

use crate::io::binary::traits::{Binary, ByteOrder};
use crate::io::result::Result;
use crate::mesh::status::{FlagBits, Status};

////////////////////////////////////////////////////////////////////////////////
// Implementations for mesh status.

impl Binary for Status {
    fn is_streamable() -> bool {
        true
    }

    fn size_of_type() -> usize {
        mem::size_of::<Self>()
    }

    fn store_endian<B: ByteOrder>(&self, writer: &mut dyn Write) -> Result<usize> {
        self.bits().store_endian::<B>(writer)
    }

    fn restore_endian<B: ByteOrder>(&mut self, reader: &mut dyn Read) -> Result<usize> {
        let mut bits: FlagBits = Zero::zero();
        let len = bits.restore_endian::<B>(reader)?;
        *self = Status::from_bits_truncate(bits);
        Ok(len)
    }
}

#[cfg(test)]
mod test {
    use crate::io::binary::test;
    use crate::io::binary::traits::Endian::{Big, Little};
    use crate::mesh::status::Status;

    #[test]
    fn test_store() {
        let flags1 =
            Status::DELETED | Status::LOCKED | Status::HIDDEN | Status::TAGGED | Status::TAGGED2;
        test::test_store(Little, &flags1, &[0x6b, 0x00, 0x00, 0x00]);
        test::test_store(Big, &flags1, &[0x00, 0x00, 0x00, 0x6b]);
        let flags2 = Status::SELECTED | Status::FEATURE | Status::FIXED_NON_MANIFOLD;
        test::test_store(Little, &flags2, &[0x94, 0x00, 0x00, 0x00]);
        test::test_store(Big, &flags2, &[0x00, 0x00, 0x00, 0x94]);
    }

    #[rustfmt::skip]
    #[test]
    fn test_restore() {
        let flags1 =
            Status::DELETED | Status::LOCKED | Status::HIDDEN | Status::TAGGED | Status::TAGGED2;
        test::test_restore(Little, &[0x6b, 0xff, 0xff, 0xff], Status::empty, &flags1);
        test::test_restore(Big   , &[0xff, 0xff, 0xff, 0x6b], Status::empty, &flags1);

        let flags2 = Status::SELECTED | Status::FEATURE | Status::FIXED_NON_MANIFOLD;
        test::test_restore(Little, &[0x94, 0xff, 0xff, 0xff], Status::empty, &flags2);
        test::test_restore(Big   , &[0xff, 0xff, 0xff, 0x94], Status::empty, &flags2);
    }
}
