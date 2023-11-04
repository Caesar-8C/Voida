#[allow(clippy::module_inception)]
pub mod world;
pub mod celestials;
pub mod config;
pub mod spaceship;

pub use world::{Body, World};
