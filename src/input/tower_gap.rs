use std::error::Error;

use super::core::{HandleEvent, InputContext, InputEvent};
use crate::model::{Point, Positioned};

#[derive(Debug, Clone, Copy)]
pub struct TowerGap {
    position: Point,
    range: f32,
}

impl TowerGap {
    pub fn new(position: Point, radius: f32) -> Self {
        Self {
            position: position,
            range: radius,
        }
    }

    pub fn gap(&self) -> f32 {
        self.range
    }
}

impl Positioned for TowerGap {
    fn position(&self) -> Point {
        self.position
    }
}

impl HandleEvent for TowerGap {
    fn handle(
        &mut self,
        event: InputEvent,
        input_context: &InputContext,
    ) -> Result<(), Box<dyn Error>> {
        match event {
            InputEvent::MouseMovedTo(pos) => {
                self.position = pos.to_world_point(input_context.screen_info());
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
