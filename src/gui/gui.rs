use crate::gui::Control;
use crate::world::Body::Celestial;
use crate::world::World;
use embedded_graphics::mono_font::ascii::FONT_6X9;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::prelude::{DrawTarget, Point, Primitive, Size};
use embedded_graphics::primitives::{Circle, PrimitiveStyle, Rectangle};
use embedded_graphics::text::Text;
use embedded_graphics::{pixelcolor::BinaryColor, Drawable};
use embedded_graphics_simulator::sdl2::{Keycode, MouseButton};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent,
    Window,
};
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, watch};

struct Shift {
    pub x: i32,
    pub y: i32,
    pub mouse_x: u32,
    pub mouse_y: u32,
    pub pressed: bool,
}

pub struct Gui {
    fps: f64,
    controller: mpsc::Sender<Control>,
    shift: Shift,
}

impl Gui {
    pub fn new(fps: f64, controller: mpsc::Sender<Control>) -> Self {
        Self {
            fps,
            controller,
            shift: Shift {
                x: 0,
                y: 0,
                mouse_x: 0,
                mouse_y: 0,
                pressed: false,
            },
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

                for event in window.events() {
                    match event {
                        SimulatorEvent::Quit => {
                            self.controller
                                .blocking_send(Control::Shutdown)
                                .map_err(|e| e.to_string())?;
                            return Ok(());
                        }
                        SimulatorEvent::KeyDown { keycode, .. } => {
                            match keycode {
                                Keycode::Q => {
                                    self.controller
                                        .blocking_send(Control::Shutdown)
                                        .map_err(|e| e.to_string())?;
                                    return Ok(());
                                }
                                _ => {}
                            }
                        }
                        SimulatorEvent::MouseButtonDown { mouse_btn, point} => {
                            if mouse_btn == MouseButton::Middle {
                                self.shift.mouse_x = point.x as u32;
                                self.shift.mouse_y = point.y as u32;
                                self.shift.pressed = true;
                            }
                        }
                        SimulatorEvent::MouseButtonUp { mouse_btn, ..} => {
                            if mouse_btn == MouseButton::Middle {
                                self.shift.pressed = false;
                            }
                        }
                        SimulatorEvent::MouseMove { point } => {
                            if self.shift.pressed {
                                self.shift.x += point.x - self.shift.mouse_x as i32;
                                self.shift.y += point.y - self.shift.mouse_y as i32;
                                self.shift.mouse_x = point.x as u32;
                                self.shift.mouse_y = point.y as u32;
                            }
                        }
                        _ => {}
                    }
                }

                display.clear(BinaryColor::Off).unwrap();

                let r_u = (r / 100_000.) as u32;
                let r_i = r_u as i32;
                Circle::new(Point::new(200 - r_i + self.shift.x, 100 - r_i + self.shift.y), r_u * 2)
                    .into_styled(line_style)
                    .draw(&mut display)
                    .unwrap();
                Rectangle::new(
                    Point::new(
                        ((x_i - x_e) / 100_000. + 195.) as i32 + self.shift.x,
                        ((y_e - y_i) / 100_000. + 95.) as i32 + self.shift.y,
                    ),
                    Size::new(10, 10),
                )
                .into_styled(line_style)
                .draw(&mut display)
                .unwrap();
                Rectangle::new(
                    Point::new(
                        ((x_2 - x_e) / 100_000. + 195.) as i32 + self.shift.x,
                        ((y_e - y_2) / 100_000. + 95.) as i32 + self.shift.y,
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
