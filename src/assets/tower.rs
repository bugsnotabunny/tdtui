use std::time::Duration;

use crate::model::{
    damage::{Damage, DamageType},
    tower::TowerInfo,
};

pub const ARCHER_TOWER_INFO: TowerInfo = TowerInfo {
    cooldown: Duration::from_millis(1500),
    cost: 10,
    range: 50.0,
    damage: Damage {
        value: 10.0,
        kind: DamageType::Kinnetic,
    },
    name: "Archer tower",
    description: "",
    close_up_sprite: "",
};

pub const MAGE_TOWER_INFO: TowerInfo = TowerInfo {
    cooldown: Duration::from_millis(2000),
    cost: 10,
    range: 100.0,
    damage: Damage {
        value: 5.0,
        kind: DamageType::Magic,
    },
    name: "Mage tower",
    description: "",
    close_up_sprite: "",
};