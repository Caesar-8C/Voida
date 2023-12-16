use super::{Canvas, Window};

pub struct TextWindow {
    pub window: Canvas,
    pub data: String,
    pub update_pending: bool,
}

impl Window for TextWindow {
    fn render(&mut self, force: bool) -> Option<Vec<Vec<char>>> {
        if !self.update_pending && !force {
            return None;
        }

        let mut render =
            vec![vec![' '; self.window.width]; self.window.height];

        let mut k = 0;
        for row in render.iter_mut() {
            for item in row.iter_mut() {
                if let Some(char) = self.data.chars().nth(k) {
                    k += 1;
                    if char == '\n' {
                        break;
                    } else {
                        *item = char;
                    }
                }
            }
        }

        Some(render)
    }

    fn position(&self) -> (usize, usize) {
        (self.window.x, self.window.y)
    }
}
