//! See documentation for the `Quadric` struct.

use crate::geometry::math::Real;
use crate::geometry::vector::{Vec3, Vec4};
use num::traits::{One, Zero};
use std::ops::{Add, Mul};

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
/// Represents a quadric function `v -> (v' M v)` applicable to 4-vectors `v` (or a 3-vectors with
/// implicit 1 in the fourth coordinate) where `M` is a symmetric matrix.
///
/// The implementation uses the upper-triangular representation of the symmetric matrix `M`.
/// The indexes and symbols corresponding to this part are as follows
/// ```text
/// [0 1 2 3]   [a b c d]   [xx xy xz xw]
/// [  4 5 6] = [  e f g] = [   yy yz yw]
/// [    7 8]   [    h i]   [      zz zw]
/// [      9]   [      j]   [         ww]
/// ```
pub struct Quadric<Scalar>([Scalar; 10]);

impl<Scalar: Real> Quadric<Scalar> {
    /// Quadric in the symmetric upper-triangular matrix form.
    pub fn from_matrix(m: [Scalar; 10]) -> Self {
        Quadric(m)
    }

    /// Quadric representing distance from plane defined by ax + by + cz + d = 0.
    #[rustfmt::skip]
    pub fn from_plane_coeffs(a: Scalar, b: Scalar, c: Scalar, d: Scalar) -> Self {
        Quadric([
            a * a, a * b, a * c, a * d,
                   b * b, b * c, b * d,
                          c * c, c * d,
                                 d * d,
        ])
    }

    /// Quadric representing distance from plane defined by normal n and point pt.
    pub fn from_plane(n: &Vec3<Scalar>, pt: &Vec3<Scalar>) -> Self {
        Self::from_plane_coeffs(n[0], n[1], n[2], -n.dot(&pt))
    }

    /// Sets quadric in the symmetric upper-triangular matrix form.
    pub fn set(&mut self, m: [Scalar; 10]) {
        self.0 = m;
    }

    /// Sets quadric to represent the squared distance to a point.
    #[rustfmt::skip]
    pub fn set_distance_to_point(&mut self, pt: Vec3<Scalar>) {
        self.0 = [
            One::one(), Zero::zero(), Zero::zero(), -pt[0],
                          One::one(), Zero::zero(), -pt[1],
                                        One::one(), -pt[2],
                                         pt.norm_squared(),
        ];
    }

    /// Sets quadric to represent the squared distance to a plane represented by
    /// ax + by + cz + d = 0.
    pub fn set_distance_to_plane_coeffs(&mut self, a: Scalar, b: Scalar, c: Scalar, d: Scalar) {
        self.0 = Self::from_plane_coeffs(a, b, c, d).0;
    }

    /// Sets quadric to represent the squared distance to a plane that is normal to vector n and
    /// goes through a point pt.
    pub fn set_distance_to_plane(&mut self, n: &Vec3<Scalar>, pt: &Vec3<Scalar>) {
        self.0 = Self::from_plane(n, pt).0;
    }

    /// Sets the quadric to all zeros.
    pub fn clear(&mut self) {
        self.0 = [Zero::zero(); 10]
    }
}

#[rustfmt::skip]
impl<Scalar: Real> Quadric<Scalar> {
    #[allow(missing_docs)] pub fn a(&self) -> Scalar { self.0[0] }
    #[allow(missing_docs)] pub fn b(&self) -> Scalar { self.0[1] }
    #[allow(missing_docs)] pub fn c(&self) -> Scalar { self.0[2] }
    #[allow(missing_docs)] pub fn d(&self) -> Scalar { self.0[3] }
    #[allow(missing_docs)] pub fn e(&self) -> Scalar { self.0[4] }
    #[allow(missing_docs)] pub fn f(&self) -> Scalar { self.0[5] }
    #[allow(missing_docs)] pub fn g(&self) -> Scalar { self.0[6] }
    #[allow(missing_docs)] pub fn h(&self) -> Scalar { self.0[7] }
    #[allow(missing_docs)] pub fn i(&self) -> Scalar { self.0[8] }
    #[allow(missing_docs)] pub fn j(&self) -> Scalar { self.0[9] }
    #[allow(missing_docs)] pub fn xx(&self) -> Scalar { self.0[0] }
    #[allow(missing_docs)] pub fn xy(&self) -> Scalar { self.0[1] }
    #[allow(missing_docs)] pub fn xz(&self) -> Scalar { self.0[2] }
    #[allow(missing_docs)] pub fn xw(&self) -> Scalar { self.0[3] }
    #[allow(missing_docs)] pub fn yy(&self) -> Scalar { self.0[4] }
    #[allow(missing_docs)] pub fn yz(&self) -> Scalar { self.0[5] }
    #[allow(missing_docs)] pub fn yw(&self) -> Scalar { self.0[6] }
    #[allow(missing_docs)] pub fn zz(&self) -> Scalar { self.0[7] }
    #[allow(missing_docs)] pub fn zw(&self) -> Scalar { self.0[8] }
    #[allow(missing_docs)] pub fn ww(&self) -> Scalar { self.0[9] }
}

