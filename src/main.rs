mod gui;
mod simulation;
mod utils;
mod world;

use gui::Gui;
use simulation::Simulation;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use utils::Vec3;
use world::celestials::Celestial;
use world::{config, World};

#[tokio::main]
async fn main() -> Result<(), String> {
    let delta_t = 5_f64;
    let celestials = config::new_solar();
    let mut spaceships = HashMap::new();
    let spaceship = config::iss();
    let spaceship2 = config::iss2();
    spaceships.insert(spaceship.name(), spaceship);
    spaceships.insert(spaceship2.name(), spaceship2);
    let world = World::new(celestials, spaceships, delta_t);
    let simulation_period = Duration::from_millis(10);
    let (mut simulation, world_watch) =
        Simulation::new(world, simulation_period);

    let gui = Gui::new(20.);
    thread::spawn(move || gui.run(world_watch));

    simulation.spin().await
}
