use crate::utils::Vec3;
use embedded_graphics_simulator::sdl2::{Keycode, MouseButton};
use embedded_graphics_simulator::SimulatorEvent;
use nalgebra::Matrix3;
use tokio::sync::mpsc;

pub enum ControlMessage {
    Shutdown,
    Speedup,
    SetTimeSpeed(f64),
}

pub struct Shift {
    pub pos: Vec3,
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
    pub camera_extr: Matrix3<f64>,
    pub camera_extr_inv: Matrix3<f64>,
    pub rmb_coords: (i32, i32),
    pub change_focus: Option<(i32, i32)>,
}

impl Control {
    pub fn new(sender: mpsc::Sender<ControlMessage>) -> Self {
        Self {
            sender,
            shift: Shift {
                pos: Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
                mouse: None,
            },
            scale: 100_000.,
            camera_extr: Matrix3::identity(),
            camera_extr_inv: Matrix3::identity(),
            rmb_coords: (0, 0),
            change_focus: None,
        }
    }

    fn send(&self, message: ControlMessage) -> Result<(), String> {
        self.sender
            .blocking_send(message)
            .map_err(|e| e.to_string())
    }

    pub fn update(
        &mut self,
        events: impl Iterator<Item = SimulatorEvent>,
    ) -> Result<ControlFlow, String> {
        self.change_focus = None;

        for event in events {
            match event {
                SimulatorEvent::Quit => {
                    self.send(ControlMessage::Shutdown)?;
                    return Ok(ControlFlow::Break);
                }
                SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                    Keycode::Q => {
                        self.send(ControlMessage::Shutdown)?;
                        return Ok(ControlFlow::Break);
                    }
                    Keycode::Up => {
                        self.send(ControlMessage::Speedup)?;
                    }
                    Keycode::Space => {
                        self.send(ControlMessage::SetTimeSpeed(0.))?;
                    }
                    Keycode::Num1 => {
                        self.send(ControlMessage::SetTimeSpeed(1.))?;
                    }
                    Keycode::Num2 => {
                        self.send(ControlMessage::SetTimeSpeed(200.))?;
                    }
                    Keycode::Num3 => {
                        self.send(ControlMessage::SetTimeSpeed(500.))?;
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
                        self.shift.pos += &self.camera_extr_inv
                            * &Vec3 {
                                x: (point.x - mouse_x) as f64,
                                y: (mouse_y - point.y) as f64,
                                z: 0.,
                            }
                            * self.scale;
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
