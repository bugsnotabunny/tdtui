use crate::ui::core::Camera;

use super::core::{HandleEvent, InputEvent};

const SCROLL: f32 = 1.0;
const SCALE_SCROLL: f32 = 0.1;

impl HandleEvent for Camera {
    fn handle(&mut self, event: InputEvent) {
        match event {
            InputEvent::CameraRight => {
                let mut pos = self.position().clone();
                pos.x += SCROLL;
                self.set_position(pos);
            }
            InputEvent::CameraLeft => {
                let mut pos = self.position().clone();
                pos.x -= SCROLL;
                self.set_position(pos);
            }
            InputEvent::CameraUp => {
                let mut pos = self.position().clone();
                pos.y += SCROLL;
                self.set_position(pos);
            }
            InputEvent::CameraDown => {
                let mut pos = self.position().clone();
                pos.y -= SCROLL;
                self.set_position(pos);
            }
            InputEvent::CameraScaleDown => {
                let scale = self.scale();
                self.set_scale(scale + SCALE_SCROLL);
            }
            InputEvent::CameraScaleUp => {
                let scale = self.scale();
                if scale > SCALE_SCROLL {
                    self.set_scale(scale - SCALE_SCROLL);
                }
            }
            _ => {}
        }
    }
}
