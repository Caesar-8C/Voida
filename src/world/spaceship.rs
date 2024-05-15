use rapier3d::prelude::RigidBodyHandle;
use crate::Vec3;

#[derive(Clone, Debug)]
pub struct Spaceship {
    name: String,
    mass: f64,
    pos: Vec3,
    vel: Vec3,
    pub rapier_handle: Option<RigidBodyHandle>,
}

impl Spaceship {
    pub fn new(name: String, mass: f64, pos: Vec3, vel: Vec3) -> Self {
        Self {
            name,
            mass,
            pos,
            vel,
            rapier_handle: None,
        }
    }

    pub fn set_rapier_handle(&mut self, handle: RigidBodyHandle) {
        self.rapier_handle = Some(handle);
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn pos(&self) -> Vec3 {
        self.pos.clone()
    }

    pub fn vel(&self) -> Vec3 {
        self.vel.clone()
    }

    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }

    pub fn _apply_gravity(&mut self, acceleration: Vec3, delta_t: f64) {
        self.vel += acceleration * delta_t;
        self.pos += &self.vel * delta_t;
    }

    pub fn speedup(&mut self) {
        self.vel.y *= 2.;
    }
}
