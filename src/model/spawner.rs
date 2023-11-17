use std::time::Duration;

use rand::{seq::SliceRandom, thread_rng};

use super::{
    clock::Clock,
    core::EnemyUnique,
    enemy::{Enemy, BASIC_ENEMY_INFO, KINNETIC_RESIST_ENEMY_INFO, MAGIC_RESIST_ENEMY_INFO},
};

pub trait Spawner {
    fn maybe_spawn(&mut self) -> Option<EnemyUnique>;
}

#[derive(Default)]
pub struct BasicSpawner {}

impl Spawner for BasicSpawner {
    fn maybe_spawn(&mut self) -> Option<EnemyUnique> {
        Some(Box::new(Enemy::new(&BASIC_ENEMY_INFO)))
    }
}

pub struct SpawnerWithCooldown {
    last_spawn: Clock,
    cooldown: Duration,
}

impl SpawnerWithCooldown {
    pub fn new(cooldown: Duration) -> Self {
        Self {
            last_spawn: Clock::from_now(),
            cooldown: cooldown,
        }
    }

    fn produce_enemy() -> Box<Enemy> {
        let factories: [fn() -> Box<Enemy>; 3] = [
            || Box::new(Enemy::new(&BASIC_ENEMY_INFO)),
            || Box::new(Enemy::new(&KINNETIC_RESIST_ENEMY_INFO)),
            || Box::new(Enemy::new(&MAGIC_RESIST_ENEMY_INFO)),
        ];
        (factories.choose(&mut thread_rng()).unwrap())()
    }
}

impl Spawner for SpawnerWithCooldown {
    fn maybe_spawn(&mut self) -> Option<Box<Enemy>> {
        if self.last_spawn.elapsed() >= self.cooldown {
            self.last_spawn.tick();
            return Some(Self::produce_enemy());
        }
        return None;
    }
}
