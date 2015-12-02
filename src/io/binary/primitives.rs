extern crate byteorder;
use self::byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};

use std::io::{Read, Write};
use std::mem;

use io::binary::traits::*;
use io::result::Result;

////////////////////////////////////////////////////////////////////////////////
// Implementations for the primitive types.

// Implementations without endian awareness.
macro_rules! binary_impl_int8 {
    ($ty:ty, $read_fn:ident, $write_fn:ident) => (
        impl Binary for $ty {
            fn is_streamable() -> bool { true }
            fn size_of_type() -> usize { mem::size_of::<Self>() }

            fn store(&self, writer: &mut Write, _swap: bool) -> Result<usize> {
                try!(writer.$write_fn(*self));
                Ok(mem::size_of::<Self>())
            }

            fn restore(&mut self, reader: &mut Read, _swap: bool) -> Result<usize> {
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

            fn store(&self, writer: &mut Write, swap: bool) -> Result<usize> {
                try!(if swap { writer.$write_fn::<BigEndian>(*self) }
                     else    { writer.$write_fn::<LittleEndian>(*self) });
                Ok(mem::size_of::<Self>())
            }

            fn restore(&mut self, reader: &mut Read, swap: bool) -> Result<usize> {
                *self = try!(if swap { reader.$read_fn::<BigEndian>() }
                             else    { reader.$read_fn::<LittleEndian>() });
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
    use std::mem::transmute;
    
    #[test]
    fn test_store() {
        test::test_store(false, &0x01u8, &[0x01]);
        test::test_store(true , &0x01u8, &[0x01]);
        test::test_store(false, &0x0123u16, &[0x23, 0x01]);
        test::test_store(true , &0x0123u16, &[0x01, 0x23]);
        test::test_store(false, &0x01234567u32, &[0x67, 0x45, 0x23, 0x01]);
        test::test_store(true , &0x01234567u32, &[0x01, 0x23, 0x45, 0x67]);
        test::test_store(false, &0x0123456789abcdefu64, &[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01]);
        test::test_store(true , &0x0123456789abcdefu64, &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);
        test::test_store(false, &0x01i8, &[0x01]);
        test::test_store(true , &0x01i8, &[0x01]);
        test::test_store(false, &0x0123i16, &[0x23, 0x01]);
        test::test_store(true , &0x0123i16, &[0x01, 0x23]);
        test::test_store(false, &0x01234567i32, &[0x67, 0x45, 0x23, 0x01]);
        test::test_store(true , &0x01234567i32, &[0x01, 0x23, 0x45, 0x67]);
        test::test_store(false, &0x0123456789abcdefi64, &[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01]);
        test::test_store(true , &0x0123456789abcdefi64, &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);

        let float32: f32 = unsafe { transmute(0x3e124925u32) };
        let float64: f64 = unsafe { transmute(0x3e12492589abcdefu64) };
        test::test_store(false, &float32, &[0x25, 0x49, 0x12, 0x3e]);
        test::test_store(true , &float32, &[0x3e, 0x12, 0x49, 0x25]);
        test::test_store(false, &float64, &[0xef, 0xcd, 0xab, 0x89, 0x25, 0x49, 0x12, 0x3e]);
        test::test_store(true , &float64, &[0x3e, 0x12, 0x49, 0x25, 0x89, 0xab, 0xcd, 0xef]);
    }

    #[test]
    fn test_restore() {
        test::test_restore(false, &[0x01], || 0, &0x01u8);
        test::test_restore(true , &[0x01], || 0, &0x01u8);
        test::test_restore(false, &[0x23, 0x01], || 0, &0x0123u16);
        test::test_restore(true , &[0x01, 0x23], || 0, &0x0123u16);
        test::test_restore(false, &[0x67, 0x45, 0x23, 0x01], || 0, &0x01234567u32);
        test::test_restore(true , &[0x01, 0x23, 0x45, 0x67], || 0, &0x01234567u32);
        test::test_restore(false, &[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01], || 0, &0x0123456789abcdefu64);
        test::test_restore(true , &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef], || 0, &0x0123456789abcdefu64);
        test::test_restore(false, &[0x01], || 0, &0x01i8);
        test::test_restore(true , &[0x01], || 0, &0x01i8);
        test::test_restore(false, &[0x23, 0x01], || 0, &0x0123i16);
        test::test_restore(true , &[0x01, 0x23], || 0, &0x0123i16);
        test::test_restore(false, &[0x67, 0x45, 0x23, 0x01], || 0, &0x01234567i32);
        test::test_restore(true , &[0x01, 0x23, 0x45, 0x67], || 0, &0x01234567i32);
        test::test_restore(false, &[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01], || 0, &0x0123456789abcdefi64);
        test::test_restore(true , &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef], || 0, &0x0123456789abcdefi64);

        let float32: f32 = unsafe { transmute(0x3e124925u32) };
        let float64: f64 = unsafe { transmute(0x3e12492589abcdefu64) };
        test::test_restore(false, &[0x25, 0x49, 0x12, 0x3e], || 0.0, &float32);
        test::test_restore(true , &[0x3e, 0x12, 0x49, 0x25], || 0.0, &float32);
        test::test_restore(false, &[0xef, 0xcd, 0xab, 0x89, 0x25, 0x49, 0x12, 0x3e], || 0.0, &float64);
        test::test_restore(true , &[0x3e, 0x12, 0x49, 0x25, 0x89, 0xab, 0xcd, 0xef], || 0.0, &float64);
    }
}

