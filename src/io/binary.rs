
extern crate byteorder;

use self::byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};
use std::mem;

use geometry::vector::*;
use io::result::{Error, Result};

const UNKNOWN_SIZE: usize = !0usize;


/// A trait for describing whether an object can be serialized to file.
///
/// The default implementation assumes a non-serializable type.
pub trait Binary {
    /// Whether values of type T are streamable.
    fn is_streamable() -> bool { false }

    /// Size of all values of type T, if determinable.
    fn size_of_type() -> usize { UNKNOWN_SIZE }

    /// Size of a specific value of type T, if determinable.
    fn size_of_value(&self) -> usize { Self::size_of_type() }

    /// Stores `self` into `writer`, `swap`ping byte order if specified. Returns the number of
    /// bytes written on success.
    fn store(&self, _writer: &mut Write, _swap: bool) -> Result<usize> {
        Err(Error::Unsupported)
    }

    /// Loads `self` from `reader`, `swap`ping byte order if specified. Returns the number of
    /// bytes read on success.
    fn restore(&mut self, _reader: &mut Read, _swap: bool) -> Result<usize> {
        Err(Error::Unsupported)
    }
}


#[cfg(test)]
mod test {
    use std::fmt::Debug;
    use std::io::Cursor;
    use io::binary::Binary;

    pub fn test_store<T: Binary>(swap: bool, value: &T, expected_bytes: &[u8]) {
        let mut buf = Vec::<u8>::new();
        assert_eq!(value.store(&mut buf, swap).unwrap(), expected_bytes.len());
        assert_eq!(buf, expected_bytes);
    }

    pub fn test_restore<F, T>(swap: bool, bytes: &[u8], new: F, expected_value: &T)
        where T: Binary + Debug + PartialEq<T>, F: FnOnce() -> T
    {
        let mut buf = Cursor::new(Vec::from(bytes));
        let mut value = new();
        assert_eq!(value.restore(&mut buf, swap).unwrap(), bytes.len());
        assert_eq!(&value, expected_value);
    }
}

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

////////////////////////////////////////////////////////////////////////////////
// Implementation for string.

impl Binary for String {
    fn is_streamable() -> bool { true }
    fn size_of_value(&self) -> usize { self.len() }

    fn store(&self, writer: &mut Write, swap: bool) -> Result<usize> {
        let len = self.len();
        if len > u16::max_value() as usize {
            return Err(Error::StringExceeds64k)
        }
        // TODO: OpenMesh has a bug where len is double-swapped.
        // Reproduce the bug for backward-compatibility with OpenMesh files?
        let len_size =try!((len as u16).store(writer, swap));
        try!(writer.write_all(self.as_bytes()));
        Ok(len + len_size)
    }

    fn restore(&mut self, reader: &mut Read, swap: bool) -> Result<usize> {
        let mut len = 0u16;
        // TODO: OpenMesh has a bug where len is double-swapped.
        // Reproduce the bug for backward-compatibility with OpenMesh files?
        let len_size = try!(len.restore(reader, swap));
        let len = len as usize;

        self.clear();
        self.reserve_exact(len);
        try!(reader.take(len as u64).read_to_string(self));
        Ok(len + len_size)
    }
}


#[cfg(test)]
mod test_string {
    use ::io::binary::test;

    #[test]
    fn test_store() {
        let s = String::from("hello");
        test::test_store(false, &s, &[5, 0, 104, 101, 108, 108, 111]);
        test::test_store(true , &s, &[0, 5, 104, 101, 108, 108, 111]);
    }

    #[test]
    fn test_restore() {
        test::test_restore(false, &[5, 0, 104, 101, 108, 108, 111], || String::from("prev-content"), &String::from("hello"));
        test::test_restore(true , &[0, 5, 104, 101, 108, 108, 111], || String::from("prev-content"), &String::from("hello"));
    }
}

