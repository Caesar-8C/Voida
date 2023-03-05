pub mod celestials;
pub mod config;

use celestials::{Celestial, Celestials};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct World {
    celestials: Celestials,
    delta_t: f64,
}

impl World {
    pub fn new_solar(delta_t: f64) -> Self {
        Self {
            celestials: config::new_solar(),
            delta_t,
        }
    }

    pub fn get(&self) -> HashMap<String, Celestial>{
        self.celestials.get()
    }

    pub fn update(&mut self) {
        self.celestials.update(self.delta_t);
    }
}
