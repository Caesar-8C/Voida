use crate::tui::frame::Frame;
use crate::tui::intro::Intro;
use crate::window::Window;
use std::time::Duration;
use tokio::time::interval;

pub struct Tui {
    fps: u32,
    frame: Frame,
    windows: Vec<Box<dyn Window>>,
}

impl Tui {
    pub async fn init(fps: u32, intro_secs: u64) -> Result<Self, String> {
        let frame = Frame::new('#')?;

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

            let mut force_redraw = false;

            if self.frame.size_changed()? {
                self.frame = Frame::new('#')?;
                force_redraw = true;
            }

            for window in self.windows.iter_mut() {
                let render = window.render(force_redraw);
                if let Some(render) = render {
                    let (x, y) = window.position();
                    self.frame.try_set_window(x, y, render);
                }
            }

            self.frame.flush();
        }
    }

    pub fn add_window(&mut self, window: Box<dyn Window>) {
        self.windows.push(window);
    }
}
