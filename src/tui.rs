mod frame;
mod intro;
pub mod window;

use crate::tui::frame::Frame;
use crate::tui::intro::Intro;
use crate::tui::window::Window;
use crate::world::celestials::Celestial;
use std::collections::HashMap;
use std::time::Duration;
use termion::terminal_size;
use tokio::sync::watch::Receiver;
use tokio::time::interval;

pub struct Tui {
    fps: u32,
    world: Receiver<HashMap<String, Celestial>>,
    frame: Frame,
    windows: Vec<Window>,
}

impl Tui {
    pub async fn init(
        world: Receiver<HashMap<String, Celestial>>,
        fps: u32,
        intro_secs: u64,
    ) -> Result<Self, String> {
        let (x, y) = terminal_size().map_err(|e| format!("{}", e))?;
        let frame = Frame::new(x as usize, y as usize - 1);

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
            let (x, y) = terminal_size().map_err(|e| format!("{}", e))?;
            self.frame = Frame::new(x as usize, y as usize - 1);

            self.draw_frame();

            self.frame.flush();
        }
    }

    pub fn add_window(&mut self, window: Window) {
        self.windows.push(window);
    }

    fn draw_frame(&mut self) {
        self.frame.fill("#".to_string());

        for window in &self.windows.clone() {
            self.draw_window(window);
        }
    }

    fn draw_window(&mut self, window: &Window) {
        for x in window.x..(window.x + window.width) {
            for y in window.y..(window.y + window.height) {
                if !self.frame.inside(x, y) {
                    continue;
                }
                self.frame.vec[y][x] = " ".to_string();
            }
        }

        let world = &*self.world.borrow();
        let focus = world[&window.focus].pos();

        for celestial in world.values() {
            let char = Self::get_symbol(&celestial.name());

            let x_f64 =
                (&celestial.pos() - &focus) * &window.x_dir * window.scale * 2.
                    + window.width as f64 / 2.;
            let y_f64 =
                (&focus - &celestial.pos()) * &window.y_dir * window.scale
                    + window.height as f64 / 2.;

            if x_f64 < 0. || y_f64 < 0. {
                continue;
            }

            let mut x = x_f64 as usize;
            let mut y = y_f64 as usize;

            if !window.inside(x, y) {
                continue;
            }

            x += window.x;
            y += window.y;

            if !self.frame.inside(x, y) {
                continue;
            }

            if &char != "∘" || &self.frame.vec[y][x] == " " {
                self.frame.vec[y][x] = char;
            }
        }
    }

    fn get_symbol(name: &str) -> String {
        match name {
            "Sun" => "O".to_string(),
            "Earth" => "o".to_string(),
            "Moon" => "∘".to_string(),
            _ => "X".to_string(),
        }
    }
}
