pub mod celestials;
pub mod config;
pub mod spaceship;

use crate::world::spaceship::Spaceship;
use crate::Celestial;
use celestials::Celestials;
use std::collections::HashMap;

pub enum Body {
    Celestial(Celestial),
    Spaceship(Spaceship),
}

#[derive(Clone, Debug)]
pub struct World {
    celestials: Celestials,
    spaceships: HashMap<String, Spaceship>,
    delta_t: f64,
}

impl World {
    pub fn new(
        celestials: Celestials,
        spaceships: HashMap<String, Spaceship>,
        delta_t: f64,
    ) -> Self {
        Self {
            celestials,
            spaceships,
            delta_t,
        }
    }

    pub fn get(&self) -> HashMap<String, Body> {
        let mut res: HashMap<String, Body> = HashMap::new();
        for (key, val) in self.celestials.get() {
            res.insert(key, Body::Celestial(val));
        }
        for (key, val) in self.spaceships.clone() {
            res.insert(key, Body::Spaceship(val));
        }
        res
    }

    pub fn update(&mut self) {
        for spaceship in self.spaceships.values_mut() {
            let a = self.celestials.get_global_acceleration(spaceship.pos());
            spaceship.apply_gravity(a, self.delta_t);
        }

        self.celestials.update(self.delta_t);
    }
}
