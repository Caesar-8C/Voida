pub trait Window: Send {
    fn render(&mut self) -> Vec<Vec<char>>;
    fn position(&self) -> (usize, usize);
}

#[derive(Clone)]
pub struct Rectangle {
    pub width: usize,
    pub height: usize,
    pub x: usize,
    pub y: usize,
}

impl Rectangle {
    pub fn inside(&self, x_f64: f64, y_f64: f64) -> bool {
        if x_f64 < 0. || y_f64 < 0. {
            return false;
        }

        let x = x_f64 as usize;
        let y = y_f64 as usize;

        x < self.width && y < self.height
    }
}
