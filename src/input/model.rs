use std::error::Error;

use crate::model::{core::ConcreteGameModel, road::Road, tower::ArcherTower};

use super::core::{HandleEvent, InputEvent};

impl<R: Road> HandleEvent for ConcreteGameModel<R> {
    fn handle(&mut self, event: InputEvent) -> Result<(), Box<dyn Error>> {
        match event {
            InputEvent::MousePressedL(input) => {
                self.maybe_build(Box::new(ArcherTower::new(input.to_world_point())))?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
