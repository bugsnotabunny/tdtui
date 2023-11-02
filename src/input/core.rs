use std::{error::Error, io, time::Duration};

use crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent,
    MouseEventKind,
};
use ratatui::prelude::Rect;

use crate::{model::point::Point, ui::core::Camera};

#[derive(Debug, Clone, PartialEq)]
pub struct ScreenInfo {
    camera: Camera,
    frame_w: u16,
    frame_h: u16,
}

impl ScreenInfo {
    pub fn new(camera: Camera, frame_w: u16, frame_h: u16) -> Self {
        Self {
            camera: camera,
            frame_w: frame_w,
            frame_h: frame_h,
        }
    }

    pub fn from_frame_size(camera: Camera, frame_size: Rect) -> Self {
        Self::new(camera, frame_size.width, frame_size.height)
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn size(&self) -> (u16, u16) {
        (self.frame_w, self.frame_h)
    }

    pub fn len_x(&self) -> f64 {
        let bounds = self.camera.x_bounds(self.frame_w);
        bounds[1] - bounds[0]
    }

    pub fn len_y(&self) -> f64 {
        let bounds = self.camera.y_bounds(self.frame_h);
        bounds[1] - bounds[0]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MouseInput {
    col: u16,
    row: u16,
    info: ScreenInfo,
}

impl MouseInput {
    pub fn new(event: MouseEvent, info: ScreenInfo) -> Self {
        Self {
            col: event.column,
            row: event.row,
            info: info,
        }
    }

    pub fn to_world_point(&self) -> Point {
        let percent_x = self.col as f32 / self.info.frame_w as f32;
        let len_x = self.info.len_x() as f32;
        let x = self.info.camera().position().x + percent_x * len_x;

        let percent_y = 1.0 - self.row as f32 / self.info.frame_h as f32;
        let len_y = self.info.len_y() as f32;
        let y = self.info.camera().position().y + percent_y * len_y;
        Point { x: x, y: y }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum InputEvent {
    GameQuit,
    GamePauseSwitch,
    CameraRight,
    CameraLeft,
    CameraUp,
    CameraDown,
    CameraScaleUp,
    CameraScaleDown,
    CameraDrag(Point),
    MousePressedL(MouseInput),
    MousePressedR(MouseInput),
    TowerSelectorNext,
    None,
    Unknown,
}

pub fn poll_event(timeout: Duration, info: ScreenInfo) -> io::Result<InputEvent> {
    if !event::poll(timeout)? {
        return Ok(InputEvent::None);
    }
    let event = event::read()?;
    match event {
        Event::Key(key) => {
            if key.kind == KeyEventKind::Release {
                return Ok(InputEvent::None);
            }
            Ok(match_key(key))
        }
        Event::Mouse(mouse) => Ok(match_mouse_kind(mouse, info)),
        _ => Ok(InputEvent::Unknown),
    }
}

fn match_key(key: KeyEvent) -> InputEvent {
    match key {
        KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::CONTROL,
            kind: _,
            state: _,
        } => InputEvent::GameQuit,

        KeyEvent {
            code: KeyCode::Char('p'),
            modifiers: _,
            kind: _,
            state: _,
        } => InputEvent::GamePauseSwitch,

        KeyEvent {
            code: KeyCode::Char('d'),
            modifiers: _,
            kind: _,
            state: _,
        } => InputEvent::CameraRight,

        KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: _,
            kind: _,
            state: _,
        } => InputEvent::CameraLeft,

        KeyEvent {
            code: KeyCode::Char('w'),
            modifiers: _,
            kind: _,
            state: _,
        } => InputEvent::CameraUp,

        KeyEvent {
            code: KeyCode::Char('s'),
            modifiers: _,
            kind: _,
            state: _,
        } => InputEvent::CameraDown,

        KeyEvent {
            code: KeyCode::Char('z'),
            modifiers: _,
            kind: _,
            state: _,
        } => InputEvent::CameraScaleUp,

        KeyEvent {
            code: KeyCode::Char('x'),
            modifiers: _,
            kind: _,
            state: _,
        } => InputEvent::CameraScaleDown,

        KeyEvent {
            code: KeyCode::Tab,
            modifiers: _,
            kind: _,
            state: _,
        } => InputEvent::TowerSelectorNext,

        _ => InputEvent::Unknown,
    }
}

fn match_mouse_kind(event: MouseEvent, info: ScreenInfo) -> InputEvent {
    use MouseEventKind::*;
    match event.kind {
        ScrollDown => InputEvent::CameraScaleDown,
        ScrollUp => InputEvent::CameraScaleUp,
        Down(button) => match button {
            MouseButton::Left => InputEvent::MousePressedL(MouseInput::new(event, info)),
            MouseButton::Right => InputEvent::MousePressedR(MouseInput::new(event, info)),
            _ => InputEvent::Unknown,
        },
        _ => InputEvent::Unknown,
    }
}

pub trait HandleEvent {
    fn handle(&mut self, event: InputEvent) -> Result<(), Box<dyn Error>>;
}
