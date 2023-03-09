use std::io::stdin;
use termion::event::Key;
use termion::input::TermRead;
use tokio::sync::watch::Sender;

pub struct Control {
    sender: Sender<bool>,
}

impl Control {
    pub fn new(sender: Sender<bool>) -> Self {
        Self { sender }
    }

    pub async fn run(self) -> Result<(), String> {
        let stdin = stdin();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Up => {
                    self.sender.send(true).map_err(|e| format!("{}", e))?;
                }
                Key::Down => {
                    self.sender.send(false).map_err(|e| format!("{}", e))?;
                }
                Key::Ctrl('q') => {
                    break;
                }
                _ => (),
            }
        }
        Ok(())
    }
}
