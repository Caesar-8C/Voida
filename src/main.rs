mod gui;
mod simulation;
mod utils;
mod world;

use gui::Gui;
use simulation::Simulation;
use std::collections::HashMap;
use std::thread;
use tokio::sync::mpsc;
use utils::Vec3;
use world::celestials::Celestial;
use world::{config, World};

#[tokio::main]
async fn main() -> Result<(), String> {
    let celestials = config::new_solar();
    let mut spaceships = HashMap::new();
    let spaceship = config::iss();
    let spaceship2 = config::iss2();
    spaceships.insert(spaceship.name(), spaceship);
    spaceships.insert(spaceship2.name(), spaceship2);
    let world = World::new(celestials, spaceships);
    let (control_sender, control_receiver) = mpsc::channel(100);

    let simulation_fps = 200_000;
    let time_speed = 500.;

    let (mut simulation, world_watch) =
        Simulation::new(world, simulation_fps, time_speed, control_receiver);

    let gui = Gui::new(20., world_watch, control_sender);
    let gui_handle = thread::spawn(move || gui.run());

    simulation.spin().await?;

    gui_handle.join().unwrap()
}
