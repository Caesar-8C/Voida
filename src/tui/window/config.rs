use super::{Camera, CameraWindow, Rectangle};
use crate::{Vec3, World};
use tokio::sync::watch::Receiver;

pub fn sun_standard(world: Receiver<World>) -> Box<CameraWindow> {
    Box::new(CameraWindow {
        window: Rectangle {
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
    })
}

pub fn earth_standard(world: Receiver<World>) -> Box<CameraWindow> {
    Box::new(CameraWindow {
        window: Rectangle {
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
    })
}

pub fn moon_from_side(world: Receiver<World>) -> Box<CameraWindow> {
    Box::new(CameraWindow {
        window: Rectangle {
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
    })
}

pub fn iss(world: Receiver<World>) -> Box<CameraWindow> {
    Box::new(CameraWindow {
        window: Rectangle {
            width: 80,
            height: 26,
            x: 21,
            y: 27,
        },
        camera: Camera {
            scale: 10_f64 / 6_f64 / 10_f64.powi(6),
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
    })
}
