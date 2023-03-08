pub struct Logo {
    name: String,
    width: usize,
    height: usize,
    x: usize,
    y: usize,
}

impl Logo {
    pub fn voida() -> Self {
        Self {
            name:
"                                                              dddddddd                  |
VVVVVVVV           VVVVVVVV                 iiii              d::::::d                  |
V::::::V           V::::::V                i::::i             d::::::d                  |
V::::::V           V::::::V                 iiii              d::::::d                  |
V::::::V           V::::::V                                   d:::::d                   |
 V:::::V           V:::::V  ooooooooooo   iiiiiii     ddddddddd:::::d   aaaaaaaaaaaaa   |
  V:::::V         V:::::V oo:::::::::::oo i:::::i   dd::::::::::::::d   a::::::::::::a  |
   V:::::V       V:::::V o:::::::::::::::o i::::i  d::::::::::::::::d   aaaaaaaaa:::::a |
    V:::::V     V:::::V  o:::::ooooo:::::o i::::i d:::::::ddddd:::::d            a::::a |
     V:::::V   V:::::V   o::::o     o::::o i::::i d::::::d    d:::::d     aaaaaaa:::::a |
      V:::::V V:::::V    o::::o     o::::o i::::i d:::::d     d:::::d   aa::::::::::::a |
       V:::::V:::::V     o::::o     o::::o i::::i d:::::d     d:::::d  a::::aaaa::::::a |
        V:::::::::V      o::::o     o::::o i::::i d:::::d     d:::::d a::::a    a:::::a |
         V:::::::V       o:::::ooooo:::::o i::::i d::::::ddddd::::::d a::::a    a:::::a |
          V:::::V        o:::::::::::::::o i::::i  d::::::::::::::::d a:::::aaaa::::::a |
           V:::V          oo:::::::::::oo i::::::i  d:::::::::ddd::::d a::::::::::aa:::a|
            VVV             ooooooooooo   iiiiiiii   ddddddddd   ddddd  aaaaaaaaaa  aaaa|".to_string(),
            width: 88,
            height: 17,
            x: 0,
            y: 0,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn frame_center(&mut self, frame_width: usize, frame_height: usize) {
        self.x = (frame_width as f64 / 2. - self.width as f64 / 2.) as usize;
        self.y = (frame_height as f64 / 2. - self.height as f64 / 2.) as usize;
    }

    pub fn get_params(&self) -> (usize, usize, usize, usize) {
        (self.x, self.y, self.width, self.height)
    }

    pub fn inside(&self, x: f64, y: f64) -> bool {
        x >= self.x as f64
            && x < (self.x + self.width) as f64
            && y >= self.y as f64
            && y < (self.y + self.height) as f64
    }
}
