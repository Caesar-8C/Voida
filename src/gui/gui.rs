use crate::world::World;
use embedded_graphics::prelude::{DrawTarget, Point, Primitive, Size};
use embedded_graphics::primitives::{Circle, PrimitiveStyle, Rectangle};
use embedded_graphics::{pixelcolor::BinaryColor, Drawable};
use embedded_graphics_simulator::sdl2::Keycode;
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent,
    Window,
};
use std::time::{Duration, Instant};
use tokio::sync::watch;
use crate::world::Body::Celestial;

pub struct Gui {
    fps: f64,
}

impl Gui {
    pub fn new(fps: f64) -> Self {
        Self { fps }
    }

    pub fn run(self, world: watch::Receiver<World>) {
        let mut display =
            SimulatorDisplay::<BinaryColor>::new(Size::new(400, 200));
        let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

        let output_settings = OutputSettingsBuilder::new()
            .theme(BinaryColorTheme::OledBlue)
            .build();
        let mut window = Window::new("Hello World", &output_settings);
        window.update(&display);

        let period = Duration::from_secs_f64(1. / self.fps);
        let mut next_wake = Instant::now();

        loop {
            let now = Instant::now();
            if now < next_wake {
                std::thread::sleep(next_wake - now);
            }
            next_wake += period;

            let mut x_i = 0.0;
            let mut y_i = 0.0;
            let mut x_2 = 0.0;
            let mut y_2 = 0.0;
            let mut x_e = 0.0;
            let mut y_e = 0.0;
            let mut r = 0.0;

            if world.has_changed().unwrap_or(false) {
                let world = world.borrow().get();

                for body in world.values() {
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

                for event in window.events() {
                    match event {
                        SimulatorEvent::Quit => std::process::exit(0),
                        SimulatorEvent::KeyDown { keycode, .. } => {
                            match keycode {
                                Keycode::Q => {
                                    std::process::exit(0);
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }

                display.clear(BinaryColor::Off).unwrap();

                let r_u = (r/100_000.) as u32;
                let r_i = r_u as i32;
                Circle::new(Point::new(200 - r_i, 100 - r_i), r_u*2)
                    .into_styled(line_style)
                    .draw(&mut display)
                    .unwrap();
                Rectangle::new(
                    Point::new(
                        ((x_i - x_e) / 100_000. + 195.) as i32,
                        ((y_e - y_i) / 100_000. + 95.) as i32,
                    ),
                    Size::new(10, 10),
                )
                .into_styled(line_style)
                .draw(&mut display)
                .unwrap();
                Rectangle::new(
                    Point::new(
                        ((x_2 - x_e) / 100_000. + 195.) as i32,
                        ((y_e - y_2) / 100_000. + 95.) as i32,
                    ),
                    Size::new(10, 10),
                )
                .into_styled(line_style)
                .draw(&mut display)
                .unwrap();
                window.update(&display);
            }
        }
    }
}
