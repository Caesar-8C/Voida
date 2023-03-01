use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::watch::{Receiver, Sender};
use tokio::time::interval;
use tokio::sync::watch;
use crate::body::Body;
use crate::utils::NormVec3;
use crate::Vec3;

const G: f64 = 6.6743 as f64 * 0.000_000_000_01;
const DELTA_T: f64 = 60. * 60.;

pub struct World {
    bodies: HashMap<String, Body>,
    world_publisher: Sender<HashMap<String, Body>>,
}

impl World {
    pub fn new() -> (Self, Receiver<HashMap<String, Body>>) {
        let bodies = HashMap::new();
        let (world_publisher, world_watch) = watch::channel(bodies.clone());
        (
            Self {
                bodies,
                world_publisher,
            },
            world_watch
        )
    }

    pub fn add_body(&mut self, new_body: Body) {
        self.bodies.insert(new_body.name(), new_body);
    }

    pub fn get_global_acceleration(&self, origin: Vec3) -> Vec3 {
        let mut acceleration = Vec3::default();

        for (_, body) in &self.bodies {
            let NormVec3 { distance_sq, unit_direction, .. } = (body.pos() - &origin).normalize();
            if distance_sq > 1. {
                acceleration += unit_direction * (G * body.mass() / distance_sq);
            }
        }

        acceleration
    }

    pub async fn spin(&mut self, simulation_period: Duration) {
        let mut interval = interval(simulation_period);

        loop {
            interval.tick().await;

            let cloned_keys: Vec<String> = self.bodies.keys().cloned().collect();
            let mut accelerations = HashMap::new();
            for key in &cloned_keys {
                let a = self.get_global_acceleration(self.bodies[key].pos());
                accelerations.insert(key.clone(), a);
            }
            for key in &cloned_keys {
                if let Some(body) = self.bodies.get_mut(key) {
                    if let Some(a) = accelerations.get(key) {
                        body.apply_gravity(a.clone(), DELTA_T);
                    }
                }
            }

            self.world_publisher.send(self.bodies.clone()).unwrap();
        }
    }
}