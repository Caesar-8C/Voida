use approx::AbsDiff;
use nalgebra::{Matrix3, Translation, Vector3};
use std::ops;

pub const G: f64 = 6.6743_f64 * 0.000_000_000_01;

#[derive(Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct NormVec3 {
    pub distance_sq: f64,
    pub distance: f64,
    pub unit_direction: Vec3,
}

impl Vec3 {
    pub fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn normalize(&self) -> NormVec3 {
        let distance_sq = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
        let dist = distance_sq.sqrt();

        let x = self.x / dist;
        let y = self.y / dist;
        let z = self.z / dist;

        NormVec3 {
            distance_sq,
            distance: dist,
            unit_direction: Self { x, y, z },
        }
    }

    pub fn equal_to(&self, other: &Vec3, epsilon: f64) -> bool {
        AbsDiff::default().epsilon(epsilon).eq(&self.x, &other.x)
            && AbsDiff::default().epsilon(epsilon).eq(&self.y, &other.y)
            && AbsDiff::default().epsilon(epsilon).eq(&self.z, &other.z)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Mul<&Vec3> for Vec3 {
    type Output = f64;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl ops::Mul<&Vec3> for &Vec3 {
    type Output = f64;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl ops::Mul<&Vec3> for &Matrix3<f64> {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self[(0, 0)] * rhs.x + self[(0, 1)] * rhs.y + self[(0, 2)] * rhs.z,
            y: self[(1, 0)] * rhs.x + self[(1, 1)] * rhs.y + self[(1, 2)] * rhs.z,
            z: self[(2, 0)] * rhs.x + self[(2, 1)] * rhs.y + self[(2, 2)] * rhs.z,
        }
    }
}

impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl From<Vec3> for Vector3<f32> {
    fn from(val: Vec3) -> Self {
        Vector3::new(val.x as f32, val.y as f32, val.z as f32)
    }
}

impl From<Translation<f32, 3>> for Vec3 {
    fn from(val: Translation<f32, 3>) -> Self {
        Vec3 {
            x: val.x as f64,
            y: val.y as f64,
            z: val.z as f64,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_normalize() {
        let vec = Vec3 {
            x: 3.0,
            y: 4.0,
            z: 0.0,
        };
        let norm_vec = vec.normalize();

        assert_abs_diff_eq!(norm_vec.distance_sq, 25.0);
        assert_abs_diff_eq!(norm_vec.distance, 5.0);
        assert_abs_diff_eq!(norm_vec.unit_direction.x, 0.6);
        assert_abs_diff_eq!(norm_vec.unit_direction.y, 0.8);
        assert_abs_diff_eq!(norm_vec.unit_direction.z, 0.0);
    }

    #[test]
    fn test_equal_to() {
        let vec1 = Vec3 {
            x: 3.0,
            y: 4.0,
            z: 0.0,
        };
        let vec2 = Vec3 {
            x: 3.0,
            y: 4.0,
            z: 0.0009,
        };
        let vec3 = Vec3 {
            x: 3.0,
            y: 4.0,
            z: 0.0011,
        };

        assert!(vec1.equal_to(&vec2, 0.001));
        assert!(!vec1.equal_to(&vec3, 0.001));
    }
}
