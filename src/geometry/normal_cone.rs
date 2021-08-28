//! See documentation for the `NormalCone` struct.

use crate::geometry::math::{max, min, BaseFloat};
use crate::geometry::vector::{Dot, Vec3};
use num::traits::{One, Zero};

#[repr(C)]
#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
/// Normal cone as defined by a direction vector and the angle from the vector to the cone surface.
pub struct NormalCone<T> {
    center_normal: Vec3<T>,
    angle: T,
}

impl<T: BaseFloat + One + Zero> NormalCone<T> {
    /// Plane normal to vector n that contains point pt.
    pub fn new(center_normal: &Vec3<T>, angle: T) -> Self {
        NormalCone {
            center_normal: *center_normal,
            angle,
        }
    }
    /// Zero cone.
    pub fn zero() -> Self {
        NormalCone {
            center_normal: Zero::zero(),
            angle: Zero::zero(),
        }
    }

    /// Returns the cone center normal.
    pub fn center_normal(&self) -> &Vec3<T> {
        &self.center_normal
    }
    /// Returns the cone angle.
    pub fn angle(&self) -> T {
        self.angle
    }

    /// Max distance (radians) from unit vector to cone (distant side).
    pub fn max_angle_to_vec(&self, normal: &Vec3<T>) -> T {
        let dotp: T = self.center_normal.dot(&normal);
        let one: T = One::one();
        let center_angle = if dotp >= one {
            Zero::zero()
        } else if dotp <= -one {
            T::pi()
        } else {
            dotp.acos()
        };
        // TODO: Should this have been clamped to pi?
        center_angle + self.angle
    }

    /// Max distance (radians) from cone to cone (distant side).
    pub fn max_angle_to_cone(&self, cone: &NormalCone<T>) -> T {
        let dotp = self.center_normal.dot(&cone.center_normal);
        let one: T = One::one();
        let center_angle = if dotp >= one {
            Zero::zero()
        } else if dotp <= -one {
            T::pi()
        } else {
            dotp.acos()
        };
        // TODO: A large cone inside a small cone => max distance between any point on one cone to
        // any point on the other cone seems to be the sum of the angles. This formula instead
        // computes twice the largest cone angle. Check the intention with the authors.
        let side_angle0 = max(self.angle - center_angle, cone.angle);
        let side_angle1 = max(cone.angle - center_angle, self.angle);
        // TODO: Should this have been clamped to pi?
        center_angle + side_angle0 + side_angle1
    }

    /// Merges cone so that this instance encloses both former cones.
    pub fn merge(&mut self, cone: &NormalCone<T>) {
        let dotp = self.center_normal.dot(&cone.center_normal);
        let half = T::from_f32(0.5f32);
        if dotp.abs() < T::from_f32(0.99999f32) {
            // New angle
            let center_angle = dotp.acos();
            let min_angle = min(-self.angle, center_angle - cone.angle);
            let max_angle = max(self.angle, center_angle + cone.angle);
            self.angle = (max_angle - min_angle) * half;

            // Axis by SLERP
            let axis_angle = half * (min_angle + max_angle);
            self.center_normal = (self.center_normal * (center_angle - axis_angle).sin()
                + cone.center_normal * axis_angle.sin())
                / center_angle.sin();
        } else {
            self.angle = if dotp > Zero::zero() {
                // Axes point in the same direction
                max(self.angle, cone.angle)
            } else {
                // Axes point in opposite directions
                T::two_pi()
            }
        }
    }
}

/// Alias for `NormalCone<f32>`.
pub type NormalConef = NormalCone<f32>;
/// Alias for `NormalCone<f64>`.
pub type NormalConed = NormalCone<f64>;

#[cfg(test)]
mod test {
    use crate::geometry::math::FloatCompare;
    use crate::geometry::normal_cone::{NormalCone, NormalConed};
    use crate::geometry::vector::{Norm, Vec3};
    use std::f64;

    const PI: f64 = f64::consts::PI;
    const TWO_PI: f64 = f64::consts::PI * 2.0;

