use crate::tui::frame::Frame;
use crate::tui::intro::Intro;
use crate::tui::window::Window;
use crate::world::Body;
use crate::{Celestial, World};
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::watch::Receiver;
use tokio::time::interval;

pub struct Tui {
    fps: u32,
    world: Receiver<World>,
    frame: Frame,
    windows: Vec<Window>,
}

impl Tui {
    pub async fn init(
        world: Receiver<World>,
        fps: u32,
        intro_secs: u64,
    ) -> Result<Self, String> {
        let frame = Frame::new(" ".to_string())?;

        if intro_secs > 0 {
            let mut intro = Intro::new(Duration::from_secs(intro_secs), fps)?;
            intro.run().await?;
        }

        Ok(Self {
            fps,
            world,
            frame,
            windows: Vec::new(),
        })
    }

    pub async fn run(mut self) -> Result<(), String> {
        let mut interval = interval(Duration::from_millis(
            (1. / self.fps as f64 * 1000.) as u64,
        ));

        loop {
            interval.tick().await;

            self.draw_frame()?;

            self.frame.flush();
        }
    }

    pub fn add_window(&mut self, window: Window) {
        self.windows.push(window);
    }

    fn draw_frame(&mut self) -> Result<(), String> {
        self.frame = Frame::new("#".to_string())?;

        let world = self.world.borrow().get();

        for window in &self.windows.clone() {
            self.draw_window(window, &world);
        }

        Ok(())
    }

    fn draw_window(&mut self, window: &Window, world: &HashMap<String, Body>) {
        for x in window.x..(window.x + window.width) {
            for y in window.y..(window.y + window.height) {
                self.frame.try_set_usize(x, y, " ".to_string());
            }
        }

        let focus = match &world[&window.focus] {
            Body::Celestial(c) => {
                self.draw_focus_body(c, window);
                c.pos()
            }
            Body::Spaceship(ss) => ss.pos(),
        };

        for body in world.values() {
            let (name, pos) = match body {
                Body::Celestial(c) => (c.name(), c.pos()),
                Body::Spaceship(ss) => (ss.name(), ss.pos()),
            };

            let mut x = (&pos - &focus) * &window.x_dir * window.scale * 2.
                + window.width as f64 / 2.;
            let mut y = (&focus - &pos) * &window.y_dir * window.scale
                + window.height as f64 / 2.;

            if !window.inside(x, y) {
                continue;
            }

            x += window.x as f64;
            y += window.y as f64;

            let char = Self::get_symbol(&name);
            self.frame.try_set(x, y, char);
        }
    }

    fn draw_focus_body(&mut self, celestial: &Celestial, window: &Window) {
        let char = Self::get_symbol(&celestial.name());

        if window.scale * celestial.rad() > 1. {
            for i in 0..(window.width) {
                for j in 0..(window.height) {
                    let x = ((i as f64 - (window.width as f64 / 2.)) / 2.).abs();
                    let y = (j as f64 - (window.height as f64 / 2.)).abs();
                    let dist = (x * x + y * y).sqrt() / window.scale;
                    if dist < celestial.rad() {
                        self.frame.try_set_usize(i + window.x, j + window.y, char.clone());
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
