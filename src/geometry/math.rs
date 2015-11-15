extern crate num;
extern crate nalgebra as na;

pub trait BaseFloat : self::na::BaseFloat {
    /// Returns the default epsilon value for floating point equality tests.
    #[inline(always)]
    fn eps() -> Self;
    /// From float.
    #[inline(always)]
    fn from_f32(val: f32) -> Self;
}

impl BaseFloat for f32 {
    fn eps() -> Self { 1e-05f32 }
    fn from_f32(val: f32) -> Self { val }
}

impl BaseFloat for f64 {
    fn eps() -> Self { 1e-09f64 }
    fn from_f32(val: f32) -> Self { val as f64 }
}

pub trait FloatCompare : BaseFloat {
    /// Returns whether self is within the provided epsilon from 0.
    #[inline(always)]
    fn is_zero_eps(self, eps: Self) -> bool {
        self.abs() < eps
    }
    /// Returns whether self and other are within the provided epsilon.
    #[inline(always)]
    fn is_eq_eps(self, other: Self, eps: Self) -> bool {
        (self - other).is_zero_eps(eps)
    }
    /// Returns whether self is greater than other and not equal to within epsilon.
    #[inline(always)]
    fn is_gt_eps(self, other: Self, eps: Self) -> bool {
        // self >= other + eps
        self - other >= eps
    }
    /// Returns whether self is greater than other or equal to it within epsilon.
    #[inline(always)]
    fn is_ge_eps(self, other: Self, eps: Self) -> bool {
        // self > other - eps
        self - other > -eps
    }
    /// Returns whether self is greater than other and not equal to within epsilon.
    #[inline(always)]
    fn is_lt_eps(self, other: Self, eps: Self) -> bool {
        // other >= self + eps
        other - self >= eps
    }
    /// Returns whether self is greater than other or equal to it within epsilon.
    #[inline(always)]
    fn is_le_eps(self, other: Self, eps: Self) -> bool {
        // other > self - eps
        other - self > -eps
    }

    /// Checks for equality within the default epsilon.
    #[inline(always)] fn is_zero(self) -> bool { self.is_zero_eps(Self::eps()) }
    /// Checks for equality within the default epsilon.
    #[inline(always)] fn is_eq(self, other: Self) -> bool { self.is_eq_eps(other, Self::eps()) }
    /// Returns whether self is greater than other and not equal to within the default epsilon.
    #[inline(always)] fn is_gt(self, other: Self) -> bool { self.is_gt_eps(other, Self::eps()) }
    /// Returns whether self is greater than other or equal to it within the default epsilon.
    #[inline(always)] fn is_ge(self, other: Self) -> bool { self.is_ge_eps(other, Self::eps()) }
    /// Returns whether self is greater than other and not equal to within the default epsilon.
    #[inline(always)] fn is_lt(self, other: Self) -> bool { self.is_lt_eps(other, Self::eps()) }
    /// Returns whether self is greater than other or equal to it within the default epsilon.
    #[inline(always)] fn is_le(self, other: Self) -> bool { self.is_le_eps(other, Self::eps()) }
}

/// Adds 2pi to the angle if it is negative.
#[inline(always)]
pub fn positive_angle<T: BaseFloat>(ang: T) -> T {
    if ang >= T::zero() {
        ang
    } else {
        ang + T::two_pi()
    }
}

/// Computes the angle in (-pi, pi] based on the provided y (`self`) and `x`-coordinates of the
/// vector from the origin.
#[inline(always)]
pub fn angle2<T: BaseFloat>(y: T, x: T) -> T { T::atan2(y, x) }

/// Computes the angle in [0, 2pi) based on the provided y (`self`) and `x`-coordinates of the
/// vector from the origin.
pub fn positive_angle2<T: BaseFloat>(y: T, x: T) -> T { positive_angle(angle2(y, x)) }
