use std::ops;

#[derive(Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn get_data(&self, other: &Vec3) -> (f64, Vec3) {
        let sq_dist = (self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2);
        let dist = sq_dist.sqrt();

        let x = (other.x - self.x) / dist;
        let y = (other.y - self.y) / dist;
        let z = (other.z - self.z) / dist;

        let dir = Self { x, y, z };

        (sq_dist, dir)
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

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}