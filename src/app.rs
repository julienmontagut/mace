use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::env::current_dir;
use std::path::PathBuf;

#[derive(PartialEq)]
pub enum Mode {
    Normal,
    Insert,
}

pub struct App {
    pub content: String,
    pub mode: Mode,
    pub working_dir: PathBuf,
}

impl App {
    pub fn new(working_dir: Option<PathBuf>) -> Self {
        Self {
            content: String::new(),
            mode: Mode::Normal,
            working_dir: working_dir.unwrap_or_else(|| current_dir().ok().unwrap()),
        }
    }

    pub fn handle_normal_mode(&mut self, key: KeyEvent) -> bool {
        match (key.code, key.modifiers) {
            (KeyCode::Char('i'), KeyModifiers::NONE) => {
                self.mode = Mode::Insert;
                false
            }
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => true,
            _ => false,
        }
    }

    pub fn handle_insert_mode(&mut self, key: KeyEvent) -> bool {
        match (key.code, key.modifiers) {
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                self.mode = Mode::Normal;
                false
            }
            (KeyCode::Char(c), KeyModifiers::NONE) => {
                self.content.push(c);
                false
            }
            (KeyCode::Backspace, KeyModifiers::NONE) => {
                self.content.pop();
                false
            }
            (KeyCode::Enter, KeyModifiers::NONE) => {
                self.content.push('\n');
                false
            }
            _ => false,
        }
    }
}
