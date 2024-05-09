use crate::gui::control::ControlMessage;
use crate::gui::control::{Control, ControlFlow};
use crate::utils::Vec3;
use crate::world::celestials::Celestial;
use crate::world::spaceship::Spaceship;
use crate::world::{Body, World};
use embedded_graphics::geometry::OriginDimensions;
use embedded_graphics::mono_font::ascii::FONT_5X7;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::prelude::{DrawTarget, Point, Primitive, Size};
use embedded_graphics::primitives::{Circle, PrimitiveStyle, Rectangle};
use embedded_graphics::text::Text;
use embedded_graphics::{pixelcolor::BinaryColor, Drawable};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, watch};

pub struct Camera {
    pub x_dir: Vec3,
    pub y_dir: Vec3,
}

pub struct Gui {
    fps: f64,
    control: Control,
    display: SimulatorDisplay<BinaryColor>,
    camera: Camera,
    focus: Vec3,
    focus_name: String,
    world_watch: watch::Receiver<World>,
}

impl Gui {
    pub fn new(
        fps: f64,
        world_watch: watch::Receiver<World>,
        control_sender: mpsc::Sender<ControlMessage>,
    ) -> Self {
        Self {
            fps,
            control: Control::new(control_sender),
            display: SimulatorDisplay::<BinaryColor>::new(Size::new(400, 200)),
            camera: Camera {
                x_dir: Vec3 {
                    x: 1.,
                    y: 0.,
                    z: 0.,
                },
                y_dir: Vec3 {
                    x: 0.,
                    y: 1.,
                    z: 0.,
                },
            },
            focus: Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            focus_name: "Earth".to_string(),
            world_watch,
        }
    }

