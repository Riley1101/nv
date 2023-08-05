use crate::projects::{read_projects, Project};
use crate::shell::execute_command;
use crossterm::event::{self, Event, KeyCode};
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
    log: String,
}

impl UIApp {
    fn filter_projects(&mut self) {
        self.projects = read_projects()
            .into_iter()
            .filter(|p| p.title.contains(&self.input))
            .collect();
    }

    fn get_input_project(&mut self) -> Option<&Project> {
        let mut iter = self.projects.iter();
        let p = iter.find(|&x| x.title.contains(&self.input));
        match p {
            Some(p) => {
                self.log = p.path.clone();
            }
            None => {
                self.log = String::from("No project found");
            }
        }
        p
    }
}

impl Default for UIApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Editing,
            projects: read_projects(),
            log: String::new(),
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
                    KeyCode::Enter => {
                        let to_go = app.get_input_project();
                        match to_go {
                            Some(p) => {
                                //let _ = execute_command(&p.path);
                                app.log = p.title.clone();
                            }
                            None => {
                                app.log = "No project found".to_string();
                            }
                        }
                    }
                    KeyCode::Esc => {
                        return Ok(());
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Up => {
                        app.input_mode = InputMode::Normal;
                        app.get_input_project();
                    }
                    KeyCode::Enter => {
                        let p = app.projects.first();
                        match p {
                            Some(p) => {
                                match execute_command(&p.path) {
                                    Ok(_) => {
                                        // quit the run_app
                                        return Ok(());
                                    }
                                    Err(_) => {
                                        app.log = "No project found".to_string();
                                    }
                                }
                            }
                            None => {
                                app.log = "No project found".to_string();
                            }
                        }
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                        app.filter_projects();
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                        app.filter_projects();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
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
                Constraint::Min(4),
                Constraint::Length(4),
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

    // render log string in list
    let log =
        Paragraph::new(app.log.as_ref()).block(Block::default().borders(Borders::ALL).title("LOG"));

    f.render_widget(log, chunks[3]);

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
