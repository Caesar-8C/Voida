use crate::celestials::Celestial;
use crate::Vec3;
use std::collections::HashMap;
use std::io::stdin;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use tokio::sync::watch;
use tokio::sync::watch::{Receiver, Sender};
use tokio::time::interval;

async fn listen_keys(view_sender: Sender<String>) {
    let stdin = stdin();
    let mut view = "global".to_string();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('g') => {
                view = "global".to_string();
            }
            Key::Char('e') => {
                view = "earth".to_string();
            }
            Key::Char('i') => {
                view = "iss".to_string();
            }
            _ => (),
        };
        view_sender.send(view.clone()).unwrap();
    }
}

pub struct TUI {
    fps: u32,
    world: Receiver<HashMap<String, Celestial>>,
    view: Receiver<String>,
    scales: HashMap<String, f64>,
}

impl TUI {
    pub async fn init(
        world: Receiver<HashMap<String, Celestial>>,
        fps: u32,
    ) -> Self {
        let (user_input_tx, view) = watch::channel("global".to_string());
        tokio::spawn(listen_keys(user_input_tx));

        let mut scales = HashMap::new();
        scales.insert("global".to_string(), 10_f64 / 10_f64.powi(11));
        scales.insert("earth".to_string(), 10_f64 / 8_f64 / 10_f64.powi(8));
        scales.insert("iss".to_string(), 10_f64 / 8_f64 / 10_f64.powi(8));

        Self {
            fps,
            world,
            view,
            scales,
        }
    }

    pub async fn run(self) {
        let mut interval = interval(Duration::from_millis(
            (1. / self.fps as f32 * 1000.) as u64,
        ));

        loop {
            interval.tick().await;

            let world_copy = self.world.borrow().clone();
            let view: &str = &self.view.borrow();
            let focus = match view {
                "global" => Vec3::default(),
                "earth" => world_copy["Earth"].pos(),
                "iss" => world_copy["Earth"].pos(),
                _ => return,
            };

            let mut map = self.construct_map();
            self.draw_celestials(&mut map, world_copy, view, focus);

            self.flush(&map);
        }
    }

    fn construct_map(&self) -> Vec<Vec<String>> {
        let mut map = vec![vec![" ".to_string(); 82]; 42];
        for i in 0..82 {
            map[41][i] = "-".to_string();
            if i < 42 {
                map[i][81] = "|".to_string();
            }
        }
        map[41][81] = "/".to_string();
        map
    }

    fn draw_celestials(
        &self,
        map: &mut Vec<Vec<String>>,
        world: HashMap<String, Celestial>,
        view: &str,
        focus: Vec3,
    ) {
        for (_, celestial) in &world {
            let char = Self::get_symbol(&celestial.name());

            let x_f64 =
                (celestial.pos().x - focus.x) * self.scales[view] * 2. + 40.;
            let y_f64 = (celestial.pos().y - focus.y) * self.scales[view] + 20.;

            if x_f64 > 81. || x_f64 < 0. || y_f64 > 41. || y_f64 < 0. {
                continue;
            }

            let (x, y) = (x_f64 as usize, 40 - y_f64 as usize);

            if &char != "∘" || &map[y][x] == " " {
                map[y][x] = char;
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

    fn flush(&self, map: &Vec<Vec<String>>) {
        let mut st = "".to_string();
        for first in map {
            for second in first {
                st += second;
            }
            st += "\n";
        }
        print!("{}c{}", 27 as char, st);
    }
}
