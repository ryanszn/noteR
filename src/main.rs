mod app;
mod input;
mod terminal;
mod ui;

use anyhow::Result;
use app::App;

fn main() -> Result<()> {
    let mut terminal = terminal::init()?;
    let mut app = App::new();

    let result = app.run(&mut terminal);

    terminal::restore(&mut terminal);

    result
}
