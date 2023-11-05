use super::spaceship::Spaceship;
use crate::{Celestial, Vec3};
use super::celestials::Celestials;
use std::collections::HashMap;

pub enum Body {
    Celestial(Celestial),
    Spaceship(Spaceship),
}

impl Body {
    pub fn pos(&self) -> Vec3 {
        match self {
            Body::Celestial(c) => c.pos(),
            Body::Spaceship(ss) => ss.pos(),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Body::Celestial(c) => c.name(),
            Body::Spaceship(ss) => ss.name(),
        }
    }
}

impl From<Spaceship> for Body {
    fn from(sh: Spaceship) -> Self {
        Body::Spaceship(sh)
    }
}

impl From<Celestial> for Body {
    fn from(ce: Celestial) -> Self {
        Body::Celestial(ce)
    }
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
            res.insert(key, val.into());
        }
        for (key, val) in self.spaceships.clone() {
            res.insert(key, val.into());
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
