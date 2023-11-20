use std::error::Error;

use super::core::{HandleEvent, InputContext, InputEvent};
use crate::model::{ConcreteGameModel, GameModel, Spawner, Trajectory};

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
                        .produce_current(input.to_world_point(input_context.screen_info())),
                )?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
