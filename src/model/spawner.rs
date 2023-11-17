use std::time::Duration;

use rand::{seq::SliceRandom, thread_rng};

use crate::assets::enemy::*;

use super::{
    core::{GameModel, UpdatableObject},
    enemy::Enemy,
};

pub trait Spawner: UpdatableObject + Default {
    fn spawn(&mut self);
    fn take_spawned(&mut self) -> Option<Enemy>;
}

#[derive(Default)]
pub struct RandomizedSpawnerWithCooldown {
    cooldown_elapsed: Duration,
    cooldown: Duration,
    buf: Option<Enemy>,
}

impl RandomizedSpawnerWithCooldown {
    pub fn new(cooldown: Duration) -> Self {
        Self {
            cooldown_elapsed: Duration::from_millis(0),
            cooldown: cooldown,
            buf: None,
        }
    }
}

impl UpdatableObject for RandomizedSpawnerWithCooldown {
    fn on_update(&mut self, _: &mut dyn GameModel, delta_time: Duration) {
        self.cooldown_elapsed += delta_time;

        if self.cooldown_elapsed >= self.cooldown {
            self.cooldown_elapsed = Duration::from_millis(0);
            self.spawn();
        }
    }
}

impl Spawner for RandomizedSpawnerWithCooldown {
    fn spawn(&mut self) {
        self.buf = Some(Self::produce_enemy());
    }

    fn take_spawned(&mut self) -> Option<Enemy> {
        return std::mem::take(&mut self.buf);
    }
}

impl RandomizedSpawnerWithCooldown {
    fn produce_enemy() -> Enemy {
        const FACTORIES: &[fn() -> Enemy] = &[
            || Enemy::new(&BASIC_ENEMY_INFO),
            || Enemy::new(&KINNETIC_RESIST_ENEMY_INFO),
            || Enemy::new(&MAGIC_RESIST_ENEMY_INFO),
        ];
        (FACTORIES.choose(&mut thread_rng()).unwrap())()
    }
}
