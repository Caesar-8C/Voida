use crate::World;
use std::time::Duration;
use tokio::sync::watch;
use tokio::sync::watch::{Receiver, Sender};
use tokio::time::interval;

pub struct Simulation {
    world: World,
    world_publisher: Sender<World>,
    simulation_period: Duration,
}

impl Simulation {
    pub fn new(
        world: World,
        simulation_period: Duration,
    ) -> (Self, Receiver<World>) {
        let (world_publisher, world_watch) = watch::channel(world.clone());
        (
            Self {
                world,
                world_publisher,
                simulation_period,
            },
            world_watch,
        )
    }

    pub async fn spin(&mut self) -> Result<(), String> {
        let mut interval = interval(self.simulation_period);

        loop {
            interval.tick().await;

            self.world.update();

            self.world_publisher
                .send(self.world.clone())
                .map_err(|e| format!("{}", e))?;
        }
    }
}