////////////////////////////////////////////////////////////////////////////////
// Implementations for the geometry vector types.

// Implementations without endian awareness.
macro_rules! binary_impl_geometry_vec_int8 {
    ($vec_ty:ty, $N:expr, $read_fn:ident, $write_fn:ident) => (
        impl Binary for $vec_ty {
            fn is_streamable() -> bool { true }
            fn size_of_type() -> usize { mem::size_of::<Self>() }

            fn store(&self, writer: &mut Write, _swap: bool) -> Result<usize> {
                for i in 0..$N {
                    try!(writer.$write_fn(self[i]));
                }
                Ok(mem::size_of::<Self>())
            }

            fn restore(&mut self, reader: &mut Read, _swap: bool) -> Result<usize> {
                for i in 0..$N {
                    self[i] = try!(reader.$read_fn());
                }
                Ok(mem::size_of::<Self>())
            }
        }
    )
}

binary_impl_geometry_vec_int8!(Vec2c , 2, read_i8, write_i8);
binary_impl_geometry_vec_int8!(Vec2uc, 2, read_u8, write_u8);

binary_impl_geometry_vec_int8!(Vec3c , 3, read_i8, write_i8);
binary_impl_geometry_vec_int8!(Vec3uc, 3, read_u8, write_u8);

binary_impl_geometry_vec_int8!(Vec4c , 4, read_i8, write_i8);
binary_impl_geometry_vec_int8!(Vec4uc, 4, read_u8, write_u8);

binary_impl_geometry_vec_int8!(Vec6c , 6, read_i8, write_i8);
binary_impl_geometry_vec_int8!(Vec6uc, 6, read_u8, write_u8);


// Implementations with endian awareness.
macro_rules! binary_impl_geometry_vec {
    ($vec_ty:ty, $N:expr, $read_fn:ident, $write_fn:ident) => (
        impl Binary for $vec_ty {
            fn is_streamable() -> bool { true }
            fn size_of_type() -> usize { mem::size_of::<Self>() }

            fn store(&self, writer: &mut Write, swap: bool) -> Result<usize> {
                if swap { for i in 0..$N { try!(writer.$write_fn::<BigEndian>(self[i])); } }
                else    { for i in 0..$N { try!(writer.$write_fn::<LittleEndian>(self[i])); } }
                Ok(mem::size_of::<Self>())
            }

            fn restore(&mut self, reader: &mut Read, swap: bool) -> Result<usize> {
                if swap { for i in 0..$N { self[i] = try!(reader.$read_fn::<BigEndian>()); } }
                else    { for i in 0..$N { self[i] = try!(reader.$read_fn::<LittleEndian>()); } }
                Ok(mem::size_of::<Self>())
            }
        }
    )
}

binary_impl_geometry_vec!(Vec2s , 2, read_i16, write_i16);
binary_impl_geometry_vec!(Vec2i , 2, read_i32, write_i32);
binary_impl_geometry_vec!(Vec2us, 2, read_u16, write_u16);
binary_impl_geometry_vec!(Vec2ui, 2, read_u32, write_u32);
binary_impl_geometry_vec!(Vec2f , 2, read_f32, write_f32);
binary_impl_geometry_vec!(Vec2d , 2, read_f64, write_f64);

binary_impl_geometry_vec!(Vec3s , 3, read_i16, write_i16);
binary_impl_geometry_vec!(Vec3i , 3, read_i32, write_i32);
binary_impl_geometry_vec!(Vec3us, 3, read_u16, write_u16);
binary_impl_geometry_vec!(Vec3ui, 3, read_u32, write_u32);
binary_impl_geometry_vec!(Vec3f , 3, read_f32, write_f32);
binary_impl_geometry_vec!(Vec3d , 3, read_f64, write_f64);

