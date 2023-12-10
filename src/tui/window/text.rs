use super::{Rectangle, Window};

pub struct TextWindow {
    pub window: Rectangle,
    pub data: String,
}

impl Window for TextWindow {
    fn render(&mut self) -> Vec<Vec<String>> {
        let mut render =
            vec![vec![" ".to_string(); self.window.width]; self.window.height];

        let mut k = 0;
        for row in render.iter_mut() {
            for item in row.iter_mut() {
                if let Some(char) = self.data.chars().nth(k) {
                    k += 1;
                    if char == '\n' {
                        break;
                    } else {
                        *item = char.to_string();
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
