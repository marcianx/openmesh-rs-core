// `io::binary::Binary` trait impls for the static-length geometry vecs
// `Vec2`, `Vec3`, `Vec4`, `Vec6`

use std::io::{Read, Write};

use crate::geometry::vector::{Vec2, Vec3, Vec4, Vec6};
use crate::io::binary::traits::{Binary, ByteOrder};
use crate::io::result::Result;

////////////////////////////////////////////////////////////////////////////////
// Implementations for the geometry vector types.

macro_rules! binary_impl_vec {
    ($vec_ty:ident, $item_ty:ty, $read_fn:ident, $write_fn:ident) => {
        impl Binary for $vec_ty<$item_ty> {
            fn is_streamable() -> bool {
                true
            }

            fn size_of_value(&self) -> usize {
                self.len() * <$item_ty as Binary>::size_of_type()
            }

            fn size_of_type() -> usize {
                ::std::mem::size_of::<Self>()
            }

            fn store_endian<B: ByteOrder>(&self, writer: &mut dyn Write) -> Result<usize> {
                for val in self.as_ref().iter() {
                    (*val).store_endian::<B>(writer)?;
                }
                Ok(self.size_of_value())
            }

            fn restore_endian<B: ByteOrder>(&mut self, reader: &mut dyn Read) -> Result<usize> {
                for val in self.as_mut().iter_mut() {
                    (*val).restore_endian::<B>(reader)?;
                }
                Ok(self.size_of_value())
            }
        }
    };
}

binary_impl_vec!(Vec2, i8, read_i8, write_i8);
binary_impl_vec!(Vec2, u8, read_u8, write_u8);

binary_impl_vec!(Vec3, i8, read_i8, write_i8);
binary_impl_vec!(Vec3, u8, read_u8, write_u8);

binary_impl_vec!(Vec4, i8, read_i8, write_i8);
binary_impl_vec!(Vec4, u8, read_u8, write_u8);

binary_impl_vec!(Vec6, i8, read_i8, write_i8);
binary_impl_vec!(Vec6, u8, read_u8, write_u8);

binary_impl_vec!(Vec2, i16, read_i16, write_i16);
binary_impl_vec!(Vec2, i32, read_i32, write_i32);
binary_impl_vec!(Vec2, u16, read_u16, write_u16);
binary_impl_vec!(Vec2, u32, read_u32, write_u32);
binary_impl_vec!(Vec2, f32, read_f32, write_f32);
binary_impl_vec!(Vec2, f64, read_f64, write_f64);

binary_impl_vec!(Vec3, i16, read_i16, write_i16);
binary_impl_vec!(Vec3, i32, read_i32, write_i32);
binary_impl_vec!(Vec3, u16, read_u16, write_u16);
binary_impl_vec!(Vec3, u32, read_u32, write_u32);
binary_impl_vec!(Vec3, f32, read_f32, write_f32);
binary_impl_vec!(Vec3, f64, read_f64, write_f64);

binary_impl_vec!(Vec4, i16, read_i16, write_i16);
binary_impl_vec!(Vec4, i32, read_i32, write_i32);
binary_impl_vec!(Vec4, u16, read_u16, write_u16);
binary_impl_vec!(Vec4, u32, read_u32, write_u32);
binary_impl_vec!(Vec4, f32, read_f32, write_f32);
binary_impl_vec!(Vec4, f64, read_f64, write_f64);

binary_impl_vec!(Vec6, i16, read_i16, write_i16);
binary_impl_vec!(Vec6, i32, read_i32, write_i32);
binary_impl_vec!(Vec6, u16, read_u16, write_u16);
binary_impl_vec!(Vec6, u32, read_u32, write_u32);
binary_impl_vec!(Vec6, f32, read_f32, write_f32);
binary_impl_vec!(Vec6, f64, read_f64, write_f64);

#[cfg(test)]
mod test {
    // Test only a subset of the impls above but all lines in the macros.
    use crate::geometry::vector::*;
    use crate::io::binary::test;
    use crate::io::binary::traits::Endian::{Big, Little};
    use num::Zero;
    use std::mem::transmute;

