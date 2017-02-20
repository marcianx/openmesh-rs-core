//! See documentation for the `Quadric` struct.

extern crate num;
use self::num::traits::{One,Zero};
use std::ops::{Add, Mul};
use geometry::math::BaseFloat;
use geometry::vector::{Vec3, Vec4, Dot, Norm};

#[repr(C)]
#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
/// Represents a quadric function `v -> (v' M v)` applicable to 4-vectors `v` (or a 3-vectors with
/// implicit 1 in the fourth coordinate) where `M` is a symmetric matrix.
///
/// The implementation uses the upper-triangular representation of the symmetric matrix `M`.
/// The indexes and symbols corresponding to this part are as follows
/// ```
/// [0 1 2 3]   [a b c d]   [xx xy xz xw]
/// [  4 5 6] = [  e f g] = [   yy yz yw]
/// [    7 8]   [    h i]   [      zz zw]
/// [      9]   [      j]   [         ww]
/// ```
pub struct Quadric<Scalar> where Scalar: Copy {
  m: [Scalar; 10]
}

impl<Scalar: BaseFloat> Quadric<Scalar> {
    /// Quadric in the symmetric upper-triangular matrix form.
    pub fn from_matrix(m: [Scalar; 10]) -> Self {
        Quadric { m: m }
    }
    /// Quadric representing distance from plane defined by ax + by + cz + d = 0.
    pub fn from_plane_coeffs(a: Scalar, b: Scalar, c: Scalar, d: Scalar) -> Self {
        Quadric {
            m: [
                /* */ a * a, a * b,  a * c,  a * d,
                /* */        b * b,  b * c,  b * d,
                /* */                c * c,  c * d,
                /* */                        d * d,
            ]
        }
    }
    /// Quadric representing distance from plane defined by normal n and point pt.
    pub fn from_plane(n: &Vec3<Scalar>, pt: &Vec3<Scalar>) -> Self {
        Self::from_plane_coeffs(n[0], n[1], n[2], -n.dot(&pt))
    }

    /// Sets quadric in the symmetric upper-triangular matrix form.
    pub fn set(&mut self, m: [Scalar; 10]) {
        self.m = m;
    }
    /// Sets quadric to represent the squared distance to a point.
    pub fn set_distance_to_point(&mut self, pt: Vec3<Scalar>) {
        self.m = [
            /* */ One::one(), Zero::zero(), Zero::zero(), -pt[0],
            /* */               One::one(), Zero::zero(), -pt[1],
            /* */                             One::one(), -pt[2],
            /* */                                    pt.sqnorm(),
        ];
    }
    /// Sets quadric to represent the squared distance to a plane represented by
    /// ax + by + cz + d = 0.
    pub fn set_distance_to_plane_coeffs(&mut self, a: Scalar, b: Scalar, c: Scalar, d: Scalar) {
        self.m = Self::from_plane_coeffs(a, b, c, d).m;
    }
    /// Sets quadric to represent the squared distance to a plane that is normal to vector n and
    /// goes through a point pt.
    pub fn set_distance_to_plane(&mut self, n: &Vec3<Scalar>, pt: &Vec3<Scalar>) {
        self.m = Self::from_plane(n, pt).m;
    }
    /// Sets the quadric to all zeros.
    pub fn clear(&mut self) {
        self.m = [Zero::zero(); 10]
    }

    #[allow(missing_docs)]
    pub fn a(&self)  -> Scalar { self.m[0] }
    #[allow(missing_docs)]
    pub fn b(&self)  -> Scalar { self.m[1] }
    #[allow(missing_docs)]
    pub fn c(&self)  -> Scalar { self.m[2] }
    #[allow(missing_docs)]
    pub fn d(&self)  -> Scalar { self.m[3] }
    #[allow(missing_docs)]
    pub fn e(&self)  -> Scalar { self.m[4] }
    #[allow(missing_docs)]
    pub fn f(&self)  -> Scalar { self.m[5] }
    #[allow(missing_docs)]
    pub fn g(&self)  -> Scalar { self.m[6] }
    #[allow(missing_docs)]
    pub fn h(&self)  -> Scalar { self.m[7] }
    #[allow(missing_docs)]
    pub fn i(&self)  -> Scalar { self.m[8] }
    #[allow(missing_docs)]
    pub fn j(&self)  -> Scalar { self.m[9] }

    #[allow(missing_docs)]
    pub fn xx(&self) -> Scalar { self.m[0] }
    #[allow(missing_docs)]
    pub fn xy(&self) -> Scalar { self.m[1] }
    #[allow(missing_docs)]
    pub fn xz(&self) -> Scalar { self.m[2] }
    #[allow(missing_docs)]
    pub fn xw(&self) -> Scalar { self.m[3] }
    #[allow(missing_docs)]
    pub fn yy(&self) -> Scalar { self.m[4] }
    #[allow(missing_docs)]
    pub fn yz(&self) -> Scalar { self.m[5] }
    #[allow(missing_docs)]
    pub fn yw(&self) -> Scalar { self.m[6] }
    #[allow(missing_docs)]
    pub fn zz(&self) -> Scalar { self.m[7] }
    #[allow(missing_docs)]
    pub fn zw(&self) -> Scalar { self.m[8] }
    #[allow(missing_docs)]
    pub fn ww(&self) -> Scalar { self.m[9] }
}

