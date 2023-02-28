use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::watch::Receiver;
use tokio::time::Instant;
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

pub async fn run(rx: Receiver<HashMap<String, Body>>) {
    let scale = 10_f64 / 2_f64 / 10_f64.powi(11);
    let start = Instant::now();
    let period = Duration::from_millis(20);
    let mut wake = start + period;

    loop {
        let now = Instant::now();
        if wake > now {
            tokio::time::sleep(wake - now).await;
        }
        wake = now + period;

        let mut map = vec![vec![" ".to_string(); 82]; 42];
        for i in 0..82 {
            map[41][i] = "-".to_string();
            if i < 42 {
                map[i][81] = "|".to_string();
            }
        }
        map[41][81] = "/".to_string();

        for (_, body) in &*rx.borrow() {
            let char = if body.name() == "Sun".to_string() {
                "O".to_string()
            } else if body.name() == "Earth".to_string() {
                "o".to_string()
            } else if body.name() == "Moon".to_string() {
                "°".to_string()
            } else {
                "X".to_string()
            };

            let x_f64 = body.pos().x * scale * 2. + 40.;
            let y_f64 = body.pos().y * scale + 20.;

            if x_f64 > 81. || x_f64 < 0. || y_f64 > 41. || y_f64 < 0. {
                continue;
            }

            let (x, y) = (x_f64 as usize, 40 - y_f64 as usize);

            map[y][x] = char;
        }
        draw(&map);
    }
}