    #[test]
    fn test_store() {
        test::test_store(Little, &Vec2::<u8>::new(0x01u8, 0xac), &[0x01, 0xac]);
        test::test_store(Big, &Vec2::<u8>::new(0x01u8, 0xac), &[0x01, 0xac]);
        test::test_store(Little, &Vec2::<i8>::new(0x01i8, 0x7c), &[0x01, 0x7c]);
        test::test_store(Big, &Vec2::<i8>::new(0x01i8, 0x7c), &[0x01, 0x7c]);
        test::test_store(
            Little,
            &Vec2::<u32>::new(0x01234567u32, 0x13570246),
            &[0x67, 0x45, 0x23, 0x01, 0x46, 0x02, 0x57, 0x13],
        );
        test::test_store(
            Big,
            &Vec2::<u32>::new(0x01234567u32, 0x13570246),
            &[0x01, 0x23, 0x45, 0x67, 0x13, 0x57, 0x02, 0x46],
        );
        test::test_store(
            Little,
            &Vec2::<i32>::new(0x01234567i32, 0x13570246),
            &[0x67, 0x45, 0x23, 0x01, 0x46, 0x02, 0x57, 0x13],
        );
        test::test_store(
            Big,
            &Vec2::<i32>::new(0x01234567i32, 0x13570246),
            &[0x01, 0x23, 0x45, 0x67, 0x13, 0x57, 0x02, 0x46],
        );

        let vec2f: Vec2<f32> = unsafe { transmute([0x3e124925u32, 0x6537851a]) };
        let vec2d: Vec2<f64> = unsafe { transmute([0x3e12492589abcdefu64, 0x3fae1e1f3ac15743]) };
        test::test_store(
            Little,
            &vec2f,
            &[0x25, 0x49, 0x12, 0x3e, 0x1a, 0x85, 0x37, 0x65],
        );
        test::test_store(
            Big,
            &vec2f,
            &[0x3e, 0x12, 0x49, 0x25, 0x65, 0x37, 0x85, 0x1a],
        );
        test::test_store(
            Little,
            &vec2d,
            &[
                0xef, 0xcd, 0xab, 0x89, 0x25, 0x49, 0x12, 0x3e, 0x43, 0x57, 0xc1, 0x3a, 0x1f, 0x1e,
                0xae, 0x3f,
            ],
        );
        test::test_store(
            Big,
            &vec2d,
            &[
                0x3e, 0x12, 0x49, 0x25, 0x89, 0xab, 0xcd, 0xef, 0x3f, 0xae, 0x1e, 0x1f, 0x3a, 0xc1,
                0x57, 0x43,
            ],
        );
    }

    #[test]
    fn test_restore() {
        test::test_restore(
            Little,
            &[0x01, 0xac],
            Zero::zero,
            &Vec2::<u8>::new(0x01u8, 0xac),
        );
        test::test_restore(
            Big,
            &[0x01, 0xac],
            Zero::zero,
            &Vec2::<u8>::new(0x01u8, 0xac),
        );
        test::test_restore(
            Little,
            &[0x01, 0x7c],
            Zero::zero,
            &Vec2::<i8>::new(0x01i8, 0x7c),
        );
        test::test_restore(
            Big,
            &[0x01, 0x7c],
            Zero::zero,
            &Vec2::<i8>::new(0x01i8, 0x7c),
        );
        test::test_restore(
            Little,
            &[0x67, 0x45, 0x23, 0x01, 0x46, 0x02, 0x57, 0x13],
            Zero::zero,
            &Vec2::<u32>::new(0x01234567u32, 0x13570246),
        );
        test::test_restore(
            Big,
            &[0x01, 0x23, 0x45, 0x67, 0x13, 0x57, 0x02, 0x46],
            Zero::zero,
            &Vec2::<u32>::new(0x01234567u32, 0x13570246),
        );
        test::test_restore(
            Little,
            &[0x67, 0x45, 0x23, 0x01, 0x46, 0x02, 0x57, 0x13],
            Zero::zero,
            &Vec2::<i32>::new(0x01234567i32, 0x13570246),
        );
        test::test_restore(
            Big,
            &[0x01, 0x23, 0x45, 0x67, 0x13, 0x57, 0x02, 0x46],
            Zero::zero,
            &Vec2::<i32>::new(0x01234567i32, 0x13570246),
        );

        let vec2f: Vec2<f32> = unsafe { transmute([0x3e124925u32, 0x6537851a]) };
        let vec2d: Vec2<f64> = unsafe { transmute([0x3e12492589abcdefu64, 0x3fae1e1f3ac15743]) };
        test::test_restore(
            Little,
            &[0x25, 0x49, 0x12, 0x3e, 0x1a, 0x85, 0x37, 0x65],
            Zero::zero,
            &vec2f,
        );
        test::test_restore(
            Big,
            &[0x3e, 0x12, 0x49, 0x25, 0x65, 0x37, 0x85, 0x1a],
            Zero::zero,
            &vec2f,
        );
        test::test_restore(
            Little,
            &[
                0xef, 0xcd, 0xab, 0x89, 0x25, 0x49, 0x12, 0x3e, 0x43, 0x57, 0xc1, 0x3a, 0x1f, 0x1e,
                0xae, 0x3f,
            ],
            Zero::zero,
            &vec2d,
        );
        test::test_restore(
            Big,
            &[
                0x3e, 0x12, 0x49, 0x25, 0x89, 0xab, 0xcd, 0xef, 0x3f, 0xae, 0x1e, 0x1f, 0x3a, 0xc1,
                0x57, 0x43,
            ],
            Zero::zero,
            &vec2d,
        );
    }
}
