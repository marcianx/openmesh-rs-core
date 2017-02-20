//! See documentation for the `Plane3` struct.

extern crate num;
use self::num::traits::Zero;
use geometry::math::BaseFloat;
use geometry::vector::{Vec3, Dot, Norm};

#[repr(C)]
#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
/// Defines a plane in the form:
///   n . <x, y, z> + d = 0
pub struct Plane3<T> {
  #[allow(missing_docs)]
  pub n: Vec3<T>,
  #[allow(missing_docs)]
  pub d: T
}

impl<T: BaseFloat> Plane3<T> {
    /// Plane normal to vector n that contains point pt.
    pub fn new(n: &Vec3<T>, pt: &Vec3<T>) -> Self {
        let mut n = *n;
        n.normalize_mut();
        Plane3 { n: n, d: -n.dot(&pt) }
    }
    /// Zero plane.
    pub fn zero() -> Self {
        Plane3 {
            n: Zero::zero(),
            d: Zero::zero()
        }
    }
    /// Signed distance of a point from the plane.
    pub fn signed_dist(&self, pt: &Vec3<T>) -> T {
        self.n.dot(&pt) + self.d
    }
}

/// Alias for Plane3<f32>.
pub type Plane3f = Plane3<f32>;
/// Alias for Plane3<f64>.
pub type Plane3d = Plane3<f64>;


#[cfg(test)]
mod test {
    use geometry::vector::Vec3;
    use geometry::plane3::Plane3d;

    #[test]
    fn test_init() {
        println!("{:?}", Plane3d::zero());
        println!("{:?}", Plane3d::new(&Vec3::new(1.0, 2.0, 3.0), &Vec3::new(-1.0, 0.0, -1.0)));
    }

    #[test]
    fn test_signed_dist() {
        let q = Plane3d::new(&Vec3::new(1.0, 2.0, 3.0), &Vec3::new(-1.0, 0.0, -1.0));
        assert!(q.signed_dist(&Vec3::new(-1.0, 0.0, -1.0)) == 0.0);
        assert!(q.signed_dist(&Vec3::new(0.0, 2.0, 2.0)) > 0.0);
        assert!(q.signed_dist(&Vec3::new(-2.0, -2.0, -4.0)) < 0.0);
    }
}
