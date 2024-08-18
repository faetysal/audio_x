use std::io::{stdout, Result, Stdout};

use ratatui::{backend::CrosstermBackend, crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}}, Terminal};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn init() -> Result<Tui>{
  execute!(stdout(), EnterAlternateScreen)?;
  enable_raw_mode()?;
  Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn restore() -> Result<()> {
  execute!(stdout(), LeaveAlternateScreen)?;
  disable_raw_mode()?;

  Ok(())
}