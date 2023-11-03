use crate::tui::frame::Frame;
use crate::tui::intro::Intro;
use crate::tui::window::WindowType;
use std::time::Duration;
use tokio::time::interval;

pub struct Tui {
    fps: u32,
    frame: Frame,
    windows: Vec<WindowType>,
}

impl Tui {
    pub async fn init(
        fps: u32,
        intro_secs: u64,
    ) -> Result<Self, String> {
        let frame = Frame::new(" ".to_string())?;

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

            self.frame = Frame::new("#".to_string())?;
            for windowtype in &self.windows.clone() {
                let render = windowtype.render();
                if let WindowType::Camera{window, ..} = windowtype {
                    self.frame.try_set_window(window.x, window.y, render);
                }
            }

            self.frame.flush();
        }
    }

    pub fn add_window(&mut self, window: WindowType) {
        self.windows.push(window);
    }

}
