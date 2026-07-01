use std::io;

use anyhow::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};

use ratatui::{Terminal, backend::CrosstermBackend};

pub type Tui = Terminal<CrosstermBackend<io::Stdout>>;

pub fn init() -> Result<Tui> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}

pub fn restore(terminal: &mut Tui) -> Result<()> {
    disable_raw_mode()?;

    terminal.clear()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        Clear(ClearType::All),
        MoveTo(0, 0),
    )?;

    Ok(())
}
