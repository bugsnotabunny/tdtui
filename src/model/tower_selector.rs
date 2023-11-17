use super::{
    point::Point,
    tower::{Tower, TowerInfo, ARCHER_TOWER_INFO, MAGE_TOWER_INFO},
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
        if self.idx >= Self::OPTS.len() {
            self.idx = 0;
        }
    }
}
