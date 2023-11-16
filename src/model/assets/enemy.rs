use macros::{EnemyCommon, EnemyConstructor, EnemyConstructorResist};

use crate::model::{
    core::{GameModel, UpdatableObject},
    damage::{Damage, DamageType},
    enemy::{Enemy, EnemyCommon},
    trajectory::Trajectory,
};

use std::time::Duration;

#[derive(EnemyCommon, EnemyConstructor)]
pub struct BasicEnemy {
    health: f32,
    speed: f32,
    position: f32,
    reward: u64,
}

#[derive(EnemyCommon, EnemyConstructorResist)]
pub struct EnemyWithKinneticResist {
    health: f32,
    resist: f32,
    speed: f32,
    position: f32,
    reward: u64,
}

#[derive(EnemyCommon, EnemyConstructorResist)]
pub struct EnemyWithMagicResist {
    health: f32,
    resist: f32,
    speed: f32,
    position: f32,
    reward: u64,
}

impl Enemy for BasicEnemy {
    fn take_damage(&mut self, damage: Damage) {
        self.health -= damage.value;
    }
}

impl Enemy for EnemyWithKinneticResist {
    fn take_damage(&mut self, damage: Damage) {
        let value = match damage.kind {
            DamageType::Kinnetic => damage.value * (1.0 - self.resist),
            _ => damage.value,
        };
        self.health -= value;
    }
}

impl Enemy for EnemyWithMagicResist {
    fn take_damage(&mut self, damage: Damage) {
        let value = match damage.kind {
            DamageType::Magic => damage.value * (1.0 - self.resist),
            _ => damage.value,
        };
        self.health -= value;
    }
}