impl<'a, 'b, Scalar> Quadric<Scalar>
where
    Scalar: Mul<Output = Scalar> + Add<Output = Scalar> + One + Copy + Real,
{
    /// Computes `v -> v' M v`.
    #[rustfmt::skip]
    fn eval_coords(&self, x: Scalar, y: Scalar, z: Scalar, w: Scalar) -> Scalar {
        let two = Scalar::from_f32(2.0f32);
        let ref m = self.0;
        m[0] * x*x + m[1] * two*x*y + m[2] * two*x*z + m[3] * two*x*w
                   + m[4]      *y*y + m[5] * two*y*z + m[6] * two*y*w
                                    + m[7] *     z*z + m[8] * two*z*w
                                                     + m[9] *     w*w
    }

    /// Computes `v -> v' M v` for 4-vectors.
    pub fn eval(&self, v: &Vec4<Scalar>) -> Scalar {
        self.eval_coords(v[0], v[1], v[2], v[3])
    }

    /// Computes `v -> v' M v` for 3-vectors, assuming the last coordinate to be 1.
    pub fn eval3(&self, v: &Vec3<Scalar>) -> Scalar {
        self.eval_coords(v[0], v[1], v[2], One::one())
    }
}

impl<Scalar: Real + Mul<Output = Scalar>> Quadric<Scalar> {
    /// Multiplies a quadric in-place by a scalar.
    pub fn mul_by(&mut self, s: Scalar) {
        for val in self.0.iter_mut() {
            *val = *val * s;
        }
    }
}

impl<Scalar: Add + Copy> Add<Quadric<Scalar>> for Quadric<Scalar>
where
    <Scalar as Add>::Output: Copy,
{
    type Output = Quadric<<Scalar as Add>::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Quadric([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
            self.0[3] + rhs.0[3],
            self.0[4] + rhs.0[4],
            self.0[5] + rhs.0[5],
            self.0[6] + rhs.0[6],
            self.0[7] + rhs.0[7],
            self.0[8] + rhs.0[8],
            self.0[9] + rhs.0[9],
        ])
    }
}

// Multiplication by scalar.
impl<'a, Scalar: Mul + Copy> Mul<Scalar> for &'a Quadric<Scalar>
where
    <Scalar as Mul>::Output: Copy,
{
    type Output = Quadric<<Scalar as Mul>::Output>;

    fn mul(self, s: Scalar) -> Self::Output {
        Quadric([
            self.0[0] * s,
            self.0[1] * s,
            self.0[2] * s,
            self.0[3] * s,
            self.0[4] * s,
            self.0[5] * s,
            self.0[6] * s,
            self.0[7] * s,
            self.0[8] * s,
            self.0[9] * s,
        ])
    }
}

// Matrix multiplication by vec4.
impl<'a, 'b, Scalar> Mul<&'b Vec4<Scalar>> for &'a Quadric<Scalar>
where
    Scalar: Mul<Output = Scalar> + Add<Output = Scalar> + Copy,
{
    type Output = Vec4<Scalar>;

    fn mul(self, v: &'b Vec4<Scalar>) -> Self::Output {
        Vec4::new(
            self.0[0] * v[0] + self.0[1] * v[1] + self.0[2] * v[2] + self.0[3] * v[3],
            self.0[1] * v[0] + self.0[4] * v[1] + self.0[5] * v[2] + self.0[6] * v[3],
            self.0[2] * v[0] + self.0[5] * v[1] + self.0[7] * v[2] + self.0[8] * v[3],
            self.0[3] * v[0] + self.0[6] * v[1] + self.0[8] * v[2] + self.0[9] * v[3],
        )
    }
}

// Matrix multiplication by vec4.
impl<'a, Scalar> Mul<Vec4<Scalar>> for &'a Quadric<Scalar>
where
    Scalar: Mul<Output = Scalar> + Add<Output = Scalar> + Copy,
{
    type Output = Vec4<Scalar>;

    #[allow(clippy::op_ref)]
    fn mul(self, v: Vec4<Scalar>) -> Self::Output {
        self * &v
    }
}

impl<Scalar: Zero + Copy> Zero for Quadric<Scalar> {
    /// Zero quadric.
    fn zero() -> Self {
        Quadric([Zero::zero(); 10])
    }

    /// Whether the quadric is zero.
    fn is_zero(&self) -> bool {
        self.0.iter().all(|v| v.is_zero())
    }
}

/// Alias for Quadric<f32>.
pub type Quadricf = Quadric<f32>;
/// Alias for Quadric<f64>.
pub type Quadricd = Quadric<f64>;

#[cfg(test)]
mod test {
    use crate::geometry::quadric::{Quadric, Quadricd};
    use crate::geometry::vector::{Vec3, Vec4};
    use num::traits::Zero;

    #[test]
    fn test_init() {
        println!("{:?}", Quadricd::zero());
        println!(
            "{:?}",
            Quadric::from_matrix([1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0])
        );
        println!("{:?}", Quadric::from_plane_coeffs(1.0, 2.0, 3.0, 4.0));
        println!(
            "{:?}",
            Quadric::from_plane(&Vec3::new(1.0, 2.0, 3.0), &Vec3::new(-1.0, 0.0, -1.0))
        );
    }

    #[test]
    fn test_clear() {
        let mut q: Quadricd = Quadric::from_plane_coeffs(1.0, 2.0, 3.0, 4.0);
        q.clear();
        assert!(q == Zero::zero());
    }

    #[test]
    fn test_scalar_mul() {
        let p: Quadricd = Zero::zero();
        let mut q: Quadricd = Zero::zero();
        q.mul_by(4.0);
        assert!(p == q);
        assert!(p == &p * 4.0);
    }

    #[test]
    fn test_vector_mul() {
        let q = Quadric::from_matrix([1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0]);
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(&q * v, v);
        assert_eq!(&q * &v, v);
    }

    #[test]
    fn test_eval() {
        let q = Quadric::from_matrix([1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0]);
        let v3 = Vec3::new(1.0, 2.0, 3.0);
        let v4 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(q.eval3(&v3), v3.norm_squared() + 1.0);
        assert_eq!(q.eval(&v4), v4.norm_squared());
    }
}
