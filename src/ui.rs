use std::io::{self, stdout, Stdout};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;

pub struct Camera {
    position: (f32, f32),
    scale: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: (0.0, 0.0),
            scale: 1.0,
        }
    }
}

impl Camera {
    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn position(&self) -> (f32, f32) {
        self.position
    }

    pub fn set_position(&mut self, position: (f32, f32)) -> &mut Self {
        self.position = position;
        self
    }

    pub fn set_scale(&mut self, scale: f32) -> &mut Self {
        assert!(scale > 0.0);
        self.scale = scale;
        self
    }
}

pub struct UI {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    camera: Camera,
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame<CrosstermBackend<Stdout>>, camera: &Camera);
}

impl UI {
    pub fn new() -> io::Result<Self> {
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(Self {
            camera: Camera::default(),
            terminal: terminal,
        })
    }

    pub fn init(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        Ok(())
    }

    pub fn kill(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn draw(&mut self, data: &impl Drawable) -> io::Result<()> {
        self.terminal.draw(|frame| data.draw(frame, &self.camera))?;
        Ok(())
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
}
