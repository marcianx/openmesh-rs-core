extern crate nalgebra as na;

pub use self::na::{Vec2, Vec3, Vec4, Vec6, Dot, Norm};

/// 2-byte signed vector
type Vec2c = Vec2<i8>;
/// 2-byte unsigned vector
type Vec2uc = Vec2<u8>;
/// 2-short signed vector
type Vec2s = Vec2<i16>;
/// 2-short unsigned vector
type Vec2us = Vec2<u16>;
/// 2-int signed vector
type Vec2i = Vec2<i32>;
/// 2-int unsigned vector
type Vec2ui = Vec2<u32>;
/// 2-float vector
type Vec2f = Vec2<f32>;
/// 2-double vector
type Vec2d = Vec2<f64>;

/// 3-byte signed vector
type Vec3c = Vec3<i8>;
/// 3-byte unsigned vector
type Vec3uc = Vec3<u8>;
/// 3-short signed vector
type Vec3s = Vec3<i16>;
/// 3-short unsigned vector
type Vec3us = Vec3<u16>;
/// 3-int signed vector
type Vec3i = Vec3<i32>;
/// 3-int unsigned vector
type Vec3ui = Vec3<u32>;
/// 3-float vector
type Vec3f = Vec3<f32>;
/// 3-double vector
type Vec3d = Vec3<f64>;

/// 4-byte signed vector
type Vec4c = Vec4<i8>;
/// 4-byte unsigned vector
type Vec4uc = Vec4<u8>;
/// 4-short signed vector
type Vec4s = Vec4<i16>;
/// 4-short unsigned vector
type Vec4us = Vec4<u16>;
/// 4-int signed vector
type Vec4i = Vec4<i32>;
/// 4-int unsigned vector
type Vec4ui = Vec4<u32>;
/// 4-float vector
type Vec4f = Vec4<f32>;
/// 4-double vector
type Vec4d = Vec4<f64>;

/// 6-byte signed vector
type Vec6c = Vec6<i8>;
/// 6-byte unsigned vector
type Vec6uc = Vec6<u8>;
/// 6-short signed vector
type Vec6s = Vec6<i16>;
/// 6-short unsigned vector
type Vec6us = Vec6<u16>;
/// 6-int signed vector
type Vec6i = Vec6<i32>;
/// 6-int unsigned vector
type Vec6ui = Vec6<u32>;
/// 6-float vector
type Vec6f = Vec6<f32>;
/// 6-double vector
type Vec6d = Vec6<f64>;
