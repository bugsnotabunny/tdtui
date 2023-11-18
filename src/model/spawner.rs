use std::time::Duration;

use rand::{seq::SliceRandom, thread_rng};

use crate::assets::enemy::*;

use super::{
    core::{GameModel, UpdatableObject},
    enemy::Enemy,
};

pub trait Spawner: UpdatableObject + Default {}

#[derive(Default)]
pub struct RandomizedSpawnerWithCooldown {
    cooldown_elapsed: Duration,
    cooldown: Duration,
}

impl RandomizedSpawnerWithCooldown {
    pub fn new(cooldown: Duration) -> Self {
        Self {
            cooldown_elapsed: Duration::from_millis(0),
            cooldown: cooldown,
        }
    }
}

impl UpdatableObject for RandomizedSpawnerWithCooldown {
    fn on_update(&mut self, game_model: &mut dyn GameModel, delta_time: Duration) {
        self.cooldown_elapsed += delta_time;

        if self.cooldown_elapsed >= self.cooldown {
            self.spawn(game_model);
            self.cooldown_elapsed = Duration::from_millis(0);
        }
    }
}

impl RandomizedSpawnerWithCooldown {
    fn spawn(&self, game_model: &mut dyn GameModel) {
        game_model.spawn_enemy(Self::produce_enemy())
    }

    fn produce_enemy() -> Enemy {
        const FACTORIES: &[fn() -> Enemy] = &[
            || Enemy::new(&BASIC_ENEMY_INFO),
            || Enemy::new(&KINNETIC_RESIST_ENEMY_INFO),
            || Enemy::new(&MAGIC_RESIST_ENEMY_INFO),
        ];
        (FACTORIES.choose(&mut thread_rng()).unwrap())()
    }
}

impl Spawner for RandomizedSpawnerWithCooldown {}
