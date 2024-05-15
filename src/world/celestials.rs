use crate::utils::{NormVec3, Vec3, G};
use rapier3d::prelude::RigidBodyHandle;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Celestials(HashMap<String, Celestial>);

impl Celestials {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add(&mut self, new_celestial: Celestial) {
        self.0.insert(new_celestial.name(), new_celestial);
    }

    pub fn get(&self) -> HashMap<String, Celestial> {
        self.0.clone()
    }

    pub fn get_mut (&mut self) -> &mut HashMap<String, Celestial> {
        &mut self.0
    }

    pub fn get_global_acceleration(&self, origin: Vec3) -> Vec3 {
        let mut acceleration = Vec3::default();

        for celestial in self.0.values() {
            let NormVec3 {
                distance_sq,
                unit_direction,
                ..
            } = (celestial.pos() - &origin).normalize();
            if distance_sq > 1. {
                acceleration +=
                    unit_direction * (G * celestial.mass() / distance_sq);
            }
        }

        acceleration
    }

    pub fn _update(&mut self, delta_t: f64) {
        let old_world = self.clone();

        for celestial in self.0.values_mut() {
            let a = old_world.get_global_acceleration(celestial.pos());
            celestial._apply_gravity(a, delta_t);
        }
    }
}

#[derive(Clone)]
pub struct Celestial {
    name: String,
    mass: f64,
    pos: Vec3,
    vel: Vec3,
    rad: f64,
    pub rapier_handle: Option<RigidBodyHandle>,
}

impl Celestial {
    pub fn new(
        name: String,
        mass: f64,
        pos: Vec3,
        vel: Vec3,
        rad: f64,
    ) -> Self {
        Self {
            name,
            mass,
            pos,
            vel,
            rad,
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

    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
    }

    pub fn vel(&self) -> Vec3 {
        self.vel.clone()
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }

    pub fn rad(&self) -> f64 {
        self.rad
    }

    pub fn _apply_gravity(&mut self, acceleration: Vec3, delta_t: f64) {
        self.vel += acceleration * delta_t;
        self.pos += &self.vel * delta_t;
    }
}
