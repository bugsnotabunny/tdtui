use std::{error::Error, io, time::Duration};

use crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent,
    MouseEventKind,
};
use ratatui::prelude::Rect;

use crate::{
    model::point::{Point, Positioned},
    ui::core::Camera,
};

use super::{tower_gap::TowerGap, tower_radius::TowerRadius, tower_selector::TowerSelector};

#[derive(Debug, Clone, Copy)]
pub struct InputContext {
    tower_selector: TowerSelector,
    tower_radius: TowerRadius,
    tower_gap: TowerGap,
    screen_info: ScreenInfo,
}

impl InputContext {
    pub fn new(min_tower_gap: f32) -> Self {
        let selector = TowerSelector::default();
        Self {
            screen_info: ScreenInfo {
                camera: Camera::new(Point::default(), 0.0, 0.0),
                frame_h: 0,
                frame_w: 0,
            },
            tower_selector: selector,
            tower_radius: TowerRadius::new(Point { x: 0.0, y: 0.0 }, selector.current().range),
            tower_gap: TowerGap::new(Point { x: 0.0, y: 0.0 }, min_tower_gap),
        }
    }

    pub fn tower_selector(&self) -> TowerSelector {
        self.tower_selector
    }

    pub fn set_tower_selector(&mut self, tower_selector: TowerSelector) -> &mut Self {
        self.tower_selector = tower_selector;
        self
    }

    pub fn screen_info(&self) -> ScreenInfo {
        self.screen_info
    }

    pub fn set_screen_info(&mut self, screen_info: ScreenInfo) -> &mut Self {
        self.screen_info = screen_info;
        self
    }

    pub fn tower_radius(&self) -> TowerRadius {
        self.tower_radius
    }

    pub fn tower_gap(&self) -> TowerGap {
        self.tower_gap
    }

    pub fn handle(&mut self, event: InputEvent) -> Result<(), Box<dyn Error>> {
        let mut selector = self.tower_selector;
        selector.handle(event, self)?;
        self.tower_selector = selector;

        let mut radius = self.tower_radius;
        radius.handle(event, self)?;
        self.tower_radius = radius;

        let mut gap = self.tower_gap;
        gap.handle(event, self)?;
        self.tower_gap = gap;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MousePos {
    col: u16,
    row: u16,
}

impl MousePos {
    pub fn new(event: MouseEvent) -> Self {
        Self {
            col: event.column,
            row: event.row,
        }
    }

    pub fn to_world_point(&self, info: ScreenInfo) -> Point {
        let percent_x = self.col as f32 / info.frame_w as f32;
        let len_x = info.len_x() as f32;
        let x = info.camera().position().x + percent_x * len_x;

        let percent_y = 1.0 - self.row as f32 / info.frame_h as f32;
        let len_y = info.len_y() as f32;
        let y = info.camera().position().y + percent_y * len_y;
        Point { x: x, y: y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InputEvent {
    GameQuit,
    GamePauseSwitch,
    CameraRight,
    CameraLeft,
    CameraUp,
    CameraDown,
    CameraScaleUp,
    CameraScaleDown,
    MouseMovedTo(MousePos),
    MousePressedL(MousePos),
    MousePressedR(MousePos),
    TowerSelectorNext,
    None,
    Unknown,
}

pub fn poll_event(timeout: Duration) -> io::Result<InputEvent> {
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
        Event::Mouse(mouse) => Ok(match_mouse_kind(mouse)),
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

fn match_mouse_kind(event: MouseEvent) -> InputEvent {
    use MouseEventKind::*;
    match event.kind {
        ScrollDown => InputEvent::CameraScaleDown,
        ScrollUp => InputEvent::CameraScaleUp,
        Moved => InputEvent::MouseMovedTo(MousePos::new(event)),
        Down(button) => match button {
            MouseButton::Left => InputEvent::MousePressedL(MousePos::new(event)),
            MouseButton::Right => InputEvent::MousePressedR(MousePos::new(event)),
            _ => InputEvent::Unknown,
        },
        _ => InputEvent::Unknown,
    }
}

pub trait HandleEvent {
    fn handle(
        &mut self,
        event: InputEvent,
        input_context: &InputContext,
    ) -> Result<(), Box<dyn Error>>;
}
