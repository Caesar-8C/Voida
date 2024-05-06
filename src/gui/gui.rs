use crate::gui::control::ControlMessage;
use crate::gui::control::{Control, ControlFlow};
use crate::world::celestials::Celestial;
use crate::world::spaceship::Spaceship;
use crate::world::{Body, World};
use embedded_graphics::geometry::OriginDimensions;
use embedded_graphics::mono_font::ascii::FONT_6X9;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::prelude::{DrawTarget, Point, Primitive, Size};
use embedded_graphics::primitives::{Circle, PrimitiveStyle, Rectangle};
use embedded_graphics::text::Text;
use embedded_graphics::{pixelcolor::BinaryColor, Drawable};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, watch};

pub struct Gui {
    fps: f64,
    control: Control,
    focus: (f64, f64),
}

impl Gui {
    pub fn new(fps: f64, control_sender: mpsc::Sender<ControlMessage>) -> Self {
        Self {
            fps,
            control: Control::new(control_sender),
            focus: (0., 0.),
        }
    }

    pub fn run(mut self, world: watch::Receiver<World>) -> Result<(), String> {
        let mut display =
            SimulatorDisplay::<BinaryColor>::new(Size::new(400, 200));

        let output_settings = OutputSettingsBuilder::new()
            .theme(BinaryColorTheme::OledBlue)
            .build();
        let mut window = Window::new("Voida", &output_settings);
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

            display.clear(BinaryColor::Off).unwrap();

            let world = world.borrow().clone();
            let map = world.get();

            let focus_pos = map.get("Earth").unwrap().pos();
            self.focus = (focus_pos.x, focus_pos.y);

            for body in map.values() {
                match body {
                    Body::Celestial(c) => {
                        self.draw_celestial(&mut display, c);
                    }
                    Body::Spaceship(s) => {
                        self.draw_spaceship(&mut display, s);
                    }
                }
            }

            let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);
            Text::new(
                &format!("true sim fps: {}", world.true_sim_fps),
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
            Text::new(
                &format!(
                    "clicked coords: {} {}",
                    self.control.lmb_coords().0,
                    self.control.lmb_coords().1
                ),
                Point::new(2, 20),
                text_style,
            )
            .draw(&mut display)
            .unwrap();

            window.update(&display);
        }
    }

    fn draw_celestial(
        &self,
        display: &mut SimulatorDisplay<BinaryColor>,
        c: &Celestial,
    ) {
        let size = display.size();
        let (width, height) = (size.width as f64, size.height as f64);
        let x = (c.pos().x - self.focus.0) / self.control.scale();
        let y = (c.pos().y - self.focus.1) / self.control.scale();
        let rad = c.rad() / self.control.scale();
        if x < -width / 2. && x + rad < -width / 2.
            || x > width / 2. && x - rad > width / 2.
            || y < -height / 2. && y + rad < -height / 2.
            || y > height / 2. && y - rad > height / 2.
        {
            return;
        }

        let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
        let mut r_u = (c.rad() / self.control.scale()) as u32;
        if r_u == 0 {
            r_u = 1;
        }
        let r_i = r_u as i32;
        Circle::new(
            Point::new(
                ((c.pos().x - self.focus.0) / self.control.scale()) as i32
                    + 200
                    - r_i
                    + self.control.shift().x,
                ((self.focus.1 - c.pos().y) / self.control.scale()) as i32
                    + 100
                    - r_i
                    + self.control.shift().y,
            ),
            r_u * 2,
        )
        .into_styled(line_style)
        .draw(display)
        .unwrap();
    }

    fn draw_spaceship(
        &self,
        display: &mut SimulatorDisplay<BinaryColor>,
        s: &Spaceship,
    ) {
        let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

        Rectangle::new(
            Point::new(
                ((s.pos().x - self.focus.0) / self.control.scale() + 195.)
                    as i32
                    + self.control.shift().x,
                ((self.focus.1 - s.pos().y) / self.control.scale() + 95.)
                    as i32
                    + self.control.shift().y,
            ),
            Size::new(10, 10),
        )
        .into_styled(line_style)
        .draw(display)
        .unwrap();
    }
}
