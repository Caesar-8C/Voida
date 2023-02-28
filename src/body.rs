use std::collections::HashMap;
use crate::Vec3;

const G: f64 = 6.6743 as f64 * 0.00000000001;

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
        let period = (60 * 60) as f64;
        let mut acceleration = Vec3::default();

        for (_, other) in others {
            if self.name == other.name {
                continue;
            }

            let (sq_dist, direction) = self.pos.get_data(&other.pos);
            acceleration += direction * (G * other.mass.clone() / sq_dist);
        }
        self.pos += &self.vel * period;
        self.vel += acceleration * period;
    }
}