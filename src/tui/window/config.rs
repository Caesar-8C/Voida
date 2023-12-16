use super::PlotWindow;
use super::{Camera, CameraWindow, Canvas, TextWindow};
use crate::{Vec3, World};
use std::time::{Duration, Instant};
use tokio::sync::watch::Receiver;

pub fn _sun_standard(world: Receiver<World>) -> Box<CameraWindow> {
    Box::new(CameraWindow {
        window: Canvas {
            width: 80,
            height: 26,
            x: 21,
            y: 0,
        },
        camera: Camera {
            scale: 10_f64 / 2_f64 / 10_f64.powi(11),
            focus: "Sun".to_string(),
            x_dir: Vec3 {
                x: 1.,
                y: 0.,
                z: 0.,
            },
            y_dir: Vec3 {
                x: 0.,
                y: 1.,
                z: 0.,
            },
            world,
        },
        update_period: Duration::from_millis(200),
        next_update: Instant::now(),
    })
}

pub fn earth_standard(world: Receiver<World>) -> Box<CameraWindow> {
    Box::new(CameraWindow {
        window: Canvas {
            width: 80,
            height: 40,
            x: 102,
            y: 0,
        },
        camera: Camera {
            scale: 10_f64 / 3_f64 / 10_f64.powi(8),
            focus: "Earth".to_string(),
            x_dir: Vec3 {
                x: 1.,
                y: 0.,
                z: 0.,
            },
            y_dir: Vec3 {
                x: 0.,
                y: 1.,
                z: 0.,
            },
            world,
        },
        update_period: Duration::from_millis(200),
        next_update: Instant::now(),
    })
}

pub fn _moon_from_side(world: Receiver<World>) -> Box<CameraWindow> {
    Box::new(CameraWindow {
        window: Canvas {
            width: 80,
            height: 12,
            x: 102,
            y: 41,
        },
        camera: Camera {
            scale: 10_f64 / 3_f64 / 10_f64.powi(8),
            focus: "Earth".to_string(),
            x_dir: Vec3 {
                x: 1.,
                y: 0.,
                z: 0.,
            },
            y_dir: Vec3 {
                x: 0.,
                y: 0.,
                z: 1.,
            },
            world,
        },
        update_period: Duration::from_millis(200),
        next_update: Instant::now(),
    })
}

pub fn iss(world: Receiver<World>) -> Box<CameraWindow> {
    Box::new(CameraWindow {
        window: Canvas {
            width: 80,
            height: 26,
            x: 21,
            y: 27,
        },
        camera: Camera {
            scale: 10_f64 / 6_f64 / 10_f64.powi(6),
            focus: "ISS".to_string(),
            x_dir: Vec3 {
                x: 1.,
                y: 0.,
                z: 0.,
            },
            y_dir: Vec3 {
                x: 0.,
                y: 1.,
                z: 0.,
            },
            world,
        },
        update_period: Duration::from_millis(100),
        next_update: Instant::now(),
    })
}

pub fn plot_test(world: Receiver<World>) -> Box<PlotWindow> {
    Box::new(PlotWindow {
        window: Canvas {
            width: 80,
            height: 26,
            x: 21,
            y: 0,
        },
        world,
        data: vec![400_000.; 200],
        cursor: 0,
        update_period: Duration::from_secs(2),
        next_update: Instant::now(),
    })
}

pub fn text_test() -> Box<TextWindow> {
    Box::new(TextWindow {
        window: Canvas {
            width: 80,
            height: 12,
            x: 102,
            y: 41,
        },
        data: "\n Hello, World!\n How are things?\n Let's do stuff".to_string(),
        update_pending: true,
    })
}
