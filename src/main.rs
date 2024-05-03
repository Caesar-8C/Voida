mod gui;
mod simulation;
mod tui;
mod utils;
mod world;

use crate::gui::Gui;
use crate::simulation::Simulation;
use crate::tui::window;
use crate::tui::Tui;
use crate::world::{config, World};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use utils::Vec3;
use world::celestials::Celestial;

#[tokio::main]
async fn main() -> Result<(), String> {
    let delta_t = 5_f64;
    let celestials = config::new_solar();
    let mut spaceships = HashMap::new();
    let spaceship = config::iss();
    spaceships.insert(spaceship.name(), spaceship);
    let world = World::new(celestials, spaceships, delta_t);
    let simulation_period = Duration::from_millis(10);
    let (mut simulation, world_watch) =
        Simulation::new(world, simulation_period);

    let gui = Gui::new(20.);
    thread::spawn(move || gui.run(world_watch));

    simulation.spin().await
}
