use std::error::Error;

use crate::model::{
    point::{Point, Positioned},
    tower::TowerInfo,
};

use super::core::{HandleEvent, InputContext, InputEvent};

#[derive(Debug, Clone, Copy)]
pub struct TowerRadius {
    position: Point,
    range: f32,
}

impl TowerRadius {
    pub fn new(position: Point, radius: f32) -> Self {
        Self {
            position: position,
            range: radius,
        }
    }

    pub fn range(&self) -> f32 {
        self.range
    }
}

impl Positioned for TowerRadius {
    fn position(&self) -> Point {
        self.position
    }
}

impl HandleEvent for TowerRadius {
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
            InputEvent::TowerSelectorNext => {
                self.range = input_context.tower_selector().current().range;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
