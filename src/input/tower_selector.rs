use std::error::Error;

use crate::model::tower_selector::TowerSelector;

use super::core::{HandleEvent, InputEvent};

impl HandleEvent for TowerSelector {
    fn handle(&mut self, event: InputEvent) -> Result<(), Box<dyn Error>> {
        match event {
            InputEvent::TowerSelectorNext => {
                self.to_next();
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
