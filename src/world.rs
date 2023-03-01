use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::watch::{Receiver, Sender};
use tokio::time::{Instant, interval};
use tokio::sync::watch;
use crate::body::Body;

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

    pub async fn spin(&mut self, simulation_period: Duration) {
        let mut interval = interval(simulation_period);

        loop {
            interval.tick().await;

            let old_state = self.bodies.clone();
            for (_, body) in &mut self.bodies {
                body.apply_gravity(&old_state);
            }

            self.world_publisher.send(self.bodies.clone()).unwrap();
        }
    }
}