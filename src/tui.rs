pub mod window;
mod frame;

use crate::world::celestials::Celestial;
use std::collections::HashMap;
use std::time::Duration;
use termion::terminal_size;
use tokio::sync::watch::Receiver;
use tokio::time::interval;
use crate::tui::frame::Frame;
use crate::tui::window::Window;

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
    ) -> Self {

        let (x, y) = terminal_size().unwrap();
        let frame = Frame::new(x as usize, y as usize - 1);

        Self {
            fps,
            world,
            frame,
            windows: Vec::new(),
        }
    }

    pub async fn run(mut self) {
        let mut interval = interval(Duration::from_millis(
            (1. / self.fps as f32 * 1000.) as u64,
        ));

        loop {
            interval.tick().await;

            self.draw_frame();

            self.flush();
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
                self.frame.vec[y][x] = " ".to_string();
            }
        }

        let world = &*self.world.borrow();
        let focus = world[&window.focus].pos();

        for celestial in world.values() {
            let char = Self::get_symbol(&celestial.name());

            let x_f64 =
                (&celestial.pos() - &focus) * &window.x_dir * window.scale * 2. + window.width as f64 / 2.;
            let y_f64 = (&focus - &celestial.pos()) * &window.y_dir * window.scale + window.height as f64 / 2.;

            if x_f64 > window.width as f64 || x_f64 < 0. || y_f64 > window.height as f64 || y_f64 < 0. {
                continue;
            }

            let (x, y) = (x_f64 as usize + window.x, y_f64 as usize + window.y);
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

    fn flush(&self) {
        let mut st = "".to_string();
        for first in &self.frame.vec {
            for second in first {
                st += second;
            }
            st += "\n";
        }
        print!("{}c{}", 27 as char, st);
    }
}
