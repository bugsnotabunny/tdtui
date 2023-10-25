use crate::ui::core::Camera;

use super::core::{HandleEvents, InputMask};

const SCROLL: f32 = 1.0;
const SCALE_SCROLL: f32 = 0.1;

impl HandleEvents for Camera {
    fn handle(&mut self, events: InputMask) {
        match events {
            InputMask::RightPressed => {
                let mut pos = self.position();
                pos.0 += SCROLL;
                self.set_position(pos);
            }
            InputMask::LeftPressed => {
                let mut pos = self.position();
                pos.0 -= SCROLL;
                self.set_position(pos);
            }
            InputMask::UpPressed => {
                let mut pos = self.position();
                pos.1 += SCROLL;
                self.set_position(pos);
            }
            InputMask::DownPressed => {
                let mut pos = self.position();
                pos.1 -= SCROLL;
                self.set_position(pos);
            }
            InputMask::ScaleDownPressed => {
                let scale = self.scale();
                if scale > SCALE_SCROLL {
                    self.set_scale(scale - SCALE_SCROLL);
                }
            }
            InputMask::ScaleUpPressed => {
                let scale = self.scale();
                self.set_scale(scale + SCALE_SCROLL);
            }
            _ => {}
        }
    }
}
