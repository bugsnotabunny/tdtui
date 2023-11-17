use super::{
    point::Point,
    tower::{Tower, TowerInfo, ARCHER_TOWER_INFO, MAGE_TOWER_INFO},
};

const TOWER_SELECTOR_OPTS: &[TowerInfo] = &[ARCHER_TOWER_INFO, MAGE_TOWER_INFO];

#[derive(Debug, Default)]
pub struct TowerSelector {
    idx: usize,
}

impl TowerSelector {
    pub fn produce_current(&self, position: Point) -> Tower {
        Tower::new(position, &TOWER_SELECTOR_OPTS[self.idx])
    }

    pub fn current(&self) -> &'static TowerInfo {
        &TOWER_SELECTOR_OPTS[self.idx]
    }

    pub fn to_next(&mut self) {
        self.idx += 1;
        if self.idx >= TOWER_SELECTOR_OPTS.len() {
            self.idx = 0;
        }
    }
}
