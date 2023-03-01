use crate::utils::Vec3;

#[derive(Clone, Debug)]
pub struct Celestial {
    name: String,
    mass: f64,
    pos: Vec3,
    vel: Vec3,
}

impl Celestial {
    pub fn new(name: String, mass: f64, pos: Vec3, vel: Vec3) -> Self {
        Self {
            name,
            mass,
            pos,
            vel,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn pos(&self) -> Vec3 {
        self.pos.clone()
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }

    pub fn apply_gravity(&mut self, acceleration: Vec3, delta_t: f64) {
        self.vel += acceleration * delta_t;
        self.pos += &self.vel * delta_t;
    }
}