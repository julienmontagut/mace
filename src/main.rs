use std::{env::current_dir, path::PathBuf, io::{self, stdout}};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::Paragraph,
    Terminal,
    layout::{Layout, Constraint, Direction},
    style::{Style, Color},
    text::{Line, Span},
};

use clap::{arg, value_parser, Command};

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(PartialEq)]
enum Mode {
    Normal,
    Insert,
}

struct App {
    content: String,
    mode: Mode,
    working_dir: PathBuf,
}

impl App {
    fn new(working_dir: Option<PathBuf>) -> Self {
        Self {
            content: String::new(),
            mode: Mode::Normal,
            working_dir: working_dir.unwrap_or_else(|| current_dir().ok().unwrap()),
        }
    }

    fn handle_normal_mode(&mut self, key: event::KeyEvent) -> bool {
        match (key.code, key.modifiers) {
            (KeyCode::Char('i'), KeyModifiers::NONE) => {
                self.mode = Mode::Insert;
                false
            }
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => true,
            _ => false,
        }
    }

    fn handle_insert_mode(&mut self, key: event::KeyEvent) -> bool {
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

fn main() -> io::Result<()> {
    let matches = Command::new(APP_NAME)
        .version(APP_VERSION)
        .author("Julien Montagut <_@julienmontagut.com>")
        .about("Made is a modern accessible development environment")
        .args(&[
            // arg!(-c --config <PATH> "Path to the config directory"),
            arg!([PATH] "A file or directory to open")
                .id("path")
                .value_parser(value_parser!(PathBuf)),
        ])
        .get_matches();

    // Create app state
    let mut app = App::new(matches.get_one::<PathBuf>("path").cloned());

    if let Some(path) = matches.get_one::<PathBuf>("path") {
        if path.is_dir() {
            todo!();
        } else {
            if let Ok(content) = std::fs::read_to_string(path) {
                app.content = content;
            }
        }
    }

    // Setup terminal
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    // Main event loop
    loop {
        terminal.draw(|frame| {
            let size = frame.area();
            
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(1),
                    Constraint::Length(1),
                ].as_ref())
                .split(size);

            let mode_style = if app.mode == Mode::Insert {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Blue)
            };

            let mode_text = if app.mode == Mode::Insert { "INSERT" } else { "NORMAL" };

            // Main editor without borders
            let editor = Paragraph::new(app.content.as_str());
            frame.render_widget(editor, chunks[0]);

            // Status bar
            let status = Line::from(vec![
                Span::raw(app.working_dir.display().to_string()),
                Span::raw(" - "),
                Span::styled(mode_text, mode_style),
            ]);
            let status_bar = Paragraph::new(status)
                .style(Style::default().bg(Color::DarkGray));
            frame.render_widget(status_bar, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            let should_quit = match app.mode {
                Mode::Normal => app.handle_normal_mode(key),
                Mode::Insert => app.handle_insert_mode(key),
            };

            if should_quit {
                break;
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    
    Ok(())
}
