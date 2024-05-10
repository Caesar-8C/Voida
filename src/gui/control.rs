use embedded_graphics_simulator::sdl2::{Keycode, MouseButton};
use embedded_graphics_simulator::SimulatorEvent;
use tokio::sync::mpsc;

pub enum ControlMessage {
    Shutdown,
    Speedup,
}

pub struct Shift {
    pub x: f64,
    pub y: f64,
    pub mouse: Option<(i32, i32)>,
}

pub enum ControlFlow {
    Continue,
    Break,
}

pub struct Control {
    sender: mpsc::Sender<ControlMessage>,
    pub shift: Shift,
    pub scale: f64,
    pub rmb_coords: (i32, i32),
    pub lmb_coords: (i32, i32),
    pub change_focus: Option<(i32, i32)>,
}

impl Control {
    pub fn new(sender: mpsc::Sender<ControlMessage>) -> Self {
        Self {
            sender,
            shift: Shift {
                x: 0.,
                y: 0.,
                mouse: None,
            },
            scale: 100_000.,
            rmb_coords: (0, 0),
            lmb_coords: (0, 0),
            change_focus: None,
        }
    }

    pub fn update(
        &mut self,
        events: impl Iterator<Item = SimulatorEvent>,
    ) -> Result<ControlFlow, String> {
        self.change_focus = None;

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
                    Keycode::Up => {
                        self.sender
                            .blocking_send(ControlMessage::Speedup)
                            .map_err(|e| e.to_string())?;
                    }
                    _ => {}
                },
                SimulatorEvent::MouseButtonDown { mouse_btn, point } => {
                    match mouse_btn {
                        MouseButton::Middle => {
                            self.shift.mouse = Some((point.x, point.y));
                        }
                        MouseButton::Right => {
                            self.rmb_coords = (point.x, point.y);
                        }
                        MouseButton::Left => {
                            self.lmb_coords = (point.x, point.y);
                            self.change_focus = Some((point.x, point.y));
                        }
                        _ => (),
                    }
                }
                SimulatorEvent::MouseButtonUp { mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Middle {
                        self.shift.mouse = None;
                    }
                }
                SimulatorEvent::MouseMove { point } => {
                    if let Some((mouse_x, mouse_y)) = self.shift.mouse {
                        self.shift.x += (point.x - mouse_x) as f64 * self.scale;
                        self.shift.y += (mouse_y - point.y) as f64 * self.scale;
                        self.shift.mouse = Some((point.x, point.y));
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
