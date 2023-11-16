use crate::model::{
    clock::Clock,
    core::{GameModel, UpdatableObject},
    damage::{Damage, DamageType},
    point::Point,
    tower::{Aim, Tower, TowerStats},
};

use macros::*;
use std::time::Duration;

use rand::seq::IteratorRandom;

#[derive(Tower, TowerConstructor)]
pub struct ArcherTower {
    aim: Aim,
    position: Point,
    cooldown_clock: Clock,
}

impl TowerStats for ArcherTower {
    const COOLDOWN: Duration = Duration::from_millis(1500);
    const COST: u64 = 10;
    const RANGE: f32 = 50.0;
    const DAMAGE: Damage = Damage {
        value: 10.0,
        kind: DamageType::Kinnetic,
    };
}

#[derive(Tower, TowerConstructor)]
pub struct MageTower {
    aim: Aim,
    position: Point,
    cooldown_clock: Clock,
}

impl TowerStats for MageTower {
    const COOLDOWN: Duration = Duration::from_millis(2000);
    const COST: u64 = 20;
    const RANGE: f32 = 100.0;
    const DAMAGE: Damage = Damage {
        value: 5.0,
        kind: DamageType::Magic,
    };
}
