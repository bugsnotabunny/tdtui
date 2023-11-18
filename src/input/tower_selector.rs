use std::error::Error;

use super::core::{HandleEvent, InputContext, InputEvent};

use crate::assets::tower::*;

use crate::model::{
    point::Point,
    tower::{Tower, TowerInfo},
};

#[derive(Debug, Default)]
pub struct TowerSelector {
    idx: usize,
}

impl TowerSelector {
    const OPTS: &[TowerInfo] = &[ARCHER_TOWER_INFO, MAGE_TOWER_INFO];

    pub fn produce_current(&self, position: Point) -> Tower {
        Tower::new(position, &Self::OPTS[self.idx])
    }

    pub fn current(&self) -> &'static TowerInfo {
        &Self::OPTS[self.idx]
    }

    pub fn to_next(&mut self) {
        self.idx += 1;
        self.idx %= Self::OPTS.len();
    }
}

impl HandleEvent for TowerSelector {
    fn handle(&mut self, event: InputEvent, _: &InputContext) -> Result<(), Box<dyn Error>> {
        match event {
            InputEvent::TowerSelectorNext => {
                self.to_next();
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
