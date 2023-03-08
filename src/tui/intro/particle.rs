use std::time::Duration;
use tokio::time::Instant;
use rand::Rng;

const SYMBOLS: [&str; 6] = ["#", "o", ".", ",", "*", "$"];

pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
    pub speed: f64,
    pub symbol: String,
}

impl Particle {
    pub fn spawn_into(
        container: &mut Vec<Particle>,
        amount: usize,
        x: usize,
        y: usize,
        start: Instant,
    ) {
        for _ in 0..amount {
            container.push(Particle::spawn(x, y, start));
        }
    }

    pub fn spawn(x: usize, y: usize, start: Instant) -> Self {
        let mut rng = rand::thread_rng();
        let mut angle: f64 = rng.gen();
        angle *= std::f64::consts::TAU;

        let random: f64 = rng.gen();
        let index = (random * SYMBOLS.len() as f64) as usize;

        let speed = if start.elapsed() > Duration::from_millis(500) {
            let mut speed: f64 = rng.gen();
            speed = speed * 2. + 1.;
            speed
        } else {
            2.
        };

        Self {
            x: x as f64 / 2.,
            y: y as f64 / 2.,
            angle,
            speed,
            symbol: SYMBOLS[index].to_string(),
        }
    }

    pub fn fly(&mut self) {
        self.x += self.speed * self.angle.cos() * 2.;
        self.y += self.speed * self.angle.sin() / 2.;
    }
}