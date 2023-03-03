mod utils;
mod tui;
mod world;

use std::time::Duration;
use utils::Vec3;
use world::celestials::Celestial;
use crate::tui::Tui;
use crate::tui::window::Window;
use crate::world::config::Config;
use crate::world::World;

#[tokio::main]
async fn main() {
    let config = Config::new_solar();
    let (mut world, world_watch) = World::from_config(config);

    let mut tui = Tui::init(world_watch, 20).await;
    let sun_view = Window {
        width: 80,
        height: 53,
        x: 0,
        y: 0,
        scale: 10_f64 / 10_f64.powi(11),
        focus: "Sun".to_string(),
    };
    let earth_view = Window {
        width: 80,
        height: 40,
        x: 81,
        y: 0,
        scale: 10_f64 / 3_f64 / 10_f64.powi(8),
        focus: "Earth".to_string(),
    };
    let moon_view = Window{
        width: 80,
        height: 12,
        x: 81,
        y: 41,
        scale: 10_f64 / 3_f64 / 10_f64.powi(8),
        focus: "Moon".to_string(),
    };
    tui.add_window(sun_view);
    tui.add_window(earth_view);
    tui.add_window(moon_view);
    tokio::spawn(tui.run());

    let simulation_period = Duration::from_millis(10);
    world.spin(simulation_period).await;
}