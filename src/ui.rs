use std::io::{self, stdout, Stdout};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;

pub struct UI {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame<CrosstermBackend<Stdout>>);
}

impl UI {
    pub fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(Self { terminal: terminal })
    }

    pub fn kill(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn draw<GameDataT: Drawable>(&mut self, data: &GameDataT) {
        let _ = self.terminal.draw(|frame| data.draw(frame));
    }
}
