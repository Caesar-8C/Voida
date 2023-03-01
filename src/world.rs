use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::watch::{Receiver, Sender};
use tokio::time::interval;
use tokio::sync::watch;
use crate::celestial::Celestial;
use crate::utils::NormVec3;
use crate::Vec3;

const G: f64 = 6.6743 as f64 * 0.000_000_000_01;
const DELTA_T: f64 = 60. * 60.;

pub struct World {
    celestials: HashMap<String, Celestial>,
    world_publisher: Sender<HashMap<String, Celestial>>,
}

impl World {
    pub fn new() -> (Self, Receiver<HashMap<String, Celestial>>) {
        let celestials = HashMap::new();
        let (world_publisher, world_watch) = watch::channel(celestials.clone());
        (
            Self {
                celestials,
                world_publisher,
            },
            world_watch
        )
    }

    pub async fn spin(&mut self, simulation_period: Duration) {
        let mut interval = interval(simulation_period);

        loop {
            interval.tick().await;

            self.update_celestials();

            self.world_publisher.send(self.celestials.clone()).unwrap();
        }
    }

    pub fn add_celestial(&mut self, new_celestial: Celestial) {
        self.celestials.insert(new_celestial.name(), new_celestial);
    }

    pub fn get_global_acceleration(&self, origin: Vec3) -> Vec3 {
        let mut acceleration = Vec3::default();

        for (_, celestial) in &self.celestials {
            let NormVec3 { distance_sq, unit_direction, .. } = (celestial.pos() - &origin).normalize();
            if distance_sq > 1. {
                acceleration += unit_direction * (G * celestial.mass() / distance_sq);
            }
        }

        acceleration
    }

    fn update_celestials(&mut self) {
        let cloned_keys: Vec<String> = self.celestials.keys().cloned().collect();
        let mut accelerations = HashMap::new();
        for key in &cloned_keys {
            let a = self.get_global_acceleration(self.celestials[key].pos());
            accelerations.insert(key.clone(), a);
        }
        for key in &cloned_keys {
            if let Some(celestial) = self.celestials.get_mut(key) {
                if let Some(a) = accelerations.get(key) {
                    celestial.apply_gravity(a.clone(), DELTA_T);
                }
            }
        }
    }
}