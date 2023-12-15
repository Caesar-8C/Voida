use super::{Rectangle, Window};
use crate::world::Body;
use crate::World;
use textplots::{Chart, Plot, Shape};
use tokio::sync::watch::Receiver;

pub struct PlotWindow {
    pub window: Rectangle,
    pub world: Receiver<World>,
    pub data: Vec<f64>,
    pub cursor: usize,
}

impl PlotWindow {
    fn update(&mut self) {
        let world = self.world.borrow().get();
        let earth = world[&"Earth".to_string()].pos();
        let iss = world[&"ISS".to_string()].pos();
        if let Body::Celestial(e) = &world[&"Earth".to_string()] {
            let dist = ((earth.x - iss.x) * (earth.x - iss.x)
                + (earth.y - iss.y) * (earth.y - iss.y)
                + (earth.z - iss.z) * (earth.z - iss.z))
                .sqrt()
                - e.rad();
            self.cursor %= self.data.len();
            self.data[self.cursor] = dist;
            self.cursor += 1;
        }
    }

    fn get_data(&self, x: f32) -> f32 {
        let mut x_usize = 0;
        if x > 0. {
            x_usize = x as usize;
        }
        if x_usize >= self.data.len() {
            x_usize = self.data.len() - 1;
        }

        let index = (x_usize + self.cursor) % self.data.len();

        self.data[index] as f32
    }
}

impl Window for PlotWindow {
    fn render(&mut self) -> Vec<Vec<char>> {
        let mut render =
            vec![vec![' '; self.window.width]; self.window.height];

        self.update();

        let mut chart = Chart::new(130, 100, 0., 200.);
        let shape = Shape::Continuous(Box::new(|x| self.get_data(x)));
        let lineplot = chart.lineplot(&shape);
        lineplot.axis();
        lineplot.figures();
        let plot_string = format!("{lineplot}");

        let mut k = 0;
        for row in render.iter_mut() {
            for item in row.iter_mut() {
                if let Some(char) = plot_string.chars().nth(k) {
                    k += 1;
                    if char == '\n' {
                        break;
                    } else {
                        *item = char;
                    }
                }
            }
        }

        render
    }

    fn position(&self) -> (usize, usize) {
        (self.window.x, self.window.y)
    }
}