    pub fn run(mut self) -> Result<(), String> {
        let output_settings = OutputSettingsBuilder::new()
            .theme(BinaryColorTheme::OledBlue)
            .build();
        let mut window = Window::new("Voida", &output_settings);
        window.update(&mut self.display);

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

            self.display.clear(BinaryColor::Off).unwrap();

            let world = self.world_watch.borrow().clone();
            let bodies = world.get_bodies();

            self.get_focus(&bodies);

            for body in bodies.values() {
                match body {
                    Body::Celestial(c) => {
                        self.draw_celestial(c);
                    }
                    Body::Spaceship(s) => {
                        self.draw_spaceship(s);
                    }
                }
            }

            let text_style = MonoTextStyle::new(&FONT_5X7, BinaryColor::On);
            Text::new(
                &format!("sim fps: {}", world.true_sim_fps),
                Point::new(2, 6),
                text_style,
            )
            .draw(&mut self.display)
            .unwrap();
            Text::new(
                &format!("gui fps: {}", fps_reporter),
                Point::new(2, 13),
                text_style,
            )
            .draw(&mut self.display)
            .unwrap();
            let display_x = self.control.rmb_coords.0 as f64;
            let display_y = self.control.rmb_coords.1 as f64;
            let vec = self.frame_to_world(display_x, display_y);
            Text::new(
                &format!(
                    "rmb: {} {}, {:.2} {:.2}",
                    display_x,
                    display_y,
                    vec.x / self.control.scale,
                    vec.y / self.control.scale,
                ),
                Point::new(2, 20),
                text_style,
            )
            .draw(&mut self.display)
            .unwrap();

            window.update(&self.display);
        }
    }

    fn frame_to_world(&self, x_frame: f64, y_frame: f64) -> Vec3 {
        let size = self.display.size();
        let width = size.width as f64;
        let height = size.height as f64;
        let x_world = (x_frame - width / 2. - self.control.shift.x)
            * self.control.scale
            + self.focus.x;
        let y_world = -(y_frame - height / 2. - self.control.shift.y)
            * self.control.scale
            + self.focus.y;
        Vec3 {
            x: x_world,
            y: y_world,
            z: 0.,
        }
    }

    fn world_to_frame(&self, pos_world: &Vec3) -> (f64, f64) {
        let size = self.display.size();
        let width = size.width as f64;
        let height = size.height as f64;
        let x_frame = (pos_world.x - &self.focus.x)
            // * &self.camera.x_dir
            / self.control.scale
            + width / 2.
            + self.control.shift.x;
        let y_frame = (&self.focus.y - pos_world.y)
            // * &self.camera.y_dir
            / self.control.scale
            + height / 2.
            + self.control.shift.y;
        (x_frame, y_frame)
    }

    fn get_focus(&mut self, bodies: &HashMap<String, Body>) {
        let size = self.display.size();
        if let Some((display_x, display_y)) = self.control.change_focus {
            let x = (display_x as f64
                - self.control.shift.x
                - size.width as f64 / 2.)
                * self.control.scale
                + self.focus.x;
            let y = (size.height as f64 / 2. - display_y as f64
                + self.control.shift.y)
                * self.control.scale
                + self.focus.y;
            let mut min_distance = f64::MAX;
            for (name, body) in bodies.iter() {
                let pos = body.pos();
                let distance =
                    ((pos.x - x).powi(2) + (pos.y - y).powi(2)).sqrt();
                if distance < min_distance {
                    min_distance = distance;
                    self.focus_name = name.clone();
                }
            }
        }

        self.focus = bodies.get(&self.focus_name).unwrap().pos();
    }

    fn draw_celestial(&mut self, c: &Celestial) {
        let size = self.display.size();
        let (width, height) = (size.width as f64, size.height as f64);
        let (x, y) = self.world_to_frame(&c.pos());
        let rad = c.rad() / self.control.scale;
        if x < -width / 2. && x + rad < -width / 2.
            || x > width / 2. && x - rad > width / 2.
            || y < -height / 2. && y + rad < -height / 2.
            || y > height / 2. && y - rad > height / 2.
        {
            return;
        }

        let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
        let mut r_u = (c.rad() / self.control.scale) as u32;
        if r_u == 0 {
            r_u = 1;
        }
        let r_i = r_u as i32;
        Circle::new(
            Point::new(
                ((c.pos().x - self.focus.x) / self.control.scale) as i32
                    + width as i32 / 2
                    - r_i
                    + self.control.shift.x as i32,
                ((self.focus.y - c.pos().y) / self.control.scale) as i32
                    + height as i32 / 2
                    - r_i
                    + self.control.shift.y as i32,
            ),
            r_u * 2,
        )
        .into_styled(line_style)
        .draw(&mut self.display)
        .unwrap();
    }

    fn draw_spaceship(&mut self, s: &Spaceship) {
        let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

        Rectangle::new(
            Point::new(
                ((s.pos().x - self.focus.x) / self.control.scale + 195.)
                    as i32
                    + self.control.shift.x as i32,
                ((self.focus.y - s.pos().y) / self.control.scale + 95.)
                    as i32
                    + self.control.shift.y as i32,
            ),
            Size::new(10, 10),
        )
        .into_styled(line_style)
        .draw(&mut self.display)
        .unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::world::celestials::Celestials;
    use approx::assert_abs_diff_eq;
    use crate::gui::control::Shift;

    fn setup() -> Gui {
        let (control_sender, _) = mpsc::channel(100);
        let (_, world_receiver) =
            watch::channel(World::new(Celestials::new(), Default::default()));
        Gui::new(20., world_receiver, control_sender)
    }

    #[test]
    fn test_conversions() {
        let mut gui = setup();
        let pos = Vec3 {
            x: 1_000_000.,
            y: 1_000_000.,
            z: 0.,
        };

        gui.control.scale = 1.5;
        gui.control.shift = Shift {
            x: 1.,
            y: 315.,
            mouse: None,
        };

        let (x, y) = gui.world_to_frame(&pos);
        let pos2 = gui.frame_to_world(x, y);
        println!("{:?}\n{:?}", pos, pos2);
        assert!(pos.equal_to(&pos2, 0.001));

        let pos = Vec3 {
            x: 1.521_f64 * 10_f64.powi(11),
            y: 1_000_000.,
            z: 0.,
        };
        let (x, _) = gui.world_to_frame(&pos);
        assert_abs_diff_eq!(x, 1521200.);
    }
}
