use crate::celestials::{Celestial, Celestials};
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::watch;
use tokio::sync::watch::{Receiver, Sender};
use tokio::time::interval;

const DELTA_T: f64 = 60. * 60.;

pub struct World {
    celestials: Celestials,
    world_publisher: Sender<HashMap<String, Celestial>>,
}

impl World {
    pub fn new() -> (Self, Receiver<HashMap<String, Celestial>>) {
        let celestials = Celestials::new();
        let (world_publisher, world_watch) = watch::channel(celestials.get());
        (
            Self {
                celestials,
                world_publisher,
            },
            world_watch,
        )
    }

    pub async fn spin(&mut self, simulation_period: Duration) {
        let mut interval = interval(simulation_period);

        loop {
            interval.tick().await;

            self.celestials.update(DELTA_T);

            self.world_publisher
                .send(self.celestials.get())
                .unwrap();
        }
    }

    pub fn add_celestial(&mut self, new_celestial: Celestial) {
        self.celestials.add(new_celestial);
    }
}