    #[test]
    fn test_init() {
        println!("{:?}", NormalConed::zero());
        println!("{:?}", NormalCone::new(&Vec3::new(1.0, 2.0, 3.0), TWO_PI));
        assert_eq!(
            NormalCone::new(&Vec3::new(0.0, 0.0, 0.0), 0.0),
            NormalCone::zero()
        );
    }

    #[test]
    fn test_max_angle_to_vec() {
        let angle = PI / 8.0;
        let dir = Vec3::new(1.0, 2.0, 3.0);
        let dir_perp = Vec3::new(-1.0, 2.0, -1.0);
        let cone = NormalCone::new(&dir, angle);
        assert!(cone.max_angle_to_vec(&dir).is_eq(angle));
        let max_angle = cone.max_angle_to_vec(&dir_perp);
        assert!(
            max_angle.is_eq(PI / 2.0 + angle),
            "max_angle_to_vec = {}, expected = {}",
            max_angle,
            PI / 2.0 + angle
        );
        let max_angle = cone.max_angle_to_vec(&-dir);
        assert!(
            max_angle.is_eq(PI + angle),
            "max_angle_to_vec = {}, expected = {}",
            max_angle,
            PI + angle
        );
    }

    #[test]
    fn test_max_angle_to_cone() {
        let angle1 = PI / 8.0;
        let angle2 = PI / 4.0;
        let dir = Vec3::new(1.0, 2.0, 3.0);
        let dir_perp = Vec3::new(-1.0, 2.0, -1.0);
        let cone = NormalCone::new(&dir, angle1);
        let cone_parallel = NormalCone::new(&dir, angle2);
        let cone_perp = NormalCone::new(&dir_perp, angle2);
        let cone_opp = NormalCone::new(&-dir, angle2);
        // NOTE: Really, expected angle1 + angle2 instead of 2*angle2.
        assert!(cone.max_angle_to_cone(&cone_parallel).is_eq(2.0 * angle2));
        assert!(cone
            .max_angle_to_cone(&cone_perp)
            .is_eq(PI / 2.0 + angle1 + angle2));
        assert!(cone
            .max_angle_to_cone(&cone_opp)
            .is_eq(PI + angle1 + angle2));
    }

    #[test]
    fn test_merge() {
        let angle = PI / 8.0;
        let dir = Vec3::new(1.0, 2.0, 3.0);
        let dir_perp = Vec3::new(-1.0, 2.0, -1.0);
        let dir_unit = dir.normalize();
        let dir_unit_perp = dir_perp.normalize();

        // Cones in the same direction.
        let mut cone1 = NormalCone::new(&dir, angle / 16.0);
        let cone2 = NormalCone::new(&dir, angle);
        cone1.merge(&cone2);
        assert_eq!(cone1.angle(), angle);
        assert_eq!(cone1.center_normal(), &dir);

        // Cones in opposite directions.
        cone1 = NormalCone::new(&dir, angle / 16.0);
        let cone2 = NormalCone::new(&-dir, angle);
        cone1.merge(&cone2);
        assert_eq!(cone1.angle(), TWO_PI);

        // Cones of the same size perpendicular to each other.
        cone1 = NormalCone::new(&dir_unit, angle);
        let cone2 = NormalCone::new(&dir_unit_perp, angle);
        cone1.merge(&cone2);
        let expected_angle = PI / 4.0 + angle;
        let expected_normal = ((dir_unit + dir_unit_perp) / 2.0).normalize();
        assert!(
            cone1.angle().is_eq(expected_angle),
            "cone1.angle() = {}, expected = {}",
            cone1.angle(),
            expected_angle
        );
        assert!((*cone1.center_normal() - expected_normal).norm().is_zero());

        // Cones of different sizes perpendicular to each other.
        cone1 = NormalCone::new(&dir, angle / 16.0);
        let cone2 = NormalCone::new(&dir_perp, angle);
        cone1.merge(&cone2);
        let expected_angle = PI / 4.0 + angle * (17.0 / 32.0);
        assert!(
            cone1.angle().is_eq(expected_angle),
            "cone1.angle() = {}, expected = {}",
            cone1.angle(),
            expected_angle
        );
    }
}
