mod gui;
mod simulation;
mod utils;
mod world;

use gui::Gui;
use simulation::Simulation;
use std::collections::HashMap;
use std::thread;
use utils::Vec3;
use world::celestials::Celestial;
use world::{config, World};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), String> {
    let delta_t = 5_f64;
    let celestials = config::new_solar();
    let mut spaceships = HashMap::new();
    let spaceship = config::iss();
    let spaceship2 = config::iss2();
    spaceships.insert(spaceship.name(), spaceship);
    spaceships.insert(spaceship2.name(), spaceship2);
    let world = World::new(celestials, spaceships);
    let (control_sender, control_receiver) = mpsc::channel(100);
    let simulation_fps = 100;
    let (mut simulation, world_watch) =
        Simulation::new(world, simulation_fps, delta_t, control_receiver);

    let gui = Gui::new(20., control_sender);
    thread::spawn(move || gui.run(world_watch));

    simulation.spin().await
}
