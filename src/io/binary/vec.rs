// `io::binary::Binary` trait impls for
// * the statically-sized `Vec2`, `Vec3`, `Vec4`, `Vec6`

extern crate byteorder;
use self::byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};

use std::io::{Read, Write};
use std::mem;

use geometry::vector::*;
use io::binary::traits::*;
use io::result::Result;

// Implementations without endian awareness.
macro_rules! binary_impl_vec_int8 {
    ($vec_ty:ident, $item_ty:ty, $read_fn:ident, $write_fn:ident, $static_length: expr) => (
        impl Binary for $vec_ty<$item_ty> {
            fn is_streamable() -> bool { true }
            fn size_of_value(&self) -> usize { self.len() * <$item_ty as Binary>::size_of_type() }
            fn size_of_type() -> usize { $static_length }

            fn store(&self, writer: &mut Write, _swap: bool) -> Result<usize> {
                for val in self.as_ref().iter() {
                    try!(writer.$write_fn(*val));
                }
                Ok(mem::size_of::<Self>())
            }

            fn restore(&mut self, reader: &mut Read, _swap: bool) -> Result<usize> {
                for val in self.as_mut().iter_mut() {
                    *val = try!(reader.$read_fn());
                }
                Ok(mem::size_of::<Self>())
            }
        }
    )
}


// Implementations with endian awareness.
macro_rules! binary_impl_vec {
    ($vec_ty:ident, $item_ty:ty, $read_fn:ident, $write_fn:ident, $static_length: expr) => (
        impl Binary for $vec_ty<$item_ty> {
            fn is_streamable() -> bool { true }
            fn size_of_value(&self) -> usize { self.len() * <$item_ty as Binary>::size_of_type() }
            fn size_of_type() -> usize { $static_length }

            fn store(&self, writer: &mut Write, swap: bool) -> Result<usize> {
                let iter = self.as_ref().iter();
                if swap { for val in iter { try!(writer.$write_fn::<BigEndian>(*val)); } }
                else    { for val in iter { try!(writer.$write_fn::<LittleEndian>(*val)); } }
                Ok(mem::size_of::<Self>())
            }

            fn restore(&mut self, reader: &mut Read, swap: bool) -> Result<usize> {
                let iter_mut = self.as_mut().iter_mut();
                if swap { for val in iter_mut { *val = try!(reader.$read_fn::<BigEndian>()); } }
                else    { for val in iter_mut { *val = try!(reader.$read_fn::<LittleEndian>()); } }
                Ok(mem::size_of::<Self>())
            }
        }
    )
}

////////////////////////////////////////////////////////////////////////////////
// Implementations for the geometry vector types.

macro_rules! binary_impl_geometry_vec_int8 {
    ($vec_ty:ident, $item_ty:ty, $read_fn:ident, $write_fn:ident) => (
        binary_impl_vec_int8!($vec_ty, $item_ty, $read_fn, $write_fn, mem::size_of::<Self>());
    )
}

binary_impl_geometry_vec_int8!(Vec2, i8, read_i8, write_i8);
binary_impl_geometry_vec_int8!(Vec2, u8, read_u8, write_u8);

binary_impl_geometry_vec_int8!(Vec3, i8, read_i8, write_i8);
binary_impl_geometry_vec_int8!(Vec3, u8, read_u8, write_u8);

binary_impl_geometry_vec_int8!(Vec4, i8, read_i8, write_i8);
binary_impl_geometry_vec_int8!(Vec4, u8, read_u8, write_u8);

binary_impl_geometry_vec_int8!(Vec6, i8, read_i8, write_i8);
binary_impl_geometry_vec_int8!(Vec6, u8, read_u8, write_u8);


macro_rules! binary_impl_geometry_vec {
    ($vec_ty:ident, $item_ty:ty, $read_fn:ident, $write_fn:ident) => (
        binary_impl_vec!($vec_ty, $item_ty, $read_fn, $write_fn, mem::size_of::<Self>());
    )
}

binary_impl_geometry_vec!(Vec2, i16, read_i16, write_i16);
binary_impl_geometry_vec!(Vec2, i32, read_i32, write_i32);
binary_impl_geometry_vec!(Vec2, u16, read_u16, write_u16);
binary_impl_geometry_vec!(Vec2, u32, read_u32, write_u32);
binary_impl_geometry_vec!(Vec2, f32, read_f32, write_f32);
binary_impl_geometry_vec!(Vec2, f64, read_f64, write_f64);

binary_impl_geometry_vec!(Vec3, i16, read_i16, write_i16);
binary_impl_geometry_vec!(Vec3, i32, read_i32, write_i32);
binary_impl_geometry_vec!(Vec3, u16, read_u16, write_u16);
binary_impl_geometry_vec!(Vec3, u32, read_u32, write_u32);
binary_impl_geometry_vec!(Vec3, f32, read_f32, write_f32);
binary_impl_geometry_vec!(Vec3, f64, read_f64, write_f64);

binary_impl_geometry_vec!(Vec4, i16, read_i16, write_i16);
binary_impl_geometry_vec!(Vec4, i32, read_i32, write_i32);
binary_impl_geometry_vec!(Vec4, u16, read_u16, write_u16);
binary_impl_geometry_vec!(Vec4, u32, read_u32, write_u32);
binary_impl_geometry_vec!(Vec4, f32, read_f32, write_f32);
binary_impl_geometry_vec!(Vec4, f64, read_f64, write_f64);

