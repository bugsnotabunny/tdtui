use std::{io, time::Duration};

use bitmask_enum::bitmask;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

#[bitmask]
pub enum InputMask {
    Quitted = 0b00000001,
    RightPressed = 0b00000010,
    LeftPressed = 0b00000100,
    UpPressed = 0b00001000,
    DownPressed = 0b00010000,
    ScaleUpPressed = 0b00100000,
    ScaleDownPressed = 0b01000000,
}

impl InputMask {
    pub fn set(&mut self, bits: InputMask) -> &mut Self {
        *self |= bits;
        self
    }

    pub fn unset(&mut self, bits: InputMask) -> &mut Self {
        *self &= bits.not();
        self
    }
}

pub fn poll_events(timeout: Duration) -> io::Result<InputMask> {
    let mut result = InputMask::none();
    if !event::poll(timeout)? {
        return Ok(result);
    }

    while event::poll(Duration::from_millis(0))? {
        if let Event::Key(key) = event::read()? {
            match key {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: _,
                    state: _,
                } => maybe_set(&mut result, key, InputMask::Quitted),

                KeyEvent {
                    code: KeyCode::Char('d'),
                    modifiers: _,
                    kind: _,
                    state: _,
                } => maybe_set(&mut result, key, InputMask::RightPressed),

                KeyEvent {
                    code: KeyCode::Char('a'),
                    modifiers: _,
                    kind: _,
                    state: _,
                } => maybe_set(&mut result, key, InputMask::LeftPressed),

                KeyEvent {
                    code: KeyCode::Char(']'),
                    modifiers: _,
                    kind: _,
                    state: _,
                } => maybe_set(&mut result, key, InputMask::UpPressed),
                KeyEvent {
                    code: KeyCode::Char('['),
                    modifiers: _,
                    kind: _,
                    state: _,
                } => maybe_set(&mut result, key, InputMask::DownPressed),
                KeyEvent {
                    code: KeyCode::Char('w'),
                    modifiers: _,
                    kind: _,
                    state: _,
                } => maybe_set(&mut result, key, InputMask::ScaleUpPressed),
                KeyEvent {
                    code: KeyCode::Char('s'),
                    modifiers: _,
                    kind: _,
                    state: _,
                } => maybe_set(&mut result, key, InputMask::ScaleDownPressed),
                _ => {}
            }
        }
    }
    Ok(result)
}

fn maybe_set(inputs: &mut InputMask, key: KeyEvent, to_set: InputMask) {
    match key.kind {
        KeyEventKind::Press => {
            inputs.set(to_set);
        }
        KeyEventKind::Release => {
            inputs.unset(to_set);
        }
        KeyEventKind::Repeat => {}
    }
}
