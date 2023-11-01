use std::{cell::RefCell, error::Error};

use crate::model::{
    core::ConcreteGameModel, spawner::Spawner, tower::ArcherTower, trajectory::Trajectory,
};

use super::core::{HandleEvent, InputEvent};

impl<S: Spawner, T: Trajectory> HandleEvent for ConcreteGameModel<S, T> {
    fn handle(&mut self, event: InputEvent) -> Result<(), Box<dyn Error>> {
        match event {
            InputEvent::MousePressedL(input) => {
                self.maybe_build(Box::new(RefCell::new(ArcherTower::new(
                    input.to_world_point(),
                ))))?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
