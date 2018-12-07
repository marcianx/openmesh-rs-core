extern crate byteorder;
use self::byteorder::{ReadBytesExt, WriteBytesExt};

use std::io::{Read, Write};
use std::mem;

use crate::io::binary::traits::{Binary, ByteOrder};
use crate::io::result::Result;

////////////////////////////////////////////////////////////////////////////////
// Implementations for the primitive types.

// Implementations without endian awareness.
macro_rules! binary_impl_int8 {
    ($ty:ty, $read_fn:ident, $write_fn:ident) => (
        impl Binary for $ty {
            fn is_streamable() -> bool { true }
            fn size_of_type() -> usize { mem::size_of::<Self>() }

            fn store_endian<B: ByteOrder>(&self, writer: &mut Write) -> Result<usize> {
                writer.$write_fn(*self)?;
                Ok(mem::size_of::<Self>())
            }

            fn restore_endian<B: ByteOrder>(&mut self, reader: &mut Read) -> Result<usize> {
                let value = reader.$read_fn()?;
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
                writer.$write_fn::<B>(*self)?;
                Ok(mem::size_of::<Self>())
            }

            fn restore_endian<B: ByteOrder>(&mut self, reader: &mut Read) -> Result<usize> {
                *self = reader.$read_fn::<B>()?;
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
mod test {
    use crate::io::binary::test;
    use crate::io::binary::traits::Endian::{Big, Little};
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


#[cfg(test)]
mod test_vec {
    use crate::io::binary::test;
    use crate::io::binary::traits::Endian::{Big, Little};
    use std::mem::transmute;

    #[test]
    fn test_store() {
        test::test_store(Little, &Vec::<u8>::new(), &[]);
        test::test_store(Big   , &Vec::<u8>::new(), &[]);
        test::test_store(Little, &Vec::<u32>::new(), &[]);
        test::test_store(Big   , &Vec::<u32>::new(), &[]);
        test::test_store(Little, &vec![0x01u8, 0xac], &[0x01, 0xac]);
        test::test_store(Big   , &vec![0x01u8, 0xac], &[0x01, 0xac]);
        test::test_store(Little, &vec![0x01i8, 0x7c], &[0x01, 0x7c]);
        test::test_store(Big   , &vec![0x01i8, 0x7c], &[0x01, 0x7c]);
        test::test_store(Little, &vec![0x01234567u32, 0x13570246], &[0x67, 0x45, 0x23, 0x01, 0x46, 0x02, 0x57, 0x13]);
        test::test_store(Big   , &vec![0x01234567u32, 0x13570246], &[0x01, 0x23, 0x45, 0x67, 0x13, 0x57, 0x02, 0x46]);
        test::test_store(Little, &vec![0x01234567i32, 0x13570246], &[0x67, 0x45, 0x23, 0x01, 0x46, 0x02, 0x57, 0x13]);
        test::test_store(Big   , &vec![0x01234567i32, 0x13570246], &[0x01, 0x23, 0x45, 0x67, 0x13, 0x57, 0x02, 0x46]);

        let vecf = Vec::<f32>::from(&unsafe { transmute::<_, [f32; 2]>([0x3e124925u32, 0x6537851a]) }[..]);
        let vecd = Vec::<f64>::from(&unsafe { transmute::<_, [f64; 2]>([0x3e12492589abcdefu64, 0x3fae1e1f3ac15743]) }[..]);
        test::test_store(Little, &vecf, &[0x25, 0x49, 0x12, 0x3e, 0x1a, 0x85, 0x37, 0x65]);
        test::test_store(Big   , &vecf, &[0x3e, 0x12, 0x49, 0x25, 0x65, 0x37, 0x85, 0x1a]);
        test::test_store(Little, &vecd, &[0xef, 0xcd, 0xab, 0x89, 0x25, 0x49, 0x12, 0x3e, 0x43, 0x57, 0xc1, 0x3a, 0x1f, 0x1e, 0xae, 0x3f]);
        test::test_store(Big   , &vecd, &[0x3e, 0x12, 0x49, 0x25, 0x89, 0xab, 0xcd, 0xef, 0x3f, 0xae, 0x1e, 0x1f, 0x3a, 0xc1, 0x57, 0x43]);
    }

    #[test]
    fn test_restore() {
        test::test_restore(Little, &[], Vec::new, &Vec::<u8>::new());
        test::test_restore(Big   , &[], Vec::new, &Vec::<u8>::new());
        test::test_restore(Little, &[], Vec::new, &Vec::<u32>::new());
        test::test_restore(Big   , &[], Vec::new, &Vec::<u32>::new());
        test::test_restore(Little, &[0x01, 0xac], || vec![0, 0], &vec![0x01u8, 0xac]);
        test::test_restore(Big   , &[0x01, 0xac], || vec![0, 0], &vec![0x01u8, 0xac]);
        test::test_restore(Little, &[0x01, 0x7c], || vec![0, 0], &vec![0x01i8, 0x7c]);
        test::test_restore(Big   , &[0x01, 0x7c], || vec![0, 0], &vec![0x01i8, 0x7c]);
        test::test_restore(Little, &[0x67, 0x45, 0x23, 0x01, 0x46, 0x02, 0x57, 0x13], || vec![0, 0], &vec![0x01234567u32, 0x13570246]);
        test::test_restore(Big   , &[0x01, 0x23, 0x45, 0x67, 0x13, 0x57, 0x02, 0x46], || vec![0, 0], &vec![0x01234567u32, 0x13570246]);
        test::test_restore(Little, &[0x67, 0x45, 0x23, 0x01, 0x46, 0x02, 0x57, 0x13], || vec![0, 0], &vec![0x01234567i32, 0x13570246]);
        test::test_restore(Big   , &[0x01, 0x23, 0x45, 0x67, 0x13, 0x57, 0x02, 0x46], || vec![0, 0], &vec![0x01234567i32, 0x13570246]);

        let vecf = Vec::<f32>::from(&unsafe { transmute::<_, [f32; 2]>([0x3e124925u32, 0x6537851a]) }[..]);
        let vecd = Vec::<f64>::from(&unsafe { transmute::<_, [f64; 2]>([0x3e12492589abcdefu64, 0x3fae1e1f3ac15743]) }[..]);
        test::test_restore(Little, &[0x25, 0x49, 0x12, 0x3e, 0x1a, 0x85, 0x37, 0x65], || vec![0.0, 0.0], &vecf);
        test::test_restore(Big   , &[0x3e, 0x12, 0x49, 0x25, 0x65, 0x37, 0x85, 0x1a], || vec![0.0, 0.0], &vecf);
        test::test_restore(Little, &[0xef, 0xcd, 0xab, 0x89, 0x25, 0x49, 0x12, 0x3e, 0x43, 0x57, 0xc1, 0x3a, 0x1f, 0x1e, 0xae, 0x3f], || vec![0.0, 0.0], &vecd);
        test::test_restore(Big   , &[0x3e, 0x12, 0x49, 0x25, 0x89, 0xab, 0xcd, 0xef, 0x3f, 0xae, 0x1e, 0x1f, 0x3a, 0xc1, 0x57, 0x43], || vec![0.0, 0.0], &vecd);
    }
}
