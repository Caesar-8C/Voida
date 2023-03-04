#[derive(Clone)]
pub struct Window {
    pub width: usize,
    pub height: usize,
    pub x: usize,
    pub y: usize,
    pub scale: f64,
    pub focus: String,
}

pub fn sun_standard() -> Window {
    Window {
        width: 80,
        height: 53,
        x: 0,
        y: 0,
        scale: 10_f64 / 10_f64.powi(11),
        focus: "Sun".to_string(),
    }
}

pub fn earth_standard() -> Window {
    Window {
        width: 80,
        height: 40,
        x: 81,
        y: 0,
        scale: 10_f64 / 3_f64 / 10_f64.powi(8),
        focus: "Earth".to_string(),
    }
}

pub fn moon_from_side() -> Window {
    Window {
        width: 80,
        height: 12,
        x: 81,
        y: 41,
        scale: 10_f64 / 3_f64 / 10_f64.powi(8),
        focus: "Moon".to_string(),
    }
}
