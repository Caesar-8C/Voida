#[derive(Clone)]
pub struct Window {
    pub width: usize,
    pub height: usize,
    pub x: usize,
    pub y: usize,
    pub scale: f64,
    pub focus: String,
}
