use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::env;

use nv::app::{run_app, UIApp};
use nv::projects::mark;
use std::{error::Error, io};
use tui::{backend::CrosstermBackend, Terminal};

#[derive(Debug)]
enum Mode {
    Open,
    Mark,
}

fn check_open_or_mark() -> Mode {
    let args = env::args().collect::<Vec<String>>();
    if args.len() == 3 {
        Mode::Mark
    } else {
        Mode::Open
    }
}

fn open() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = UIApp::default();

    let _ = run_app(&mut terminal, app);
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn main() {
    let _mode = check_open_or_mark();
    println!("mode: {:?}", _mode);
    match _mode {
        Mode::Open => {
            open().expect("Error running open mode");
        }
        Mode::Mark => {
            mark();
        }
    }
}
