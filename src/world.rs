pub mod celestials;
pub mod config;
mod spaceship;

use celestials::{Celestial, Celestials};
use std::collections::HashMap;
use crate::world::spaceship::Spaceship;

#[derive(Clone, Debug)]
pub struct World {
    celestials: Celestials,
    spaceships: Vec<Spaceship>,
    delta_t: f64,
}

impl World {
    pub fn new_solar(delta_t: f64) -> Self {
        Self {
            celestials: config::new_solar(),
            spaceships: vec![config::iss()],
            delta_t,
        }
    }

    pub fn get(&self) -> HashMap<String, Celestial>{
        self.celestials.get()
    }

    pub fn update(&mut self) {
        self.celestials.update(self.delta_t);

        for spaceship in &mut self.spaceships {
            let a = self.celestials.get_global_acceleration(spaceship.pos());
            spaceship.apply_gravity(a, self.delta_t);
        }
    }
}
