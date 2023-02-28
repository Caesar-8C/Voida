mod utils;
mod tui;
mod world;
mod body;

use std::time::Duration;
use utils::Vec3;
use tokio;
use crate::body::Body;
use crate::world::World;

#[tokio::main]
async fn main() {
    let sun = Body::new(
        "Sun".to_string(),
        1.989110 as f64 * 10_f64.powi(30),
        Vec3 { x: 0., y: 0., z: 0. },
        Vec3 { x: 0., y: 0., z: 0. },
    );

    let earth = Body::new(
        "Earth".to_string(),
        5.972 as f64 * 10_f64.powi(24),
        Vec3 { x: 1.4812 as f64 * 10_f64.powi(11), y: 0., z: 0. },
        Vec3 { x: 0., y: 29780., z: 0. },
    );

    let moon = Body::new(
        "Moon".to_string(),
        7.34767309 as f64 * 10_f64.powi(22),
        Vec3 { x: 1.485255 as f64 * 10_f64.powi(11), y: 0., z: 0. },
        Vec3 { x: 0., y: 30860., z: 0. },
    );

    let (mut world, world_watch) = World::new();
    world.add_body(sun);
    world.add_body(earth);
    world.add_body(moon);

    tokio::spawn(tui::run(world_watch, 20));
    let simulation_period = Duration::from_millis(10);
    world.spin(simulation_period).await;
}