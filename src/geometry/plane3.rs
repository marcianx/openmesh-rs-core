//! See documentation for the `Plane3` struct.

use crate::geometry::math::Real;
use crate::geometry::vector::Vec3;
use num::traits::{One, Zero};

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
/// Defines a plane in the form:
///   n . <x, y, z> + d = 0
pub struct Plane3<T: Real> {
    n: Vec3<T>,
    d: T,
}

impl<T: Real> Plane3<T> {
    /// Plane normal to vector `n` that contains point `pt`. `n` must be non-zero.
    pub fn new(mut n: Vec3<T>, pt: &Vec3<T>) -> Self {
        let mag = n.norm();
        assert!(mag > Zero::zero());
        n.unscale_mut(mag);
        Plane3 { n, d: -n.dot(pt) }
    }

    /// Plane normal - always normalized.
    pub fn normal(&self) -> Vec3<T> {
        self.n
    }

    /// x-y plane.
    pub fn xy() -> Self {
        Plane3 {
            n: Vec3::new(Zero::zero(), Zero::zero(), One::one()),
            d: Zero::zero(),
        }
    }

    /// Signed distance of a point from the plane.
    pub fn signed_dist(&self, pt: &Vec3<T>) -> T {
        self.n.dot(&pt) + self.d
    }

    /// Signed distance to the origin. Same as `self.signed_dist(origin)`.
    pub fn signed_dist_to_origin(&self) -> T {
        self.d
    }
}

/// Alias for Plane3<f32>.
pub type Plane3f = Plane3<f32>;
/// Alias for Plane3<f64>.
pub type Plane3d = Plane3<f64>;

#[cfg(test)]
mod test {
    use crate::geometry::plane3::Plane3d;
    use crate::geometry::vector::Vec3;

    #[test]
    fn test_init() {
        println!("{:?}", Plane3d::xy());
        println!(
            "{:?}",
            Plane3d::new(Vec3::new(1.0, 2.0, 3.0), &Vec3::new(-1.0, 0.0, -1.0))
        );
    }

    #[test]
    fn test_signed_dist() {
        let q = Plane3d::new(Vec3::new(1.0, 2.0, 3.0), &Vec3::new(-1.0, 0.0, -1.0));
        assert!(q.signed_dist(&Vec3::new(-1.0, 0.0, -1.0)) == 0.0);
        assert!(q.signed_dist(&Vec3::new(0.0, 2.0, 2.0)) > 0.0);
        assert!(q.signed_dist(&Vec3::new(-2.0, -2.0, -4.0)) < 0.0);
    }
}