binary_impl_geometry_vec!(Vec4s , 4, read_i16, write_i16);
binary_impl_geometry_vec!(Vec4i , 4, read_i32, write_i32);
binary_impl_geometry_vec!(Vec4us, 4, read_u16, write_u16);
binary_impl_geometry_vec!(Vec4ui, 4, read_u32, write_u32);
binary_impl_geometry_vec!(Vec4f , 4, read_f32, write_f32);
binary_impl_geometry_vec!(Vec4d , 4, read_f64, write_f64);

binary_impl_geometry_vec!(Vec6s , 6, read_i16, write_i16);
binary_impl_geometry_vec!(Vec6i , 6, read_i32, write_i32);
binary_impl_geometry_vec!(Vec6us, 6, read_u16, write_u16);
binary_impl_geometry_vec!(Vec6ui, 6, read_u32, write_u32);
binary_impl_geometry_vec!(Vec6f , 6, read_f32, write_f32);
binary_impl_geometry_vec!(Vec6d , 6, read_f64, write_f64);


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
        test::test_store(false, &Vec2uc::new(0x01u8, 0xac), &[0x01, 0xac]);
        test::test_store(true , &Vec2uc::new(0x01u8, 0xac), &[0x01, 0xac]);
        test::test_store(false,  &Vec2c::new(0x01i8, 0x7c), &[0x01, 0x7c]);
        test::test_store(true ,  &Vec2c::new(0x01i8, 0x7c), &[0x01, 0x7c]);
        test::test_store(false, &Vec2ui::new(0x01234567u32, 0x13570246), &[0x67, 0x45, 0x23, 0x01, 0x46, 0x02, 0x57, 0x13]);
        test::test_store(true , &Vec2ui::new(0x01234567u32, 0x13570246), &[0x01, 0x23, 0x45, 0x67, 0x13, 0x57, 0x02, 0x46]);
        test::test_store(false,  &Vec2i::new(0x01234567i32, 0x13570246), &[0x67, 0x45, 0x23, 0x01, 0x46, 0x02, 0x57, 0x13]);
        test::test_store(true ,  &Vec2i::new(0x01234567i32, 0x13570246), &[0x01, 0x23, 0x45, 0x67, 0x13, 0x57, 0x02, 0x46]);

        let vec2f: Vec2f = unsafe { transmute([0x3e124925u32, 0x6537851a]) };
        let vec2d: Vec2d = unsafe { transmute([0x3e12492589abcdefu64, 0x3fae1e1f3ac15743]) };
        test::test_store(false, &Vec2ui::new(0x01234567u32, 0x13570246), &[0x67, 0x45, 0x23, 0x01, 0x46, 0x02, 0x57, 0x13]);
        test::test_store(true , &Vec2ui::new(0x01234567u32, 0x13570246), &[0x01, 0x23, 0x45, 0x67, 0x13, 0x57, 0x02, 0x46]);
        test::test_store(false,  &Vec2i::new(0x01234567i32, 0x13570246), &[0x67, 0x45, 0x23, 0x01, 0x46, 0x02, 0x57, 0x13]);
        test::test_store(true ,  &Vec2i::new(0x01234567i32, 0x13570246), &[0x01, 0x23, 0x45, 0x67, 0x13, 0x57, 0x02, 0x46]);
        test::test_store(false, &vec2f, &[0x25, 0x49, 0x12, 0x3e, 0x1a, 0x85, 0x37, 0x65]);
        test::test_store(true , &vec2f, &[0x3e, 0x12, 0x49, 0x25, 0x65, 0x37, 0x85, 0x1a]);
        test::test_store(false, &vec2d, &[0xef, 0xcd, 0xab, 0x89, 0x25, 0x49, 0x12, 0x3e, 0x43, 0x57, 0xc1, 0x3a, 0x1f, 0x1e, 0xae, 0x3f]);
        test::test_store(true , &vec2d, &[0x3e, 0x12, 0x49, 0x25, 0x89, 0xab, 0xcd, 0xef, 0x3f, 0xae, 0x1e, 0x1f, 0x3a, 0xc1, 0x57, 0x43]);
    }

    #[test]
    fn test_restore() {
        test::test_restore(false, &[0x01, 0xac], Zero::zero, &Vec2uc::new(0x01u8, 0xac));
        test::test_restore(true , &[0x01, 0xac], Zero::zero, &Vec2uc::new(0x01u8, 0xac));
        test::test_restore(false, &[0x01, 0x7c], Zero::zero,  &Vec2c::new(0x01i8, 0x7c));
        test::test_restore(true , &[0x01, 0x7c], Zero::zero,  &Vec2c::new(0x01i8, 0x7c));
        test::test_restore(false, &[0x67, 0x45, 0x23, 0x01, 0x46, 0x02, 0x57, 0x13], Zero::zero, &Vec2ui::new(0x01234567u32, 0x13570246));
        test::test_restore(true , &[0x01, 0x23, 0x45, 0x67, 0x13, 0x57, 0x02, 0x46], Zero::zero, &Vec2ui::new(0x01234567u32, 0x13570246));
        test::test_restore(false, &[0x67, 0x45, 0x23, 0x01, 0x46, 0x02, 0x57, 0x13], Zero::zero,  &Vec2i::new(0x01234567i32, 0x13570246));
        test::test_restore(true , &[0x01, 0x23, 0x45, 0x67, 0x13, 0x57, 0x02, 0x46], Zero::zero,  &Vec2i::new(0x01234567i32, 0x13570246));

        let vec2f: Vec2f = unsafe { transmute([0x3e124925u32, 0x6537851a]) };
        let vec2d: Vec2d = unsafe { transmute([0x3e12492589abcdefu64, 0x3fae1e1f3ac15743]) };
        test::test_restore(false, &[0x67, 0x45, 0x23, 0x01, 0x46, 0x02, 0x57, 0x13], Zero::zero, &Vec2ui::new(0x01234567u32, 0x13570246));
        test::test_restore(true , &[0x01, 0x23, 0x45, 0x67, 0x13, 0x57, 0x02, 0x46], Zero::zero, &Vec2ui::new(0x01234567u32, 0x13570246));
        test::test_restore(false, &[0x67, 0x45, 0x23, 0x01, 0x46, 0x02, 0x57, 0x13], Zero::zero,  &Vec2i::new(0x01234567i32, 0x13570246));
        test::test_restore(true , &[0x01, 0x23, 0x45, 0x67, 0x13, 0x57, 0x02, 0x46], Zero::zero,  &Vec2i::new(0x01234567i32, 0x13570246));
        test::test_restore(false, &[0x25, 0x49, 0x12, 0x3e, 0x1a, 0x85, 0x37, 0x65], Zero::zero, &vec2f);
        test::test_restore(true , &[0x3e, 0x12, 0x49, 0x25, 0x65, 0x37, 0x85, 0x1a], Zero::zero, &vec2f);
        test::test_restore(false, &[0xef, 0xcd, 0xab, 0x89, 0x25, 0x49, 0x12, 0x3e, 0x43, 0x57, 0xc1, 0x3a, 0x1f, 0x1e, 0xae, 0x3f], Zero::zero, &vec2d);
        test::test_restore(true , &[0x3e, 0x12, 0x49, 0x25, 0x89, 0xab, 0xcd, 0xef, 0x3f, 0xae, 0x1e, 0x1f, 0x3a, 0xc1, 0x57, 0x43], Zero::zero, &vec2d);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Implementations for mesh status.

// TODO

////////////////////////////////////////////////////////////////////////////////
// Implementation for exact-size iterators of primitives.

// TODO

////////////////////////////////////////////////////////////////////////////////
// Implementation for exact-size iterators of strings.

// TODO

////////////////////////////////////////////////////////////////////////////////
// Implementation for exact-size iterators of bools.

// TODO