binary_impl_geometry_vec!(Vec6, i16, read_i16, write_i16);
binary_impl_geometry_vec!(Vec6, i32, read_i32, write_i32);
binary_impl_geometry_vec!(Vec6, u16, read_u16, write_u16);
binary_impl_geometry_vec!(Vec6, u32, read_u32, write_u32);
binary_impl_geometry_vec!(Vec6, f32, read_f32, write_f32);
binary_impl_geometry_vec!(Vec6, f64, read_f64, write_f64);


#[cfg(test)]
mod test_geometry_vec {
    // Test only a subset of the impls above but all lines in the macros.
    extern crate num;
    use self::num::Zero;
    use io::binary::test;
    use geometry::vector::*;
    use std::mem::transmute;

    #[test]
    fn test_store() {
        test::test_store(false, &Vec2::<u8>::new(0x01u8, 0xac), &[0x01, 0xac]);
        test::test_store(true , &Vec2::<u8>::new(0x01u8, 0xac), &[0x01, 0xac]);
        test::test_store(false, &Vec2::<i8>::new(0x01i8, 0x7c), &[0x01, 0x7c]);
        test::test_store(true , &Vec2::<i8>::new(0x01i8, 0x7c), &[0x01, 0x7c]);
        test::test_store(false, &Vec2::<u32>::new(0x01234567u32, 0x13570246), &[0x67, 0x45, 0x23, 0x01, 0x46, 0x02, 0x57, 0x13]);
        test::test_store(true , &Vec2::<u32>::new(0x01234567u32, 0x13570246), &[0x01, 0x23, 0x45, 0x67, 0x13, 0x57, 0x02, 0x46]);
        test::test_store(false, &Vec2::<i32>::new(0x01234567i32, 0x13570246), &[0x67, 0x45, 0x23, 0x01, 0x46, 0x02, 0x57, 0x13]);
        test::test_store(true , &Vec2::<i32>::new(0x01234567i32, 0x13570246), &[0x01, 0x23, 0x45, 0x67, 0x13, 0x57, 0x02, 0x46]);

        let vec2f: Vec2<f32> = unsafe { transmute([0x3e124925u32, 0x6537851a]) };
        let vec2d: Vec2<f64> = unsafe { transmute([0x3e12492589abcdefu64, 0x3fae1e1f3ac15743]) };
        test::test_store(false, &vec2f, &[0x25, 0x49, 0x12, 0x3e, 0x1a, 0x85, 0x37, 0x65]);
        test::test_store(true , &vec2f, &[0x3e, 0x12, 0x49, 0x25, 0x65, 0x37, 0x85, 0x1a]);
        test::test_store(false, &vec2d, &[0xef, 0xcd, 0xab, 0x89, 0x25, 0x49, 0x12, 0x3e, 0x43, 0x57, 0xc1, 0x3a, 0x1f, 0x1e, 0xae, 0x3f]);
        test::test_store(true , &vec2d, &[0x3e, 0x12, 0x49, 0x25, 0x89, 0xab, 0xcd, 0xef, 0x3f, 0xae, 0x1e, 0x1f, 0x3a, 0xc1, 0x57, 0x43]);
    }

    #[test]
    fn test_restore() {
        test::test_restore(false, &[0x01, 0xac], Zero::zero, &Vec2::<u8>::new(0x01u8, 0xac));
        test::test_restore(true , &[0x01, 0xac], Zero::zero, &Vec2::<u8>::new(0x01u8, 0xac));
        test::test_restore(false, &[0x01, 0x7c], Zero::zero, &Vec2::<i8>::new(0x01i8, 0x7c));
        test::test_restore(true , &[0x01, 0x7c], Zero::zero, &Vec2::<i8>::new(0x01i8, 0x7c));
        test::test_restore(false, &[0x67, 0x45, 0x23, 0x01, 0x46, 0x02, 0x57, 0x13], Zero::zero, &Vec2::<u32>::new(0x01234567u32, 0x13570246));
        test::test_restore(true , &[0x01, 0x23, 0x45, 0x67, 0x13, 0x57, 0x02, 0x46], Zero::zero, &Vec2::<u32>::new(0x01234567u32, 0x13570246));
        test::test_restore(false, &[0x67, 0x45, 0x23, 0x01, 0x46, 0x02, 0x57, 0x13], Zero::zero, &Vec2::<i32>::new(0x01234567i32, 0x13570246));
        test::test_restore(true , &[0x01, 0x23, 0x45, 0x67, 0x13, 0x57, 0x02, 0x46], Zero::zero, &Vec2::<i32>::new(0x01234567i32, 0x13570246));

        let vec2f: Vec2<f32> = unsafe { transmute([0x3e124925u32, 0x6537851a]) };
        let vec2d: Vec2<f64> = unsafe { transmute([0x3e12492589abcdefu64, 0x3fae1e1f3ac15743]) };
        test::test_restore(false, &[0x25, 0x49, 0x12, 0x3e, 0x1a, 0x85, 0x37, 0x65], Zero::zero, &vec2f);
        test::test_restore(true , &[0x3e, 0x12, 0x49, 0x25, 0x65, 0x37, 0x85, 0x1a], Zero::zero, &vec2f);
        test::test_restore(false, &[0xef, 0xcd, 0xab, 0x89, 0x25, 0x49, 0x12, 0x3e, 0x43, 0x57, 0xc1, 0x3a, 0x1f, 0x1e, 0xae, 0x3f], Zero::zero, &vec2d);
        test::test_restore(true , &[0x3e, 0x12, 0x49, 0x25, 0x89, 0xab, 0xcd, 0xef, 0x3f, 0xae, 0x1e, 0x1f, 0x3a, 0xc1, 0x57, 0x43], Zero::zero, &vec2d);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Implementation for vectors of primitives.

// TODO

