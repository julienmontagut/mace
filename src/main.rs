use clap::{arg, value_parser, Command};
use crossterm::{
    event::{self, Event},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Terminal,
};
use std::{
    io::{self, stdout},
    path::PathBuf,
};

mod app;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> io::Result<()> {
    let matches = Command::new(APP_NAME)
        .version(APP_VERSION)
        .author("Julien Montagut <_@julienmontagut.com>")
        .about("Mace is a modern accessible development environment")
        .args(&[
            // arg!(-c --config <PATH> "Path to the config directory"),
            arg!([PATH] "A file or directory to open")
                .id("path")
                .value_parser(value_parser!(PathBuf)),
        ])
        .get_matches();

    // Create app state
    let mut app = app::App::new(matches.get_one::<PathBuf>("path").cloned());

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
                .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
                .split(size);

            let mode_style = if app.mode == app::Mode::Insert {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Blue)
            };

            let mode_text = if app.mode == app::Mode::Insert {
                "INSERT"
            } else {
                "NORMAL"
            };

            // Main editor without borders
            let editor = Paragraph::new(app.content.as_str());
            frame.render_widget(editor, chunks[0]);

            // Status bar
            let status = Line::from(vec![
                Span::raw(app.working_dir.display().to_string()),
                Span::raw(" - "),
                Span::styled(mode_text, mode_style),
            ]);
            let status_bar = Paragraph::new(status).style(Style::default().bg(Color::DarkGray));
            frame.render_widget(status_bar, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            let should_quit = match app.mode {
                app::Mode::Normal => app.handle_normal_mode(key),
                app::Mode::Insert => app.handle_insert_mode(key),
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
