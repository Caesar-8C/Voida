use crate::world::Body;
use crate::{Celestial, Vec3, World};
use tokio::sync::watch::Receiver;

#[derive(Clone)]
pub enum WindowType {
    Camera { window: Window, camera: Camera },
}

impl WindowType {
    pub fn render(&self) -> Vec<Vec<String>> {
        match self {
            WindowType::Camera { window, camera } => {
                Self::render_camera(window, camera)
            }
        }
    }

    fn render_camera(window: &Window, camera: &Camera) -> Vec<Vec<String>> {
        let mut render =
            vec![vec![" ".to_string(); window.width]; window.height];

        let world = camera.world.borrow().get();

        let focus = match &world[&camera.focus] {
            Body::Celestial(c) => {
                Self::draw_focus_body(c, window, camera, &mut render);
                c.pos()
            }
            Body::Spaceship(ss) => ss.pos(),
        };

        for body in world.values() {
            let (name, pos) = match body {
                Body::Celestial(c) => (c.name(), c.pos()),
                Body::Spaceship(ss) => (ss.name(), ss.pos()),
            };

            let x = (&pos - &focus) * &camera.x_dir * camera.scale * 2.
                + window.width as f64 / 2.;
            let y = (&focus - &pos) * &camera.y_dir * camera.scale
                + window.height as f64 / 2.;

            if window.inside(x, y) {
                let char = Self::get_symbol(&name);
                if (&char != "∘" && &char != "I") || render[y as usize][x as usize] == " " {
                    render[y as usize][x as usize] = char;
                }
            }
        }

        render
    }

    fn draw_focus_body(
        celestial: &Celestial,
        window: &Window,
        camera: &Camera,
        render: &mut Vec<Vec<String>>,
    ) {
        let char = Self::get_symbol(&celestial.name());

        if camera.scale * celestial.rad() > 1. {
            for i in 0..(window.width) {
                for j in 0..(window.height) {
                    let x =
                        ((i as f64 - (window.width as f64 / 2.)) / 2.).abs();
                    let y = (j as f64 - (window.height as f64 / 2.)).abs();
                    let dist = (x * x + y * y).sqrt() / camera.scale;
                    if dist < celestial.rad() {
                        render[j][i] = char.clone();
                    }
                }
            }
        }
    }

    fn get_symbol(name: &str) -> String {
        match name {
            "Sun" => "O".to_string(),
            "Earth" => "o".to_string(),
            "Moon" => "∘".to_string(),
            "ISS" => "I".to_string(),
            _ => "X".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Window {
    pub width: usize,
    pub height: usize,
    pub x: usize,
    pub y: usize,
}

impl Window {
    pub fn inside(&self, x_f64: f64, y_f64: f64) -> bool {
        if x_f64 < 0. || y_f64 < 0. {
            return false;
        }

        let x = x_f64 as usize;
        let y = y_f64 as usize;

        x < self.width && y < self.height
    }
}

#[derive(Clone)]
pub struct Camera {
    pub scale: f64,
    pub focus: String,
    pub x_dir: Vec3,
    pub y_dir: Vec3,
    pub world: Receiver<World>,
}

pub fn sun_standard(world: Receiver<World>) -> WindowType {
    WindowType::Camera {
        window: Window {
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
    }
}

pub fn earth_standard(world: Receiver<World>) -> WindowType {
    WindowType::Camera {
        window: Window {
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
    }
}

pub fn moon_from_side(world: Receiver<World>) -> WindowType {
    WindowType::Camera {
        window: Window {
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
    }
}

pub fn iss(world: Receiver<World>) -> WindowType {
    WindowType::Camera {
        window: Window {
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
    }
}
