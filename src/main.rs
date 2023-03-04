mod utils;
mod tui;
mod world;

use std::time::Duration;
use utils::Vec3;
use world::celestials::Celestial;
use crate::tui::Tui;
use crate::tui::window;
use crate::world::config::Config;
use crate::world::World;

#[tokio::main]
async fn main() {
    let config = Config::new_solar();
    let (mut world, world_watch) = World::from_config(config);

    let mut tui = Tui::init(world_watch, 20).await;
    tui.add_window(window::sun_standard());
    tui.add_window(window::earth_standard());
    tui.add_window(window::moon_from_side());
    tokio::spawn(tui.run());

    let simulation_period = Duration::from_millis(10);
    world.spin(simulation_period).await;
}