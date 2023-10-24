use crate::{damage::Damage, update::Update};

pub struct Enemy {
    health: u8,
    speed: f32,
    position: Point,
}

impl Enemy {
    pub fn new(health: u8, speed: f32, position: Point) -> Self {
        Self {
            health: health,
            speed: speed,
            position: position,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.health == 0
    }

    pub fn position(&self) -> &Point {
        &self.position
    }

    pub fn move_forward(&mut self, value: f32) {
        self.position += value;
    }

    pub fn take_damage(&mut self, damage: Damage) {
        match self.health.checked_sub(damage.value) {
            Some(health) => self.health = health,
            None => self.health = 0,
        }
    }
}

impl Update for Enemy {
    fn update(&mut self) {
        self.move_forward(self.speed);
    }
}
