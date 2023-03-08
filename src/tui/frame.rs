pub struct Frame {
    pub width: usize,
    pub height: usize,
    vec: Vec<Vec<String>>,
}

impl Frame {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            vec: vec![vec![" ".to_string(); width]; height],
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }

    pub fn inside(&self, x_f64: f64, y_f64: f64) -> bool {
        if x_f64 < 0. || y_f64 < 0. {
            return false;
        }

        let x = x_f64 as usize;
        let y = y_f64 as usize;
        if x >= self.width || y >= self.height {
            return false;
        }
        true
    }

    pub fn inside_usize(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn fill(&mut self, val: String) {
        self.vec = vec![vec![val; self.width]; self.height];
    }

    pub fn try_set(&mut self, x_f64: f64, y_f64: f64, value: String) -> bool {
        if !self.inside(x_f64, y_f64) {
            false
        } else {
            let x = x_f64 as usize;
            let y = y_f64 as usize;

            if (&value != "∘" && &value != "I") || &self.vec[y][x] == " " {
                self.vec[y][x] = value;
            }

            true
        }
    }

    pub fn try_set_usize(&mut self, x: usize, y: usize, value: String) -> bool {
        if !self.inside_usize(x, y) {
            false
        } else {

            if (&value != "∘" && &value != "I") || &self.vec[y][x] == " " {
                self.vec[y][x] = value;
            }

            true
        }
    }

    pub fn flush(&self) {
        let mut st = "".to_string();
        for first in &self.vec {
            for second in first {
                st += second;
            }
            st += "\n";
        }
        print!("{}c{}", 27 as char, st);
    }
}
