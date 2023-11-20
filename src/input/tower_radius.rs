use std::error::Error;

use crate::model::{
    point::{Point, Positioned},
    tower::TowerInfo,
};

use super::core::{HandleEvent, InputContext, InputEvent};

#[derive(Debug, Clone, Copy)]
pub struct TowerRadius {
    position: Point,
    type_info: &'static TowerInfo,
}

impl TowerRadius {
    pub fn new(position: Point, type_info: &'static TowerInfo) -> Self {
        Self {
            position: position,
            type_info: type_info,
        }
    }

    pub fn radius(&self) -> f32 {
        self.type_info.range
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
            _ => Ok(()),
        }
    }
}
