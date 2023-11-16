use std::{cell::RefCell, time::Duration};

use rand::{seq::SliceRandom, thread_rng};

use super::{assets::enemy::*, clock::Clock, enemy::*};

pub trait Spawner {
    fn maybe_spawn(&mut self) -> Option<Box<RefCell<dyn Enemy>>>;
}

#[derive(Default)]
pub struct BasicSpawner {}

impl Spawner for BasicSpawner {
    fn maybe_spawn(&mut self) -> Option<Box<RefCell<dyn Enemy>>> {
        Some(Box::new(RefCell::new(BasicEnemy::new(4.0, 5.0, 0.0, 3))))
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

    fn produce_enemy() -> Box<RefCell<dyn Enemy>> {
        let factories: [fn() -> Box<RefCell<dyn Enemy>>; 3] = [
            || Box::new(RefCell::new(BasicEnemy::new(4.0, 5.0, 0.0, 3))),
            || {
                Box::new(RefCell::new(EnemyWithKinneticResist::new(
                    4.0, 0.8, 5.0, 0.0, 3,
                )))
            },
            || {
                Box::new(RefCell::new(EnemyWithMagicResist::new(
                    4.0, 0.6, 5.0, 0.0, 3,
                )))
            },
        ];
        (factories.choose(&mut thread_rng()).unwrap())()
    }
}

impl Spawner for SpawnerWithCooldown {
    fn maybe_spawn(&mut self) -> Option<Box<RefCell<dyn Enemy>>> {
        if self.last_spawn.elapsed() >= self.cooldown {
            self.last_spawn.tick();
            return Some(Self::produce_enemy());
        }
        return None;
    }
}
