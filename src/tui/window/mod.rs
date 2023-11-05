mod camera;
mod config;
#[allow(clippy::module_inception)]
mod window;
mod plot;

use camera::{Camera, CameraWindow};
use plot::PlotWindow;
pub use config::{earth_standard, iss, moon_from_side, plot_test};
use window::Rectangle;
pub use window::Window;
