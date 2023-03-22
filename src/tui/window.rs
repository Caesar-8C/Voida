use crate::Vec3;

#[derive(Clone)]
pub struct Window {
    pub width: usize,
    pub height: usize,
    pub x: usize,
    pub y: usize,
    pub scale: f64,
    pub focus: String,
    pub x_dir: Vec3,
    pub y_dir: Vec3,
}

impl Window {
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
}

pub fn sun_standard() -> Window {
    Window {
        width: 80,
        height: 26,
        x: 21,
        y: 0,
        scale: 10_f64 / 2_f64 / 10_f64.powi(11),
        focus: "Sun".to_string(),
        x_dir: Vec3 {x: 1., y: 0., z: 0.},
        y_dir: Vec3 {x: 0., y: 1., z: 0.},
    }
}

pub fn earth_standard() -> Window {
    Window {
        width: 80,
        height: 40,
        x: 102,
        y: 0,
        scale: 10_f64 / 3_f64 / 10_f64.powi(8),
        focus: "Earth".to_string(),
        x_dir: Vec3 {x: 1., y: 0., z: 0.},
        y_dir: Vec3 {x: 0., y: 1., z: 0.},
    }
}

pub fn moon_from_side() -> Window {
    Window {
        width: 80,
        height: 12,
        x: 102,
        y: 41,
        scale: 10_f64 / 3_f64 / 10_f64.powi(8),
        focus: "Earth".to_string(),
        x_dir: Vec3 {x: 1., y: 0., z: 0.},
        y_dir: Vec3 {x: 0., y: 0., z: 1.},
    }
}

pub fn iss() -> Window {
    Window {
        width: 80,
        height: 26,
        x: 21,
        y: 27,
        scale: 10_f64 / 6_f64 / 10_f64.powi(6),
        focus: "Earth".to_string(),
        x_dir: Vec3 {x: 1., y: 0., z: 0.},
        y_dir: Vec3 {x: 0., y: 1., z: 0.},
    }
}
