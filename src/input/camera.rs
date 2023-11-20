use std::{error::Error, fmt::Display};

use crate::{model::Positioned, ui::Camera};

use super::core::{HandleEvent, InputContext, InputEvent};

const SCROLL: f32 = 1.0;
const SCALE_SCROLL: f32 = 0.1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CameraScaleInvarianceErr {}

impl Display for CameraScaleInvarianceErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tried to take money from wallet while not having enough")
    }
}

impl Error for CameraScaleInvarianceErr {}

impl HandleEvent for Camera {
    fn handle(&mut self, event: InputEvent, _: &InputContext) -> Result<(), Box<dyn Error>> {
        match event {
            InputEvent::CameraRight => {
                let mut pos = self.position();
                pos.x += SCROLL;
                self.set_position(pos);
                Ok(())
            }
            InputEvent::CameraLeft => {
                let mut pos = self.position();
                pos.x -= SCROLL;
                self.set_position(pos);
                Ok(())
            }
            InputEvent::CameraUp => {
                let mut pos = self.position();
                pos.y += SCROLL;
                self.set_position(pos);
                Ok(())
            }
            InputEvent::CameraDown => {
                let mut pos = self.position();
                pos.y -= SCROLL;
                self.set_position(pos);
                Ok(())
            }
            InputEvent::CameraScaleDown => {
                let scale = self.scale();
                self.set_scale(scale + SCALE_SCROLL);
                Ok(())
            }
            InputEvent::CameraScaleUp => {
                let scale = self.scale();
                if scale > SCALE_SCROLL {
                    self.set_scale(scale - SCALE_SCROLL);
                }
                Err(Box::new(CameraScaleInvarianceErr {}))
            }
            _ => Ok(()),
        }
    }
}
