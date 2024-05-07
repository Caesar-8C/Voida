use embedded_graphics_simulator::sdl2::{Keycode, MouseButton};
use embedded_graphics_simulator::SimulatorEvent;
use tokio::sync::mpsc;

pub enum ControlMessage {
    Shutdown,
    Speedup,
}

pub struct Shift {
    pub x: i32,
    pub y: i32,
    pub mouse_x: u32,
    pub mouse_y: u32,
    pub pressed: bool,
}

pub enum ControlFlow {
    Continue,
    Break,
}

pub struct Control {
    sender: mpsc::Sender<ControlMessage>,
    shift: Shift,
    scale: f64,
    rmb_coords: (i32, i32),
    lmb_coords: (i32, i32),
    change_focus: Option<(i32, i32)>,
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
            rmb_coords: (0, 0),
            lmb_coords: (0, 0),
            change_focus: None,
        }
    }

    pub fn shift(&self) -> &Shift {
        &self.shift
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }

    pub fn rmb_coords(&self) -> (i32, i32) {
        self.rmb_coords
    }

    pub fn lmb_coords(&self) -> (i32, i32) {
        self.lmb_coords
    }

    pub fn change_focus(&self) -> Option<(i32, i32)> {
        self.change_focus
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
                            self.shift.mouse_x = point.x as u32;
                            self.shift.mouse_y = point.y as u32;
                            self.shift.pressed = true;
                        }
                        MouseButton::Right => {
                            self.rmb_coords = (point.x, point.y);
                        }
                        MouseButton::Left => {
                            self.lmb_coords = (point.x, point.y);
                            self.change_focus = Some((point.x, point.y));
                        }
                        _ => ()
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
