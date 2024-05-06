use embedded_graphics_simulator::sdl2::{Keycode, MouseButton};
use embedded_graphics_simulator::SimulatorEvent;
use tokio::sync::mpsc;

pub enum ControlMessage {
    Shutdown,
}

pub struct Shift {
    pub x: i32,
    pub y: i32,
    pub mouse_x: u32,
    pub mouse_y: u32,
    pub pressed: bool,
}

pub struct Control {
    sender: mpsc::Sender<ControlMessage>,
    shift: Shift,
    scale: f64,
    pressed: (i32, i32)
}

pub enum ControlFlow {
    Continue,
    Break,
}

impl Control {
    pub fn new(sender: mpsc::Sender<ControlMessage>) -> Self {
        Self {
            sender,
            shift: Shift {
                x: 0,
                y: 0,
                mouse_x: 0,
                mouse_y: 0,
                pressed: false,
            },
            scale: 100_000.,
            pressed: (0, 0),
        }
    }

    pub fn shift(&self) -> &Shift {
        &self.shift
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }

    pub fn pressed(&self) -> (i32, i32) {
        self.pressed
    }

    pub fn update(&mut self, events: impl Iterator<Item = SimulatorEvent>) -> Result<ControlFlow, String> {
        for event in events {
            match event {
                SimulatorEvent::Quit => {
                    self.sender
                        .blocking_send(ControlMessage::Shutdown)
                        .map_err(|e| e.to_string())?;
                    return Ok(ControlFlow::Break);
                }
                SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                    Keycode::Q => {
                        self.sender
                            .blocking_send(ControlMessage::Shutdown)
                            .map_err(|e| e.to_string())?;
                        return Ok(ControlFlow::Break);
                    }
                    _ => {}
                },
                SimulatorEvent::MouseButtonDown { mouse_btn, point } => {
                    if mouse_btn == MouseButton::Middle {
                        self.shift.mouse_x = point.x as u32;
                        self.shift.mouse_y = point.y as u32;
                        self.shift.pressed = true;
                    }
                    if mouse_btn == MouseButton::Left {
                        self.pressed = (point.x, point.y);
                    }
                }
                SimulatorEvent::MouseButtonUp { mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Middle {
                        self.shift.pressed = false;
                    }
                }
                SimulatorEvent::MouseMove { point } => {
                    if self.shift.pressed {
                        self.shift.x += point.x - self.shift.mouse_x as i32;
                        self.shift.y += point.y - self.shift.mouse_y as i32;
                        self.shift.mouse_x = point.x as u32;
                        self.shift.mouse_y = point.y as u32;
                    }
                }
                SimulatorEvent::MouseWheel { scroll_delta, .. } => {
                    if scroll_delta.y == 1 {
                        self.scale *= 1.1;
                    } else if scroll_delta.y == -1 {
                        self.scale /= 1.1;
                    }
                }
                _ => (),
            }
        }

        Ok(ControlFlow::Continue)
    }
}
