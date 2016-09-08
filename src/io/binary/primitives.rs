extern crate byteorder;
use self::byteorder::{ReadBytesExt, WriteBytesExt};

use std::io::{Read, Write};
use std::mem;

use io::binary::traits::{Binary, ByteOrder};
use io::result::Result;

////////////////////////////////////////////////////////////////////////////////
// Implementations for the primitive types.

// Implementations without endian awareness.
macro_rules! binary_impl_int8 {
    ($ty:ty, $read_fn:ident, $write_fn:ident) => (
        impl Binary for $ty {
            fn is_streamable() -> bool { true }
            fn size_of_type() -> usize { mem::size_of::<Self>() }

            fn store_endian<B: ByteOrder>(&self, writer: &mut Write) -> Result<usize> {
                try!(writer.$write_fn(*self));
                Ok(mem::size_of::<Self>())
            }

            fn restore_endian<B: ByteOrder>(&mut self, reader: &mut Read) -> Result<usize> {
                let value = try!(reader.$read_fn());
                *self = value;
                Ok(mem::size_of::<Self>())
            }
        }
    )
}

binary_impl_int8!(i8, read_i8, write_i8);
binary_impl_int8!(u8, read_u8, write_u8);


// Implementations with endian awareness.
macro_rules! binary_impl_primitive {
    ($ty:ty, $read_fn:ident, $write_fn:ident) => (
        impl Binary for $ty {
            fn is_streamable() -> bool { true }
            fn size_of_type() -> usize { mem::size_of::<Self>() }

            fn store_endian<B: ByteOrder>(&self, writer: &mut Write) -> Result<usize> {
                try!(writer.$write_fn::<B>(*self));
                Ok(mem::size_of::<Self>())
            }

            fn restore_endian<B: ByteOrder>(&mut self, reader: &mut Read) -> Result<usize> {
                *self = try!(reader.$read_fn::<B>());
                Ok(mem::size_of::<Self>())
            }
        }
    )
}

binary_impl_primitive!(i16, read_i16, write_i16);
binary_impl_primitive!(i32, read_i32, write_i32);
binary_impl_primitive!(i64, read_i64, write_i64);
binary_impl_primitive!(u16, read_u16, write_u16);
binary_impl_primitive!(u32, read_u32, write_u32);
binary_impl_primitive!(u64, read_u64, write_u64);
binary_impl_primitive!(f32, read_f32, write_f32);
binary_impl_primitive!(f64, read_f64, write_f64);


#[cfg(test)]
mod test_primitives {
    use io::binary::test;
    use io::binary::traits::Endian::{Big, Little};
    use std::mem::transmute;
    
    #[test]
    fn test_store() {
        test::test_store(Little, &0x01u8, &[0x01]);
        test::test_store(Big   , &0x01u8, &[0x01]);
        test::test_store(Little, &0x0123u16, &[0x23, 0x01]);
        test::test_store(Big   , &0x0123u16, &[0x01, 0x23]);
        test::test_store(Little, &0x01234567u32, &[0x67, 0x45, 0x23, 0x01]);
        test::test_store(Big   , &0x01234567u32, &[0x01, 0x23, 0x45, 0x67]);
        test::test_store(Little, &0x0123456789abcdefu64, &[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01]);
        test::test_store(Big   , &0x0123456789abcdefu64, &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);
        test::test_store(Little, &0x01i8, &[0x01]);
        test::test_store(Big   , &0x01i8, &[0x01]);
        test::test_store(Little, &0x0123i16, &[0x23, 0x01]);
        test::test_store(Big   , &0x0123i16, &[0x01, 0x23]);
        test::test_store(Little, &0x01234567i32, &[0x67, 0x45, 0x23, 0x01]);
        test::test_store(Big   , &0x01234567i32, &[0x01, 0x23, 0x45, 0x67]);
        test::test_store(Little, &0x0123456789abcdefi64, &[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01]);
        test::test_store(Big   , &0x0123456789abcdefi64, &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);

        let float32: f32 = unsafe { transmute(0x3e124925u32) };
        let float64: f64 = unsafe { transmute(0x3e12492589abcdefu64) };
        test::test_store(Little, &float32, &[0x25, 0x49, 0x12, 0x3e]);
        test::test_store(Big   , &float32, &[0x3e, 0x12, 0x49, 0x25]);
        test::test_store(Little, &float64, &[0xef, 0xcd, 0xab, 0x89, 0x25, 0x49, 0x12, 0x3e]);
        test::test_store(Big   , &float64, &[0x3e, 0x12, 0x49, 0x25, 0x89, 0xab, 0xcd, 0xef]);
    }

    #[test]
    fn test_restore() {
        test::test_restore(Little, &[0x01], || 0, &0x01u8);
        test::test_restore(Big   , &[0x01], || 0, &0x01u8);
        test::test_restore(Little, &[0x23, 0x01], || 0, &0x0123u16);
        test::test_restore(Big   , &[0x01, 0x23], || 0, &0x0123u16);
        test::test_restore(Little, &[0x67, 0x45, 0x23, 0x01], || 0, &0x01234567u32);
        test::test_restore(Big   , &[0x01, 0x23, 0x45, 0x67], || 0, &0x01234567u32);
        test::test_restore(Little, &[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01], || 0, &0x0123456789abcdefu64);
        test::test_restore(Big   , &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef], || 0, &0x0123456789abcdefu64);
        test::test_restore(Little, &[0x01], || 0, &0x01i8);
        test::test_restore(Big   , &[0x01], || 0, &0x01i8);
        test::test_restore(Little, &[0x23, 0x01], || 0, &0x0123i16);
        test::test_restore(Big   , &[0x01, 0x23], || 0, &0x0123i16);
        test::test_restore(Little, &[0x67, 0x45, 0x23, 0x01], || 0, &0x01234567i32);
        test::test_restore(Big   , &[0x01, 0x23, 0x45, 0x67], || 0, &0x01234567i32);
        test::test_restore(Little, &[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01], || 0, &0x0123456789abcdefi64);
        test::test_restore(Big   , &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef], || 0, &0x0123456789abcdefi64);

        let float32: f32 = unsafe { transmute(0x3e124925u32) };
        let float64: f64 = unsafe { transmute(0x3e12492589abcdefu64) };
        test::test_restore(Little, &[0x25, 0x49, 0x12, 0x3e], || 0.0, &float32);
        test::test_restore(Big   , &[0x3e, 0x12, 0x49, 0x25], || 0.0, &float32);
        test::test_restore(Little, &[0xef, 0xcd, 0xab, 0x89, 0x25, 0x49, 0x12, 0x3e], || 0.0, &float64);
        test::test_restore(Big   , &[0x3e, 0x12, 0x49, 0x25, 0x89, 0xab, 0xcd, 0xef], || 0.0, &float64);
    }
}

