use termion::terminal_size;

pub struct Frame {
    pub width: usize,
    pub height: usize,
    vec: Vec<Vec<char>>,
}

impl Frame {
    pub fn new(background: char) -> Result<Self, String> {
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

    pub fn try_set(&mut self, x_f64: f64, y_f64: f64, value: char) -> bool {
        if self.inside(x_f64, y_f64) {
            let x = x_f64 as usize;
            let y = y_f64 as usize;
            self.try_set_usize(x, y, value)
        } else {
            false
        }
    }

    pub fn try_set_usize(&mut self, x: usize, y: usize, value: char) -> bool {
        if !self.inside_usize(x, y) {
            false
        } else {
            self.vec[y][x] = value;
            true
        }
    }

    pub fn try_set_window(&mut self, x: usize, y:usize, render: Vec<Vec<char>>) {
        for (j, row) in render.iter().enumerate() {
            for (i, item) in row.iter().enumerate() {
                self.try_set_usize(i + x, j + y, *item);
            }
        }
    }

    pub fn flush(&self) {
        let mut st = "".to_string();
        for first in &self.vec {
            for second in first {
                st.push(*second);
            }
            st += "\n\r";
        }
        print!("{}c{}", 27 as char, st);
    }
}
