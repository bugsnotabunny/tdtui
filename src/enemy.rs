use crate::damage::Damage;

pub struct Enemy {
    health: u8,
    speed: u8,
}

impl Enemy {
    pub fn new(health: u8, speed: u8) -> Self {
        Self {
            health: health,
            speed: speed,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.health == 0
    }

    pub fn speed(&self) -> u8 {
        self.speed
    }

    pub fn health(&self) -> u8 {
        self.health
    }

    pub fn take_damage(&mut self, damage: Damage) {
        match self.health.checked_sub(damage.value) {
            Some(health) => self.health = health,
            None => self.health = 0,
        }
    }
}
