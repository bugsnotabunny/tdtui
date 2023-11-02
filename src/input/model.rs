use std::error::Error;

use crate::model::{core::GameModel, spawner::Spawner, trajectory::Trajectory};

use super::core::{HandleEvent, InputEvent};

use crate::model::core::ConcreteGameModel;

impl<S: Spawner, T: Trajectory> HandleEvent for ConcreteGameModel<S, T> {
    fn handle(&mut self, event: InputEvent) -> Result<(), Box<dyn Error>> {
        self.selector_mut().handle(event.clone())?;
        match event {
            InputEvent::MousePressedL(input) => {
                self.maybe_build_from_selector(input.to_world_point())?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
