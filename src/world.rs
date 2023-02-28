use std::time::Duration;
use tokio::sync::watch::Sender;
use tokio::time::Instant;
use crate::body::Body;

pub struct World {
    pub bodies: Vec<Body>,
}

impl World {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
        }
    }

    pub fn add_body(&mut self, new_body: Body) {
        self.bodies.push(new_body);
    }

    pub async fn spin(&mut self, tx: Sender<Vec<Body>>) {
        let start = Instant::now();
        let period = Duration::from_millis(10);
        let mut wake = start + period;

        loop {
            let now = Instant::now();
            if wake > now {
                tokio::time::sleep(wake - now).await;
            }
            wake = now + period;

            let old_state = self.bodies.clone();
            for body in &mut self.bodies {
                body.apply_gravity(&old_state);
            }

            tx.send(self.bodies.clone()).unwrap();
        }
    }
}