use std::io::{self, stdout, Stdout};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{Constraint, CrosstermBackend, Direction, Layout},
    Frame, Terminal,
};

use crate::model::core::GameModel;

use super::{road::RoadDrawable, tower::TowerDrawable};

pub struct Camera {
    position: (f32, f32),
    scale: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: (0.0, -10.0),
            scale: 1.0,
        }
    }
}

impl Camera {
    pub fn main_layout(&self) -> Layout {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100), Constraint::Percentage(100)])
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn x_bounds(&self, frame_w: u16) -> [f64; 2] {
        [
            self.position().0 as f64,
            (self.position().0 + frame_w as f32 * self.scale()) as f64,
        ]
    }

    pub fn y_bounds(&self, frame_h: u16) -> [f64; 2] {
        [
            self.position().1 as f64,
            (self.position().1 + frame_h as f32 * self.scale()) as f64,
        ]
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

pub struct Screen {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame, camera: &Camera, game_model: &impl GameModel);
}

impl Screen {
    pub fn new() -> io::Result<Self> {
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(Self { terminal: terminal })
    }

    pub fn init(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        stdout().execute(EnableMouseCapture)?;
        Ok(())
    }

    pub fn kill(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        stdout().execute(DisableMouseCapture)?;
        Ok(())
    }

    pub fn draw_frame(&mut self, camera: &Camera, game_model: &impl GameModel) -> io::Result<()> {
        self.terminal
            .draw(|frame| Self::draw_impl(frame, camera, game_model))?;
        
        Ok(())
    }

    fn draw_impl(frame: &mut Frame, camera: &Camera, game_model: &impl GameModel) {
        let road_drawable = RoadDrawable::new(game_model.road());
        road_drawable.draw(frame, camera, game_model);
        for enemy in game_model.road().enemies().iter() {
            enemy.borrow().draw(frame, camera, game_model);
        }
        for tower in game_model.towers() {
            let drawable = TowerDrawable {
                tower: tower.as_ref(),
            };
            drawable.draw(frame, camera, game_model);
        }
    }
}
