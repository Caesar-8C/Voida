use super::{Rectangle, Window};
use crate::world::Body;
use crate::{Celestial, Vec3, World};
use tokio::sync::watch::Receiver;

pub struct CameraWindow {
    pub window: Rectangle,
    pub camera: Camera,
}

impl CameraWindow {
    fn draw_circle(
        &self,
        center: (f64, f64),
        radius: f64,
        render: &mut [Vec<char>],
        char: char,
    ) {
        let r_i32 = radius as i32;
        for i in -2 * r_i32..(2 * r_i32 + 1) {
            for j in -r_i32..(r_i32 + 1) {
                if ((i * i) as f64) / 4. + ((j * j) as f64)
                    < (radius * radius) + 1.
                {
                    let x = i as f64 + center.0;
                    let y = j as f64 + center.1;
                    if self.window.inside(x, y) {
                        render[y as usize][x as usize] = char;
                    }
                }
            }
        }
    }

    fn draw_celestial(
        &self,
        celestial: &Celestial,
        x: f64,
        y: f64,
        render: &mut [Vec<char>],
    ) {
        let char = Self::get_symbol(&celestial.name());
        let center = (x, y);
        let radius = celestial.rad() * self.camera.scale;

        if x.abs() < 2. * radius + self.window.width as f64
            && y.abs() < radius + self.window.height as f64
        {
            self.draw_circle(center, radius, render, char);
        }
    }

    fn get_symbol(name: &str) -> char {
        match name {
            "Sun" => 'O',
            "Earth" => 'o',
            "Moon" => '∘',
            "ISS" => 'I',
            _ => 'X',
        }
    }
}

impl Window for CameraWindow {
    fn render(&mut self) -> Vec<Vec<char>> {
        let mut render =
            vec![vec![' '; self.window.width]; self.window.height];

        let world = self.camera.world.borrow().get();

        let focus = world[&self.camera.focus].pos();

        for body in world.values() {
            let (name, pos) = (body.name(), body.pos());

            let x =
                (&pos - &focus) * &self.camera.x_dir * self.camera.scale * 2.
                    + self.window.width as f64 / 2.;
            let y = (&focus - &pos) * &self.camera.y_dir * self.camera.scale
                + self.window.height as f64 / 2.;

            match body {
                Body::Celestial(c) => {
                    self.draw_celestial(c, x, y, &mut render);
                }
                Body::Spaceship(_) => {
                    if self.window.inside(x, y) {
                        let char = Self::get_symbol(&name);
                        if (char != '∘' && char != 'I')
                            || render[y as usize][x as usize] == ' '
                        {
                            render[y as usize][x as usize] = char;
                        }
                    }
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
pub struct Camera {
    pub scale: f64,
    pub focus: String,
    pub x_dir: Vec3,
    pub y_dir: Vec3,
    pub world: Receiver<World>,
}
