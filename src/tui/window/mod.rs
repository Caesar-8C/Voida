mod camera;
mod config;
#[allow(clippy::module_inception)]
mod window;

use camera::{Camera, CameraWindow};
pub use config::{earth_standard, iss, moon_from_side, sun_standard};
use window::Rectangle;
pub use window::Window;
