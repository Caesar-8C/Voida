use crate::World;
use std::time::Duration;
use tokio::sync::watch;
use tokio::sync::watch::{Receiver, Sender};
use tokio::time::interval;
use crate::simulation::control::Control;

pub struct Simulation {
    world: World,
    world_publisher: Sender<World>,
    simulation_period: Duration,
    control: Receiver<bool>,
}

impl Simulation {
    pub fn new(
        world: World,
        simulation_period: Duration,
    ) -> (Self, Receiver<World>) {
        let (world_publisher, world_watch) = watch::channel(world.clone());
        let (control_sender, control) = watch::channel(false);
        // let controller = Control::new(control_sender);
        // tokio::spawn(controller.run());
        (
            Self {
                world,
                world_publisher,
                simulation_period,
                control,
            },
            world_watch,
        )
    }

    pub async fn spin(&mut self) -> Result<(), String> {
        let mut interval = interval(self.simulation_period);

        loop {
            interval.tick().await;

            if *self.control.borrow() {
                self.world.update();
            }

            self.world.update();

            self.world_publisher
                .send(self.world.clone())
                .map_err(|e| format!("{}", e))?;
        }
    }
}
