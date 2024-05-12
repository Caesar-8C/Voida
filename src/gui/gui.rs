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

pub struct Gui {
    fps: f64,
    control: Control,
    display: SimulatorDisplay<BinaryColor>,
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
        window.update(&self.display);

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
            let vec = self.display_to_world(display_x, display_y);
            let e = bodies.get(&self.focus_name).unwrap().pos();
            Text::new(
                &format!(
                    "rmb: {} {}, {:.2} {:.2}",
                    display_x,
                    display_y,
                    vec.x - e.x,
                    vec.y - e.y,
                ),
                Point::new(2, 20),
                text_style,
            )
            .draw(&mut self.display)
            .unwrap();

            window.update(&self.display);
        }
    }

    fn display_to_world(&self, x_display: f64, y_display: f64) -> Vec3 {
        let size = self.display.size();
        let width = size.width as f64;
        let height = size.height as f64;

        let x_frame = x_display - width / 2.;
        let y_frame = height / 2. - y_display;

        let look_at = &self.control.camera_extr_inv
            * &Vec3 {
                x: x_frame,
                y: y_frame,
                z: 0.,
            };

        look_at * self.control.scale + &self.focus - &self.control.shift.pos
    }

    fn world_to_display(&self, pos_world: &Vec3) -> (f64, f64) {
        let size = self.display.size();
        let width = size.width as f64;
        let height = size.height as f64;

        let look_at = (pos_world - &self.focus + &self.control.shift.pos)
            / self.control.scale;
        let frame = &self.control.camera_extr * &look_at;

        let x_display = frame.x + width / 2.;
        let y_display = height / 2. - frame.y;
        (x_display, y_display)
    }

    fn get_focus(&mut self, bodies: &HashMap<String, Body>) {
        if let Some((display_x, display_y)) = self.control.change_focus {
            let click =
                self.display_to_world(display_x as f64, display_y as f64);
            let mut min_sq_distance = f64::MAX;
            for (name, body) in bodies.iter() {
                let pos = body.pos();
                let sq_distance = (pos.x - click.x).powi(2)
                    + (pos.y - click.y).powi(2)
                    + (pos.z - click.z).powi(2);
                if sq_distance < min_sq_distance {
                    min_sq_distance = sq_distance;
                    self.focus_name.clone_from(name);
                }
            }
        }

        self.focus = bodies.get(&self.focus_name).unwrap().pos();
    }

    fn draw_celestial(&mut self, c: &Celestial) {
        let size = self.display.size();
        let (width, height) = (size.width as f64, size.height as f64);
        let (x, y) = self.world_to_display(&c.pos());
        let rad = c.rad() / self.control.scale;
        if x + rad < 0. || x - rad > width || y + rad < 0. || y - rad > height {
            return;
        }

        let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
        let mut r_u = (c.rad() / self.control.scale) as u32;
        if r_u == 0 {
            r_u = 1;
        }
        let r_i = r_u as i32;
        Circle::new(Point::new(x as i32 - r_i, y as i32 - r_i), r_u * 2)
            .into_styled(line_style)
            .draw(&mut self.display)
            .unwrap();
    }

    fn draw_spaceship(&mut self, s: &Spaceship) {
        let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

        let (x, y) = self.world_to_display(&s.pos());
        Rectangle::new(
            Point::new(x as i32 - 5, y as i32 - 5),
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
    use crate::gui::control::Shift;
    use crate::world::celestials::Celestials;
    use approx::assert_abs_diff_eq;

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

        gui.control.scale = 150_000.;
        gui.control.shift = Shift {
            pos: Vec3 {
                x: 315.,
                y: 1.,
                z: 0.,
            },
            mouse: None,
        };

        let (x, y) = gui.world_to_display(&pos);
        let pos2 = gui.display_to_world(x, y);
        assert!(pos.equal_to(&pos2, 0.001));

        let pos = Vec3 {
            x: 1.521_f64 * 10_f64.powi(11),
            y: 1_000_000.,
            z: 0.,
        };
        let (x, _) = gui.world_to_display(&pos);
        assert_abs_diff_eq!(x, 1014200.0021);
    }
}
