use crate::gui::ControlMessage;
use crate::World;
use std::time::Duration;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::{mpsc, watch};
use tokio::time::{interval, Instant};

pub struct Simulation {
    world: World,
    world_publisher: watch::Sender<World>,
    control: mpsc::Receiver<ControlMessage>,
    time_speed: f64,
    simulation_fps: u32,
    delta_t: f64,
}

impl Simulation {
    pub fn new(
        world: World,
        simulation_fps: u32,
        time_speed: f64,
        control: mpsc::Receiver<ControlMessage>,
    ) -> (Self, watch::Receiver<World>) {
        let (world_publisher, world_watch) = watch::channel(world.clone());
        (
            Self {
                world,
                world_publisher,
                control,
                time_speed,
                simulation_fps,
                delta_t: time_speed / simulation_fps as f64,
            },
            world_watch,
        )
    }

    pub async fn spin(&mut self) -> Result<(), String> {
        let mut interval =
            interval(Duration::from_secs_f64(1. / self.simulation_fps as f64));

        let mut start = Instant::now();
        let mut fps_counter = 0;

        loop {
            interval.tick().await;
            fps_counter += 1;
            if start.elapsed().as_secs() >= 1 {
                self.world.true_sim_fps = fps_counter;
                start = Instant::now();
                fps_counter = 0;
            }

            loop {
                match self.control.try_recv() {
                    Ok(ControlMessage::Shutdown)
                    | Err(TryRecvError::Disconnected) => return Ok(()),
                    Ok(ControlMessage::Speedup) => {
                        self.world
                            .spaceships
                            .get_mut("ISS2")
                            .unwrap()
                            .speedup();
                    }
                    Ok(ControlMessage::SetTimeSpeed(speed)) => {
                        self.time_speed = speed;
                        self.delta_t = self.time_speed / self.simulation_fps as f64;
                    },
                    _ => break,
                }
            }

            for spaceship in self.world.spaceships.values_mut() {
                let a = self
                    .world
                    .celestials
                    .get_global_acceleration(spaceship.pos());
                spaceship.apply_gravity(a, self.delta_t);
            }
            self.world.celestials.update(self.delta_t);

            self.world_publisher
                .send(self.world.clone())
                .map_err(|e| format!("World publisher died: {}", e))?;
        }
    }
}
