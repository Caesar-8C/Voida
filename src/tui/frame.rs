use termion::terminal_size;

pub struct Frame {
    pub width: usize,
    pub height: usize,
    vec: Vec<Vec<String>>,
}

impl Frame {
    pub fn new(background: String) -> Result<Self, String> {
        let (width, height) = Self::get_terminal_size()?;
        Ok(Self {
            width,
            height,
            vec: vec![vec![background; width]; height],
        })
    }

    fn get_terminal_size() -> Result<(usize, usize), String> {
        let (x, y) = terminal_size().map_err(|e| format!("{}", e))?;
        Ok((x as usize, y as usize - 1))
    }

    pub fn size_changed(&mut self) -> Result<bool, String> {
        let (x, y) = Self::get_terminal_size()?;
        Ok(x != self.width || y != self.height)
    }

    pub fn inside(&self, x_f64: f64, y_f64: f64) -> bool {
        if x_f64 < 0. || y_f64 < 0. {
            return false;
        }

        let x = x_f64 as usize;
        let y = y_f64 as usize;
        self.inside_usize(x, y)
    }

    pub fn inside_usize(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn try_set(&mut self, x_f64: f64, y_f64: f64, value: String) -> bool {
        if self.inside(x_f64, y_f64) {
            let x = x_f64 as usize;
            let y = y_f64 as usize;
            self.try_set_usize(x, y, value)
        } else {
            false
        }
    }

    pub fn try_set_usize(&mut self, x: usize, y: usize, value: String) -> bool {
        if !self.inside_usize(x, y) {
            false
        } else {
            self.vec[y][x] = value;
            true
        }
    }

    pub fn try_set_window(&mut self, window_x: usize, window_y:usize, render: Vec<Vec<String>>) {
        for y in 0..(render.len()) {
            for x in 0..(render[y].len()) {
                self.try_set_usize(x + window_x, y + window_y, render[y][x].clone());
            }
        }
    }

    pub fn flush(&self) {
        let mut st = "".to_string();
        for first in &self.vec {
            for second in first {
                st += second;
            }
            st += "\n\r";
        }
        print!("{}c{}", 27 as char, st);
    }
}
