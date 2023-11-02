use std::time::Duration;

use crate::model::{
    damage::Damage,
    point::Point,
    tower::{ArcherTower, MageTower, Tower, TowerStats},
};

pub struct TowerTypeInfo {
    pub factory: fn(Point) -> Box<dyn Tower>,
    pub cooldown: Duration,
    pub damage: Damage,
    pub cost: u64,
    pub range: f32,
    pub name: &'static str,
    pub description: &'static str,
}

macro_rules! declare_tower_info {
    ($name:ident, $tower_type:ident, $factory_name:ident,$name_ingame:literal, $description:literal) => {
        fn $factory_name(position: Point) -> Box<dyn Tower> {
            Box::new($tower_type::new(position))
        }

        const $name: TowerTypeInfo = TowerTypeInfo {
            name: $name_ingame,
            factory: $factory_name,
            cooldown: $tower_type::COOLDOWN,
            damage: $tower_type::DAMAGE,
            cost: $tower_type::COST,
            range: $tower_type::RANGE,
            description: $description,
        };
    };
}

declare_tower_info!(
    ARCHER_INFO,
    ArcherTower,
    prod_archer,
    "Archer tower",
    "desc"
);
declare_tower_info!(MAGE_INFO, MageTower, prod_mage, "Mage tower", "desc");

const TOWER_SWITCHER_OPTS: [TowerTypeInfo; 2] = [ARCHER_INFO, MAGE_INFO];
#[derive(Debug, Default)]
pub struct TowerSelector {
    idx: usize,
}

impl TowerSelector {
    pub fn current(&self) -> &'static TowerTypeInfo {
        &TOWER_SWITCHER_OPTS[self.idx]
    }

    pub fn to_next(&mut self) {
        self.idx += 1;
        if self.idx >= TOWER_SWITCHER_OPTS.len() {
            self.idx = 0;
        }
    }
}
