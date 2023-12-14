use crate::tui::frame::Frame;
use crate::tui::intro::Intro;
use std::time::Duration;
use tokio::time::interval;
use crate::window::Window;

pub struct Tui {
    fps: u32,
    frame: Frame,
    windows: Vec<Box<dyn Window>>,
}

impl Tui {
    pub async fn init(
        fps: u32,
        intro_secs: u64,
    ) -> Result<Self, String> {
        let frame = Frame::new(' ')?;

        if intro_secs > 0 {
            let mut intro = Intro::new(Duration::from_secs(intro_secs), fps)?;
            intro.run().await?;
        }

        Ok(Self {
            fps,
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

            self.frame = Frame::new('#')?;
            for window in self.windows.iter_mut() {
                let render = window.render();
                let (x, y) = window.position();
                self.frame.try_set_window(x, y, render);
            }

            self.frame.flush();
        }
    }

    pub fn add_window(&mut self, window: Box<dyn Window>) {
        self.windows.push(window);
    }

}
