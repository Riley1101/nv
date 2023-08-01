use crate::projects::{read_projects, Project};
use crossterm::event::{self, Event, KeyCode};
use std::process::Command;
use std::{io, vec};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

pub enum InputMode {
    Normal,
    Editing,
}

pub struct UIApp {
    input: String,
    input_mode: InputMode,
    projects: Vec<Project>,
}

impl Default for UIApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            projects: read_projects(),
        }
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: UIApp) -> io::Result<()> {
    loop {
        let _ = terminal.draw(|f| ui(f, &app));
        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('i') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Esc => {
                        return Ok(());
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        println!("Navigate to project");
                        let p = app.projects.first();
                        match p {
                            Some(p) => {
                                println!("Navigating to project: {}", p.path);
                                let cmd = Command::new("/usr/bin/sh")
                                    .arg("-c")
                                    .arg(format!("cd {} && nvim .", p.path))
                                    .spawn()
                                    .expect("Error: Failed to run editor")
                                    .wait()
                                    .expect("Error: Editor returned a non-zero status");
                                if cmd.success() {
                                    println!("Editor exited successfully");
                                    // quit the run_app
                                    return Ok(());
                                } else {
                                    println!("Editor exited with error");
                                }
                            }
                            None => {
                                println!("No project found");
                            }
                        }
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                        // filter projects with matching input if input delete the list should be reset
                        app.projects = read_projects()
                            .into_iter()
                            .filter(|p| p.title.contains(&app.input))
                            .collect();
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                        app.projects = read_projects();
                    }
                    _ => {}
                },
            }
        };
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &UIApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());
    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("ESC", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("i", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to navigate to project."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
    };

    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    // render input
    f.render_widget(input, chunks[1]);

    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1,
            )
        }
    }

    // render projects
    let projects: Vec<ListItem> = app
        .projects
        .iter()
        .enumerate()
        .map(|(i, p)| {
            if i == 0 {
                let content = vec![Spans::from(Span::raw(format!(
                    "[ * ] {} - {}",
                    p.title, p.path
                )))];
                ListItem::new(content)
            } else {
                let content = vec![Spans::from(Span::raw(format!(
                    "[ ] {} - {}",
                    p.title, p.path
                )))];
                ListItem::new(content)
            }
        })
        // style the first one with index
        .enumerate()
        .map(|(i, li)| {
            if i == 0 {
                // add arrow
                let mut li = li;
                li = li.style(Style::default().add_modifier(Modifier::BOLD));
                li.style(Style::default().fg(Color::Yellow))
            } else {
                li
            }
        })
        .collect();
    let projects =
        List::new(projects).block(Block::default().borders(Borders::ALL).title("Projects"));

    f.render_widget(projects, chunks[2])
}
