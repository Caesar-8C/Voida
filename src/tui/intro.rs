use crate::tui::frame::Frame;
use rand::Rng;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::time::Duration;
use termion::terminal_size;
use tokio::time::{interval, Instant};

const SYMBOLS: [&str; 6] = ["#", "o", ".", ",", "*", "$"];
const NAME: &str =
"                                                              dddddddd                  |
VVVVVVVV           VVVVVVVV                 iiii              d::::::d                  |
V::::::V           V::::::V                i::::i             d::::::d                  |
V::::::V           V::::::V                 iiii              d::::::d                  |
V::::::V           V::::::V                                   d:::::d                   |
 V:::::V           V:::::V  ooooooooooo   iiiiiii     ddddddddd:::::d   aaaaaaaaaaaaa   |
  V:::::V         V:::::V oo:::::::::::oo i:::::i   dd::::::::::::::d   a::::::::::::a  |
   V:::::V       V:::::V o:::::::::::::::o i::::i  d::::::::::::::::d   aaaaaaaaa:::::a |
    V:::::V     V:::::V  o:::::ooooo:::::o i::::i d:::::::ddddd:::::d            a::::a |
     V:::::V   V:::::V   o::::o     o::::o i::::i d::::::d    d:::::d     aaaaaaa:::::a |
      V:::::V V:::::V    o::::o     o::::o i::::i d:::::d     d:::::d   aa::::::::::::a |
       V:::::V:::::V     o::::o     o::::o i::::i d:::::d     d:::::d  a::::aaaa::::::a |
        V:::::::::V      o::::o     o::::o i::::i d:::::d     d:::::d a::::a    a:::::a |
         V:::::::V       o:::::ooooo:::::o i::::i d::::::ddddd::::::d a::::a    a:::::a |
          V:::::V        o:::::::::::::::o i::::i  d::::::::::::::::d a:::::aaaa::::::a |
           V:::V          oo:::::::::::oo i::::::i  d:::::::::ddd::::d a::::::::::aa:::a|
            VVV             ooooooooooo   iiiiiiii   ddddddddd   ddddd  aaaaaaaaaa  aaaa|";
const NAME_X: usize = 88;
const NAME_Y: usize = 17;

struct Particle {
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

pub struct Intro {
    frame: Frame,
    fps: u32,
    duration: Duration,
    particles: Vec<Particle>,
}

impl Intro {
    pub fn new(duration: Duration, fps: u32) -> Result<Self, String> {
        let (x, y) = terminal_size().map_err(|e| format!("{}", e))?;
        let frame = Frame::new(x as usize, y as usize - 1);
        let particles = Vec::with_capacity(1000);

        Ok(Self {
            frame,
            fps,
            duration,
            particles,
        })
    }

    pub async fn run(&mut self) -> Result<(), String> {
        let start = Instant::now();

        tokio::spawn(Self::play_sound(start, self.duration));

        Particle::spawn_into(
            &mut self.particles,
            30,
            self.frame.width,
            self.frame.height,
            start,
        );

        let mut interval = interval(Duration::from_millis(
            (1. / self.fps as f64 * 1000.) as u64,
        ));

        loop {
            interval.tick().await;
            self.frame.fill(" ".to_string());

            Particle::spawn_into(
                &mut self.particles,
                3,
                self.frame.width,
                self.frame.height,
                start,
            );

            self.draw_particles();
            self.draw_logo();

            self.frame.flush();

            if start.elapsed() > self.duration {
                break;
            }
        }
        Ok(())
    }

    fn draw_particles(&mut self) {
        self.particles.retain_mut(|p|
            if p.x < 0.
                || p.y < 0.
                || !self.frame.inside(p.x as usize, p.y as usize)
            {
                false
            } else {
                self.frame.vec[p.y as usize][p.x as usize] = p.symbol.clone();
                p.fly();
                true
            }
        );
    }

    fn draw_logo(&mut self) {
        let name: Vec<char> = NAME.to_string().chars().collect();
        let start_x =
            (self.frame.width as f64 / 2. - NAME_X as f64 / 2.) as usize;
        let start_y =
            (self.frame.height as f64 / 2. - NAME_Y as f64 / 2.) as usize;

        for i in 0..NAME_Y {
            for j in 0..NAME_X {
                let index = i * (NAME_X + 2) + j;
                self.frame.vec[i + start_y][j + start_x] =
                    name[index].to_string();
            }
        }
    }

    async fn play_sound(
        start: Instant,
        duration: Duration,
    ) -> Result<(), String> {
        let (_stream, handle) =
            OutputStream::try_default().map_err(|e| format!("{}", e))?;
        let sink = Sink::try_new(&handle).map_err(|e| format!("{}", e))?;
        let source_file = File::open("media/sound/intro.wav")
            .map_err(|e| format!("{}", e))?;
        let source = Decoder::new(source_file).map_err(|e| format!("{}", e))?;
        sink.append(source);

        loop {
            if start.elapsed() > duration {
                return Ok(());
            }
        }
    }
}
