use std::error::Error;

use crate::model::{core::GameModel, spawner::Spawner, trajectory::Trajectory};

use super::core::{HandleEvent, InputContext, InputEvent};

use crate::model::core::ConcreteGameModel;

impl<S: Spawner, T: Trajectory> HandleEvent for ConcreteGameModel<S, T> {
    fn handle(
        &mut self,
        event: InputEvent,
        input_context: &InputContext,
    ) -> Result<(), Box<dyn Error>> {
        match event {
            InputEvent::MousePressedL(input) => {
                self.spawn_tower(
                    input_context
                        .tower_selector()
                        .produce_current(input.to_world_point()),
                )?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