impl<'a, 'b, Scalar> Quadric<Scalar>
    where Scalar: Mul<Output=Scalar> + Add<Output=Scalar> + One + Copy + BaseFloat
{
    /// Computes `v -> v' M v`.
    fn eval_coords(&self, x: Scalar, y: Scalar, z: Scalar, w: Scalar) -> Scalar {
        let two = Scalar::from_f32(2.0f32);
        /* */ self.m[0] * x*x + self.m[1] * two*x*y + self.m[2] * two*x*z + self.m[3] * two*x*w
        /* */                 + self.m[4] *     y*y + self.m[5] * two*y*z + self.m[6] * two*y*w
        /* */                                       + self.m[7] *     z*z + self.m[8] * two*z*w
        /* */                                                             + self.m[9] *     w*w
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

impl<Scalar: BaseFloat + Mul<Output=Scalar>> Quadric<Scalar> {
    /// Multiplies a quadric in-place by a scalar.
    pub fn mul_by(&mut self, s: Scalar) {
        for val in self.m.iter_mut() {
            *val = *val * s;
        }
    }
}

impl<Scalar: Add + Copy> Add<Quadric<Scalar>> for Quadric<Scalar>
    where <Scalar as Add>::Output: Copy
{
    type Output = Quadric<<Scalar as Add>::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Quadric {
            m: [
                self.m[0] + rhs.m[0],
                self.m[1] + rhs.m[1],
                self.m[2] + rhs.m[2],
                self.m[3] + rhs.m[3],
                self.m[4] + rhs.m[4],
                self.m[5] + rhs.m[5],
                self.m[6] + rhs.m[6],
                self.m[7] + rhs.m[7],
                self.m[8] + rhs.m[8],
                self.m[9] + rhs.m[9],
            ]
        }
    }
}

// Multiplication by scalar.
impl<'a, Scalar: Mul + Copy> Mul<Scalar> for &'a Quadric<Scalar>
    where <Scalar as Mul>::Output: Copy
{
    type Output = Quadric<<Scalar as Mul>::Output>;

    fn mul(self, s: Scalar) -> Self::Output {
        Quadric {
            m: [
                self.m[0] * s,
                self.m[1] * s,
                self.m[2] * s,
                self.m[3] * s,
                self.m[4] * s,
                self.m[5] * s,
                self.m[6] * s,
                self.m[7] * s,
                self.m[8] * s,
                self.m[9] * s,
            ]
        }
    }
}

// Matrix multiplication by vec4.
impl<'a, 'b, Scalar> Mul<&'b Vec4<Scalar>> for &'a Quadric<Scalar>
    where Scalar: Mul<Output=Scalar> + Add<Output=Scalar> + Copy
{
    type Output = Vec4<Scalar>;

    fn mul(self, v: &'b Vec4<Scalar>) -> Self::Output {
        Vec4::new(
            self.m[0] * v[0] + self.m[1] * v[1] + self.m[2] * v[2] + self.m[3] * v[3],
            self.m[1] * v[0] + self.m[4] * v[1] + self.m[5] * v[2] + self.m[6] * v[3],
            self.m[2] * v[0] + self.m[5] * v[1] + self.m[7] * v[2] + self.m[8] * v[3],
            self.m[3] * v[0] + self.m[6] * v[1] + self.m[8] * v[2] + self.m[9] * v[3],
        )
    }
}

// Matrix multiplication by vec4.
impl<'a, Scalar> Mul<Vec4<Scalar>> for &'a Quadric<Scalar>
    where Scalar: Mul<Output=Scalar> + Add<Output=Scalar> + Copy
{
    type Output = Vec4<Scalar>;

    fn mul(self, v: Vec4<Scalar>) -> Self::Output { self * &v }
}

impl<Scalar: Zero + Copy> Zero for Quadric<Scalar> {
    /// Zero quadric.
    fn zero() -> Self {
        Quadric { m: [Zero::zero(); 10] }
    }
    /// Whether the quadric is zero.
    fn is_zero(&self) -> bool {
        self.m.iter().all(|v| v.is_zero())
    }
}

/// Alias for Quadric<f32>.
pub type Quadricf = Quadric<f32>;
/// Alias for Quadric<f64>.
pub type Quadricd = Quadric<f64>;


#[cfg(test)]
mod test {
    extern crate num;
    use self::num::traits::Zero;
    use geometry::vector::{Vec3, Vec4, Norm};
    use geometry::quadric::{Quadric,Quadricd};

    #[test]
    fn test_init() {
        println!("{:?}", Quadricd::zero());
        println!("{:?}", Quadric::from_matrix([1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0]));
        println!("{:?}", Quadric::from_plane_coeffs(1.0, 2.0, 3.0, 4.0));
        println!("{:?}", Quadric::from_plane(&Vec3::new(1.0, 2.0, 3.0), &Vec3::new(-1.0, 0.0, -1.0)));
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
        assert_eq!(q.eval3(&v3), v3.sqnorm() + 1.0);
        assert_eq!(q.eval(&v4), v4.sqnorm());
    }
}
