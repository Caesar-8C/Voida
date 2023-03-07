mod simulation;
mod tui;
mod utils;
mod world;

use crate::simulation::Simulation;
use crate::tui::window;
use crate::tui::Tui;
use crate::world::{config, World};
use std::collections::HashMap;
use std::time::Duration;
use utils::Vec3;
use world::celestials::Celestial;

#[tokio::main]
async fn main() -> Result<(), String> {
    let delta_t = 60_f64 * 60.;
    let celestials = config::new_solar();
    let mut spaceships = HashMap::new();
    let spaceship = config::iss();
    // spaceships.insert(spaceship.name(), spaceship);
    let world = World::new(celestials, spaceships, delta_t);
    let simulation_period = Duration::from_millis(10);
    let (mut simulation, world_watch) =
        Simulation::new(world, simulation_period);

    let mut tui = Tui::init(world_watch, 20, 7).await?;
    tui.add_window(window::sun_standard());
    tui.add_window(window::earth_standard());
    tui.add_window(window::moon_from_side());
    tui.add_window(window::iss());
    tokio::spawn(tui.run());

    simulation.spin().await
}
