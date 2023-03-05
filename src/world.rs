pub mod celestials;
pub mod config;
pub mod spaceship;

use celestials::Celestials;
use std::collections::HashMap;
use crate::utils::Body;
use crate::world::spaceship::Spaceship;

#[derive(Clone, Debug)]
pub struct World {
    celestials: Celestials,
    spaceships: HashMap<String, Spaceship>,
    delta_t: f64,
}

impl World {
    pub fn new_solar(delta_t: f64) -> Self {
        let mut spaceships = HashMap::new();
        let spaceship = config::iss();
        spaceships.insert(spaceship.name(), spaceship);
        Self {
            celestials: config::new_solar(),
            spaceships,
            delta_t,
        }
    }

    pub fn get(&self) -> HashMap<String, Box<dyn Body>> {
        let mut res: HashMap<String, Box<dyn Body>> = HashMap::new();
        for (key, val) in self.celestials.get() {
            res.insert(key, Box::new(val));
        }
        for (key, val) in self.spaceships.clone() {
            res.insert(key, Box::new(val));
        }
        res
    }

    pub fn update(&mut self) {
        // for spaceship in &mut self.spaceships {
        //     let a = self.celestials.get_global_acceleration(spaceship.pos());
        //     spaceship.apply_gravity(a, self.delta_t);
        // }

        self.celestials.update(self.delta_t);
    }
}
