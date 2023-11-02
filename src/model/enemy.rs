use std::time::Duration;

use crate::ui::core::Drawable;

use super::{
    core::{GameModel, UpdatableObject},
    damage::{Damage, DamageType},
    trajectory::Trajectory,
};

pub trait Enemy: Drawable + UpdatableObject {
    fn position(&self) -> f32;
    fn take_damage(&mut self, damage: Damage);
    fn is_dead(&self) -> bool;
    fn reward(&self) -> u64;
}

pub struct BasicEnemy {
    health: f32,
    speed: f32,
    position: f32,
    reward: u64,
}

impl BasicEnemy {
    pub fn new(health: f32, speed: f32, position: f32, reward: u64) -> Self {
        Self {
            health: health,
            speed: speed,
            position: position,
            reward: reward,
        }
    }
}

impl UpdatableObject for BasicEnemy {
    fn on_update(&mut self, game_model: &mut dyn GameModel, delta_time: Duration) {
        self.position = move_forward_impl(
            self.speed,
            self.position,
            delta_time,
            game_model.trajectory(),
        );
    }
}

fn move_forward_impl(
    speed: f32,
    mut position: f32,
    delta_time: Duration,
    trajectory: &dyn Trajectory,
) -> f32 {
    const INITIAL_STEP: f32 = 1e-3;
    const EPSILON_MULTIPLYER: f32 = 1e2;
    const EPSILON: f32 = f32::EPSILON * EPSILON_MULTIPLYER;

    let mut move_points = speed * delta_time.as_secs_f32();
    let mut step = INITIAL_STEP;
    while move_points > EPSILON {
        let t_to_move_to = position + step;
        let self_pos = trajectory.get_point(position);
        let point_to_move_to = trajectory.get_point(t_to_move_to);
        let distance = self_pos.distance(&point_to_move_to);

        if distance > move_points {
            step /= 2.0;
            continue;
        }

        move_points -= step;
        position += step;
    }
    position
}

impl Enemy for BasicEnemy {
    fn position(&self) -> f32 {
        self.position
    }

    fn is_dead(&self) -> bool {
        self.health <= 0.0
    }

    fn reward(&self) -> u64 {
        self.reward
    }

    fn take_damage(&mut self, damage: Damage) {
        self.health -= damage.value;
    }
}

pub struct EnemyWithKinneticResist {
    health: f32,
    resist: f32,
    speed: f32,
    position: f32,
    reward: u64,
}

impl EnemyWithKinneticResist {
    pub fn new(health: f32, resist: f32, speed: f32, position: f32, reward: u64) -> Self {
        Self {
            health: health,
            resist: resist,
            speed: speed,
            position: position,
            reward: reward,
        }
    }
}

impl UpdatableObject for EnemyWithKinneticResist {
    fn on_update(&mut self, game_model: &mut dyn GameModel, delta_time: Duration) {
        self.position = move_forward_impl(
            self.speed,
            self.position,
            delta_time,
            game_model.trajectory(),
        );
    }
}

impl Enemy for EnemyWithKinneticResist {
    fn position(&self) -> f32 {
        self.position
    }

    fn is_dead(&self) -> bool {
        self.health <= 0.0
    }

    fn reward(&self) -> u64 {
        self.reward
    }

    fn take_damage(&mut self, damage: Damage) {
        let value = match damage.kind {
            DamageType::Kinnetic => damage.value * (1.0 - self.resist),
            _ => damage.value,
        };
        self.health -= value;
    }
}

pub struct EnemyWithMagicResist {
    health: f32,
    resist: f32,
    speed: f32,
    position: f32,
    reward: u64,
}

impl EnemyWithMagicResist {
    pub fn new(health: f32, resist: f32, speed: f32, position: f32, reward: u64) -> Self {
        Self {
            health: health,
            resist: resist,
            speed: speed,
            position: position,
            reward: reward,
        }
    }
}

impl UpdatableObject for EnemyWithMagicResist {
    fn on_update(&mut self, game_model: &mut dyn GameModel, delta_time: Duration) {
        self.position = move_forward_impl(
            self.speed,
            self.position,
            delta_time,
            game_model.trajectory(),
        );
    }
}

impl Enemy for EnemyWithMagicResist {
    fn position(&self) -> f32 {
        self.position
    }

    fn is_dead(&self) -> bool {
        self.health <= 0.0
    }

    fn reward(&self) -> u64 {
        self.reward
    }

    fn take_damage(&mut self, damage: Damage) {
        let value = match damage.kind {
            DamageType::Magic => damage.value * (1.0 - self.resist),
            _ => damage.value,
        };
        self.health -= value;
    }
}
