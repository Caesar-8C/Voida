use std::collections::HashMap;
use std::io::stdin;
use std::time::Duration;
use tokio::sync::watch::{Receiver, Sender};
use tokio::sync::watch;
use tokio::time::Instant;
use termion::event::Key;
use termion::input::TermRead;
use crate::body::Body;

fn draw(map: &Vec<Vec<String>>) {
    let mut st = "".to_string();
    for first in map {
        for second in first {
            st += second;
        }
        st += "\n";
    }
    print!("{}c{}", 27 as char, st);
}

async fn listen_keys(earth_view_sender: Sender<bool>) {
    let stdin = stdin();
    let mut earth_view = false;
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('v') => {
                if earth_view {
                    earth_view = false;
                }
                else {
                    earth_view = true;
                }
            },
            _ => (),
        };
        earth_view_sender.send(earth_view).unwrap();
    }
}

pub async fn run(bodies: Receiver<HashMap<String, Body>>, fps: u32) {
    let (user_input_tx, earth_view) = watch::channel(false);
    tokio::spawn(listen_keys(user_input_tx));

    let sun_scale = 10_f64 / 10_f64.powi(11);
    let earth_scale = 10_f64 / 8_f64 / 10_f64.powi(8);
    let start = Instant::now();
    let period = Duration::from_millis((1. / fps as f32 * 1000.) as u64);
    let mut wake = start + period;

    loop {
        let now = Instant::now();
        if wake > now {
            tokio::time::sleep(wake - now).await;
        }
        wake = now + period;

        let scale = match *earth_view.borrow() {
            true => earth_scale,
            false => sun_scale,
        };

        let mut map = vec![vec![" ".to_string(); 82]; 42];
        for i in 0..82 {
            map[41][i] = "-".to_string();
            if i < 42 {
                map[i][81] = "|".to_string();
            }
        }
        map[41][81] = "/".to_string();

        let earth_pos = bodies.borrow()["Earth"].pos();

        for (_, body) in &*bodies.borrow() {
            let char = if body.name() == "Sun".to_string() {
                "O".to_string()
            } else if body.name() == "Earth".to_string() {
                "o".to_string()
            } else if body.name() == "Moon".to_string() {
                "∘".to_string()
            } else {
                "X".to_string()
            };

            let mut x_f64 = body.pos().x;
            let mut y_f64 = body.pos().y;

            if *earth_view.borrow() {
                x_f64 -= earth_pos.x;
                y_f64 -= earth_pos.y;
            }

            x_f64 = x_f64 * scale * 2. + 40.;
            y_f64 = y_f64 * scale + 20.;

            if x_f64 > 81. || x_f64 < 0. || y_f64 > 41. || y_f64 < 0. {
                continue;
            }

            let (x, y) = (x_f64 as usize, 40 - y_f64 as usize);

            map[y][x] = char;
        }
        draw(&map);
    }
}