use std::time::Duration;

use super::{clock::Clock, enemy::BasicEnemy};

pub trait Spawner {
    fn maybe_spawn(&mut self) -> Option<BasicEnemy>;
}

#[derive(Default)]
pub struct BasicSpawner {}

impl Spawner for BasicSpawner {
    fn maybe_spawn(&mut self) -> Option<BasicEnemy> {
        Some(BasicEnemy::new(4, 5.0, 0.0))
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
    fn maybe_spawn(&mut self) -> Option<BasicEnemy> {
        if self.last_spawn.elapsed() >= self.cooldown {
            self.last_spawn.tick();
            return Some(BasicEnemy::new(4, 5.0, 0.0));
        }
        return None;
    }
}
