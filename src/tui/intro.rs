mod particle;
mod logo;

use crate::tui::frame::Frame;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::time::Duration;
use termion::terminal_size;
use tokio::time::{interval, Instant};
use crate::tui::intro::logo::Logo;
use crate::tui::intro::particle::Particle;

pub struct Intro {
    frame: Frame,
    fps: u32,
    duration: Duration,
    particles: Vec<Particle>,
    logo: Logo,
}

impl Intro {
    pub fn new(duration: Duration, fps: u32) -> Result<Self, String> {
        let (x, y) = Self::get_terminal_size()?;
        let frame = Frame::new(x, y);
        let particles = Vec::with_capacity(1000);
        let mut logo = Logo::voida();
        logo.frame_center(x, y);

        Ok(Self {
            frame,
            fps,
            duration,
            particles,
            logo,
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

        let (x, y) = Self::get_terminal_size()?;
        self.draw_full_frame(x, y);

        let mut interval = interval(Duration::from_millis(
            (1. / self.fps as f64 * 1000.) as u64,
        ));

        loop {
            interval.tick().await;

            Particle::spawn_into(
                &mut self.particles,
                3,
                self.frame.width,
                self.frame.height,
                start,
            );

            let (x, y) = Self::get_terminal_size()?;
            if self.terminal_size_changed(x, y) {
                self.draw_full_frame(x, y);
            } else {
                self.redraw_frame();
            }

            self.frame.flush();

            if start.elapsed() > self.duration {
                break;
            }
        }
        Ok(())
    }

    fn terminal_size_changed(&mut self, x: usize, y: usize) -> bool {
        x != self.frame.width || y != self.frame.height
    }

    fn draw_full_frame(&mut self, x: usize, y: usize) {
        self.particles.clear();
        Particle::spawn_into(
            &mut self.particles,
            30,
            self.frame.width,
            self.frame.height,
            Instant::now(),
        );

        self.frame = Frame::new(x, y);
        self.draw_particles();
        self.draw_logo();
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
        let name: Vec<char> = self.logo.name().chars().collect();
        self.logo.frame_center(self.frame.width, self.frame.height);
        let (start_x, start_y, width, height) = self.logo.get_params();

        for i in 0..height {
            for j in 0..width {
                let index = i * (width + 2) + j;
                if self.frame.inside(j + start_x, i + start_y) {
                    self.frame.vec[i + start_y][j + start_x] =
                        name[index].to_string();
                }
            }
        }
    }

    fn redraw_frame(&mut self) {
        self.redraw_particles();
    }

    fn redraw_particles(&mut self) {
        self.particles.retain_mut(|p|
            if p.x < 0.
                || p.y < 0.
                || !self.frame.inside(p.x as usize, p.y as usize)
            {
                false
            } else {
                let x = p.x as usize;
                let y = p.y as usize;
                if !self.logo.inside(x, y) {
                    self.frame.vec[y][x] = " ".to_string();
                }

                p.fly();

                if p.x < 0.
                    || p.y < 0.
                    || !self.frame.inside(p.x as usize, p.y as usize)
                {
                    false
                } else {
                    let x = p.x as usize;
                    let y = p.y as usize;
                    if !self.logo.inside(x, y) {
                        self.frame.vec[y][x] = p.symbol.clone();
                    }
                    true
                }
            }
        );
    }

    fn get_terminal_size() -> Result<(usize, usize), String> {
        let (x, y) = terminal_size().map_err(|e| format!("{}", e))?;
        Ok((x as usize, y as usize - 1))
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
