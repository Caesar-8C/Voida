mod logo;
mod particle;

use crate::tui::frame::Frame;
use crate::tui::intro::logo::Logo;
use crate::tui::intro::particle::Particle;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::time::Duration;
use tokio::time::{interval, Instant};

pub struct Intro {
    frame: Frame,
    fps: u32,
    duration: Duration,
    particles: Vec<Particle>,
    logo: Logo,
}

impl Intro {
    pub fn new(duration: Duration, fps: u32) -> Result<Self, String> {
        let frame = Frame::new(" ".to_string())?;
        let particles = Vec::with_capacity(1000);
        let mut logo = Logo::voida();
        logo.frame_center(frame.width, frame.height);

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

        self.draw_full_frame()?;

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

            if self.frame.size_changed()? {
                self.draw_full_frame()?;
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

    fn draw_full_frame(&mut self) -> Result<(), String> {
        self.particles.clear();
        Particle::spawn_into(
            &mut self.particles,
            30,
            self.frame.width,
            self.frame.height,
            Instant::now(),
        );

        self.frame = Frame::new(" ".to_string())?;
        self.draw_particles();
        self.draw_logo();

        Ok(())
    }

    fn draw_particles(&mut self) {
        self.particles.retain_mut(|p| {
            let success = self.frame.try_set(p.x, p.y, p.symbol.clone());
            p.fly();
            success
        });
    }

    fn draw_logo(&mut self) {
        let name: Vec<char> = self.logo.name().chars().collect();
        self.logo.frame_center(self.frame.width, self.frame.height);
        let (start_x, start_y, width, height) = self.logo.get_params();

        for j in 0..height {
            for i in 0..width {
                let index = j * (width + 2) + i;
                self.frame.try_set_usize(
                    i + start_x,
                    j + start_y,
                    name[index].to_string(),
                );
            }
        }
    }

    fn redraw_frame(&mut self) {
        self.redraw_particles();
    }

    fn redraw_particles(&mut self) {
        self.particles.retain_mut(|p| {
            if !self.frame.inside(p.x, p.y) {
                false
            } else {
                if !self.logo.inside(p.x, p.y) {
                    self.frame.try_set(p.x, p.y, " ".to_string());
                }

                p.fly();

                if !self.logo.inside(p.x, p.y) {
                    self.frame.try_set(p.x, p.y, p.symbol.clone());
                }
                true
            }
        });
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
