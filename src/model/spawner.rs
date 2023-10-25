use std::time::Duration;

use super::{clock::Clock, enemy::Enemy};

pub trait Spawner {
    fn maybe_spawn(&mut self) -> Option<Enemy>;
}

#[derive(Default)]
pub struct BasicSpawner {}

impl Spawner for BasicSpawner {
    fn maybe_spawn(&mut self) -> Option<Enemy> {
        Some(Enemy::new(4, 5.0, 0.0))
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
}

impl Spawner for SpawnerWithCooldown {
    fn maybe_spawn(&mut self) -> Option<Enemy> {
        if self.last_spawn.elapsed() >= self.cooldown {
            self.last_spawn.tick();
            return Some(Enemy::new(4, 5.0, 0.0));
        }
        return None;
    }
}
