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

    pub fn fill(&mut self, val: String) {
        self.vec = vec![vec![val; self.width]; self.height];
    }
}