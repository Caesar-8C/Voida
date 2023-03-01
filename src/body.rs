use std::collections::HashMap;
use crate::utils::{Vec3, NormVec3};

const G: f64 = 6.6743 as f64 * 0.000_000_000_01;
const DELTA_T: f64 = 60. * 60.;

#[derive(Clone, Debug)]
pub struct Body {
    name: String,
    mass: f64,
    pos: Vec3,
    vel: Vec3,
}

impl Body {
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

    pub fn apply_gravity(&mut self, others: &HashMap<String, Body>) {
        let mut acceleration = Vec3::default();

        for (_, other) in others {
            if self.name == other.name {
                continue;
            }

            let NormVec3 { distance_sq, unit_direction } = (&other.pos - &self.pos).normalize();
            acceleration += unit_direction * (G * other.mass / distance_sq);
        }
        self.vel += acceleration * DELTA_T;
        self.pos += &self.vel * DELTA_T;
    }
}