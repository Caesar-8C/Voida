use crate::world::Body;
use crate::{Celestial, Vec3, World};
use tokio::sync::watch::Receiver;

pub trait Window: Send {
    fn render(&self) -> Vec<Vec<String>>;
    fn position(&self) -> (usize, usize);
}

pub struct CameraWindow {
    pub window: Rectangle,
    pub camera: Camera,
}

impl CameraWindow {
    fn draw_focus_body(
        &self,
        celestial: &Celestial,
        render: &mut [Vec<String>],
    ) {
        let char = Self::get_symbol(&celestial.name());

        if self.camera.scale * celestial.rad() > 1. {
            for (j, row) in render.iter_mut().enumerate() {
                for (i, item) in row.iter_mut().enumerate() {
                    let x = ((i as f64 - (self.window.width as f64 / 2.)) / 2.)
                        .abs();
                    let y = (j as f64 - (self.window.height as f64 / 2.)).abs();
                    let dist = (x * x + y * y).sqrt() / self.camera.scale;
                    if dist < celestial.rad() {
                        *item = char.clone();
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

impl Window for CameraWindow {
    fn render(&self) -> Vec<Vec<String>> {
        let mut render =
            vec![vec![" ".to_string(); self.window.width]; self.window.height];

        let world = self.camera.world.borrow().get();

        let focus = match &world[&self.camera.focus] {
            Body::Celestial(c) => {
                self.draw_focus_body(c, &mut render);
                c.pos()
            }
            Body::Spaceship(ss) => ss.pos(),
        };

        for body in world.values() {
            let (name, pos) = match body {
                Body::Celestial(c) => (c.name(), c.pos()),
                Body::Spaceship(ss) => (ss.name(), ss.pos()),
            };

            let x =
                (&pos - &focus) * &self.camera.x_dir * self.camera.scale * 2.
                    + self.window.width as f64 / 2.;
            let y = (&focus - &pos) * &self.camera.y_dir * self.camera.scale
                + self.window.height as f64 / 2.;

            if self.window.inside(x, y) {
                let char = Self::get_symbol(&name);
                if (&char != "∘" && &char != "I")
                    || render[y as usize][x as usize] == " "
                {
                    render[y as usize][x as usize] = char;
                }
            }
        }

        render
    }

    fn position(&self) -> (usize, usize) {
        (self.window.x, self.window.y)
    }
}

#[derive(Clone)]
pub struct Rectangle {
    pub width: usize,
    pub height: usize,
    pub x: usize,
    pub y: usize,
}

impl Rectangle {
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
