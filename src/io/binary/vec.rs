// `io::binary::Binary` trait impls for the following arrays of primitives:
// * the static-length `Vec2`, `Vec3`, `Vec4`, `Vec6`
// * the dynamic-length `Vec` from the standard library

extern crate byteorder;
use self::byteorder::{ReadBytesExt, WriteBytesExt};

use std::io::{Read, Write};
use std::mem;

use geometry::vector::*;
use io::binary::traits::{Binary, ByteOrder, UNKNOWN_SIZE};
use io::result::Result;

////////////////////////////////////////////////////////////////////////////////

// Local trait to share the macro below between the static-length and dynamic-length variants.
trait ToIterable {
    type Output;
    fn iterable(&self) -> &Self::Output;
    fn iterable_mut(&mut self) -> &mut Self::Output;
}


// Implementations without endian awareness.
macro_rules! binary_impl_vec_int8 {
    ($vec_ty:ident, $item_ty:ty, $read_fn:ident, $write_fn:ident, $static_length: expr) => (
        impl Binary for $vec_ty<$item_ty> {
            fn is_streamable() -> bool { true }
            fn size_of_value(&self) -> usize { self.len() * <$item_ty as Binary>::size_of_type() }
            fn size_of_type() -> usize { $static_length }

            fn store_endian<B: ByteOrder>(&self, writer: &mut Write) -> Result<usize> {
                for val in self.iterable().iter() {
                    try!(writer.$write_fn(*val));
                }
                Ok(self.size_of_value())
            }

            fn restore_endian<B: ByteOrder>(&mut self, reader: &mut Read) -> Result<usize> {
                for val in self.iterable_mut().iter_mut() {
                    *val = try!(reader.$read_fn());
                }
                Ok(self.size_of_value())
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

            fn store_endian<B: ByteOrder>(&self, writer: &mut Write) -> Result<usize> {
                for val in self.iterable().iter() {
                    try!(writer.$write_fn::<B>(*val));
                }
                Ok(self.size_of_value())
            }

            fn restore_endian<B: ByteOrder>(&mut self, reader: &mut Read) -> Result<usize> {
                for val in self.iterable_mut().iter_mut() {
                    *val = try!(reader.$read_fn::<B>());
                }
                Ok(self.size_of_value())
            }
        }
    )
}

////////////////////////////////////////////////////////////////////////////////
// Implementations for the geometry vector types.

macro_rules! to_iterable_impl {
    ($vec_ty:ident, $N: expr) => (
        impl<T> ToIterable for $vec_ty<T> {
            type Output = [T; $N];
            fn iterable(&self) -> &Self::Output { self.as_ref() }
            fn iterable_mut(&mut self) -> &mut Self::Output { self.as_mut() }
        }
    )
}
to_iterable_impl!(Vec2, 2);
to_iterable_impl!(Vec3, 3);
to_iterable_impl!(Vec4, 4);
to_iterable_impl!(Vec6, 6);


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
    use io::binary::traits::Endian::{Big, Little};
    use geometry::vector::*;
    use std::mem::transmute;

    #[test]
    fn test_store() {
        test::test_store(Little, &Vec2::<u8>::new(0x01u8, 0xac), &[0x01, 0xac]);
        test::test_store(Big   , &Vec2::<u8>::new(0x01u8, 0xac), &[0x01, 0xac]);
        test::test_store(Little, &Vec2::<i8>::new(0x01i8, 0x7c), &[0x01, 0x7c]);
        test::test_store(Big   , &Vec2::<i8>::new(0x01i8, 0x7c), &[0x01, 0x7c]);
        test::test_store(Little, &Vec2::<u32>::new(0x01234567u32, 0x13570246), &[0x67, 0x45, 0x23, 0x01, 0x46, 0x02, 0x57, 0x13]);
        test::test_store(Big   , &Vec2::<u32>::new(0x01234567u32, 0x13570246), &[0x01, 0x23, 0x45, 0x67, 0x13, 0x57, 0x02, 0x46]);
        test::test_store(Little, &Vec2::<i32>::new(0x01234567i32, 0x13570246), &[0x67, 0x45, 0x23, 0x01, 0x46, 0x02, 0x57, 0x13]);
        test::test_store(Big   , &Vec2::<i32>::new(0x01234567i32, 0x13570246), &[0x01, 0x23, 0x45, 0x67, 0x13, 0x57, 0x02, 0x46]);

        let vec2f: Vec2<f32> = unsafe { transmute([0x3e124925u32, 0x6537851a]) };
        let vec2d: Vec2<f64> = unsafe { transmute([0x3e12492589abcdefu64, 0x3fae1e1f3ac15743]) };
        test::test_store(Little, &vec2f, &[0x25, 0x49, 0x12, 0x3e, 0x1a, 0x85, 0x37, 0x65]);
        test::test_store(Big   , &vec2f, &[0x3e, 0x12, 0x49, 0x25, 0x65, 0x37, 0x85, 0x1a]);
        test::test_store(Little, &vec2d, &[0xef, 0xcd, 0xab, 0x89, 0x25, 0x49, 0x12, 0x3e, 0x43, 0x57, 0xc1, 0x3a, 0x1f, 0x1e, 0xae, 0x3f]);
        test::test_store(Big   , &vec2d, &[0x3e, 0x12, 0x49, 0x25, 0x89, 0xab, 0xcd, 0xef, 0x3f, 0xae, 0x1e, 0x1f, 0x3a, 0xc1, 0x57, 0x43]);
    }

    #[test]
    fn test_restore() {
        test::test_restore(Little, &[0x01, 0xac], Zero::zero, &Vec2::<u8>::new(0x01u8, 0xac));
        test::test_restore(Big   , &[0x01, 0xac], Zero::zero, &Vec2::<u8>::new(0x01u8, 0xac));
        test::test_restore(Little, &[0x01, 0x7c], Zero::zero, &Vec2::<i8>::new(0x01i8, 0x7c));
        test::test_restore(Big   , &[0x01, 0x7c], Zero::zero, &Vec2::<i8>::new(0x01i8, 0x7c));
        test::test_restore(Little, &[0x67, 0x45, 0x23, 0x01, 0x46, 0x02, 0x57, 0x13], Zero::zero, &Vec2::<u32>::new(0x01234567u32, 0x13570246));
        test::test_restore(Big   , &[0x01, 0x23, 0x45, 0x67, 0x13, 0x57, 0x02, 0x46], Zero::zero, &Vec2::<u32>::new(0x01234567u32, 0x13570246));
        test::test_restore(Little, &[0x67, 0x45, 0x23, 0x01, 0x46, 0x02, 0x57, 0x13], Zero::zero, &Vec2::<i32>::new(0x01234567i32, 0x13570246));
        test::test_restore(Big   , &[0x01, 0x23, 0x45, 0x67, 0x13, 0x57, 0x02, 0x46], Zero::zero, &Vec2::<i32>::new(0x01234567i32, 0x13570246));

        let vec2f: Vec2<f32> = unsafe { transmute([0x3e124925u32, 0x6537851a]) };
        let vec2d: Vec2<f64> = unsafe { transmute([0x3e12492589abcdefu64, 0x3fae1e1f3ac15743]) };
        test::test_restore(Little, &[0x25, 0x49, 0x12, 0x3e, 0x1a, 0x85, 0x37, 0x65], Zero::zero, &vec2f);
        test::test_restore(Big   , &[0x3e, 0x12, 0x49, 0x25, 0x65, 0x37, 0x85, 0x1a], Zero::zero, &vec2f);
        test::test_restore(Little, &[0xef, 0xcd, 0xab, 0x89, 0x25, 0x49, 0x12, 0x3e, 0x43, 0x57, 0xc1, 0x3a, 0x1f, 0x1e, 0xae, 0x3f], Zero::zero, &vec2d);
        test::test_restore(Big   , &[0x3e, 0x12, 0x49, 0x25, 0x89, 0xab, 0xcd, 0xef, 0x3f, 0xae, 0x1e, 0x1f, 0x3a, 0xc1, 0x57, 0x43], Zero::zero, &vec2d);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Implementation for vectors of primitives.

impl<T> ToIterable for Vec<T> {
    type Output = Self;
    fn iterable(&self) -> &Self::Output { self }
    fn iterable_mut(&mut self) -> &mut Self::Output { self }
}


macro_rules! binary_impl_std_vec_int8 {
    ($vec_ty:ident, $item_ty:ty, $read_fn:ident, $write_fn:ident) => (
        binary_impl_vec_int8!($vec_ty, $item_ty, $read_fn, $write_fn, UNKNOWN_SIZE);
    )
}

binary_impl_std_vec_int8!(Vec, i8, read_i8, write_i8);
binary_impl_std_vec_int8!(Vec, u8, read_u8, write_u8);


macro_rules! binary_impl_std_vec {
    ($vec_ty:ident, $item_ty:ty, $read_fn:ident, $write_fn:ident) => (
        binary_impl_vec!($vec_ty, $item_ty, $read_fn, $write_fn, UNKNOWN_SIZE);
    )
}

binary_impl_std_vec!(Vec, i16, read_i16, write_i16);
binary_impl_std_vec!(Vec, i32, read_i32, write_i32);
binary_impl_std_vec!(Vec, u16, read_u16, write_u16);
binary_impl_std_vec!(Vec, u32, read_u32, write_u32);
binary_impl_std_vec!(Vec, f32, read_f32, write_f32);
binary_impl_std_vec!(Vec, f64, read_f64, write_f64);


#[cfg(test)]
mod test_std_vec {
    use io::binary::test;
    use io::binary::traits::Endian::{Big, Little};
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

