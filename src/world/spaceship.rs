use crate::Vec3;

#[derive(Clone, Debug)]
pub struct Spaceship {
    name: String,
    mass: f64,
    pos: Vec3,
    vel: Vec3,
}

impl Spaceship {
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

    pub fn speedup(&mut self) {
        self.vel.y *= 2.;
    }
}
