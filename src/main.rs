mod utils;
mod tui;
mod world;

use std::time::Duration;
use utils::Vec3;
use world::celestials::Celestial;
use crate::tui::TUI;
use crate::world::config::Config;
use crate::world::World;

#[tokio::main]
async fn main() {
    let config = Config::new_solar();
    let (mut world, world_watch) = World::from_config(config);

    let tui = TUI::init(world_watch, 20).await;
    tokio::spawn(tui.run());
    let simulation_period = Duration::from_millis(10);
    world.spin(simulation_period).await;
}