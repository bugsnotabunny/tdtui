use super::enemy::Enemy;

pub trait Spawner {
    fn spawn(&self) -> Enemy;
}

#[derive(Default)]
pub struct BasicSpawner {}

impl Spawner for BasicSpawner {
    fn spawn(&self) -> Enemy {
        Enemy::new(4, 5.0, 0.0)
    }
}