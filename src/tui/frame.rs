pub struct Frame {
    pub width: usize,
    pub height: usize,
    pub vec: Vec<Vec<String>>,
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

    pub fn inside(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        true
    }

    pub fn fill(&mut self, val: String) {
        self.vec = vec![vec![val; self.width]; self.height];
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
