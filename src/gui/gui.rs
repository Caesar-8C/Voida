use crate::gui::control::{Control, ControlFlow};
use crate::world::Body::Celestial;
use crate::world::World;
use embedded_graphics::mono_font::ascii::FONT_6X9;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::prelude::{DrawTarget, Point, Primitive, Size};
use embedded_graphics::primitives::{Circle, PrimitiveStyle, Rectangle};
use embedded_graphics::text::Text;
use embedded_graphics::{pixelcolor::BinaryColor, Drawable};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay,
    Window,
};
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, watch};
use crate::gui::control::ControlMessage;

pub struct Gui {
    fps: f64,
    control: Control,
}

impl Gui {
    pub fn new(fps: f64, control_sender: mpsc::Sender<ControlMessage>) -> Self {
        Self {
            fps,
            control: Control::new(control_sender),
        }
    }

    pub fn run(mut self, world: watch::Receiver<World>) -> Result<(), String> {
        let mut display =
            SimulatorDisplay::<BinaryColor>::new(Size::new(400, 200));
        let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
        let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);

        let output_settings = OutputSettingsBuilder::new()
            .theme(BinaryColorTheme::OledBlue)
            .build();
        let mut window = Window::new("Hello World", &output_settings);
        window.update(&display);

        let period = Duration::from_secs_f64(1. / self.fps);
        let mut next_wake = Instant::now();

        let mut start = Instant::now();
        let mut fps_counter = 0;
        let mut fps_reporter = 0;

        loop {
            let now = Instant::now();
            if now < next_wake {
                std::thread::sleep(next_wake - now);
            }
            next_wake += period;
            fps_counter += 1;
            if start.elapsed().as_secs() >= 1 {
                fps_reporter = fps_counter;
                start = Instant::now();
                fps_counter = 0;
            }

            if let ControlFlow::Break = self.control.update(window.events())? {
                return Ok(());
            }

            let mut x_i = 0.0;
            let mut y_i = 0.0;
            let mut x_2 = 0.0;
            let mut y_2 = 0.0;
            let mut x_e = 0.0;
            let mut y_e = 0.0;
            let mut r = 0.0;

            if world.has_changed().unwrap_or(false) {
                let world = world.borrow().clone();
                let counter = world.true_sim_fps;
                let map = world.get();

                for body in map.values() {
                    let (name, pos) = (body.name(), body.pos());
                    if name == "ISS" {
                        x_i = pos.x;
                        y_i = pos.y;
                    }
                    if name == "ISS2" {
                        x_2 = pos.x;
                        y_2 = pos.y;
                    }
                    if name == "Earth" {
                        x_e = pos.x;
                        y_e = pos.y;
                        if let Celestial(c) = body {
                            r = c.rad();
                        }
                    }
                }

                display.clear(BinaryColor::Off).unwrap();

                let r_u = (r / self.control.scale()) as u32;
                let r_i = r_u as i32;
                Circle::new(
                    Point::new(
                        200 - r_i + self.control.shift().x,
                        100 - r_i + self.control.shift().y,
                    ),
                    r_u * 2,
                )
                .into_styled(line_style)
                .draw(&mut display)
                .unwrap();
                Rectangle::new(
                    Point::new(
                        ((x_i - x_e) / self.control.scale() + 195.) as i32 + self.control.shift().x,
                        ((y_e - y_i) / self.control.scale() + 95.) as i32 + self.control.shift().y,
                    ),
                    Size::new(10, 10),
                )
                .into_styled(line_style)
                .draw(&mut display)
                .unwrap();
                Rectangle::new(
                    Point::new(
                        ((x_2 - x_e) / self.control.scale() + 195.) as i32 + self.control.shift().x,
                        ((y_e - y_2) / self.control.scale() + 95.) as i32 + self.control.shift().y,
                    ),
                    Size::new(10, 10),
                )
                .into_styled(line_style)
                .draw(&mut display)
                .unwrap();
                Text::new(
                    &format!("true sim fps: {}", counter),
                    Point::new(2, 6),
                    text_style,
                )
                .draw(&mut display)
                .unwrap();
                Text::new(
                    &format!("true gui fps: {}", fps_reporter),
                    Point::new(2, 13),
                    text_style,
                )
                .draw(&mut display)
                .unwrap();
                window.update(&display);
            }
        }
    }
}
