use std::time::Duration;

use crate::ui::core::Drawable;

use super::{
    core::{GameModel, UpdatableObject},
    damage::Damage,
    trajectory::Trajectory,
};

pub trait Enemy: Drawable + UpdatableObject {
    fn position(&self) -> f32;
    fn take_damage(&mut self, damage: Damage);
    fn is_dead(&self) -> bool;
    fn reward(&self) -> u64;
}

pub struct BasicEnemy {
    health: u8,
    speed: f32,
    position: f32,
    reward: u64,
}

impl BasicEnemy {
    pub fn new(health: u8, speed: f32, position: f32, reward: u64) -> Self {
        Self {
            health: health,
            speed: speed,
            position: position,
            reward: reward,
        }
    }
}

impl Enemy for BasicEnemy {
    fn position(&self) -> f32 {
        self.position
    }

    fn take_damage(&mut self, damage: Damage) {
        self.health = self.health.checked_sub(damage.value).unwrap_or(0);
    }

    fn is_dead(&self) -> bool {
        self.health == 0
    }

    fn reward(&self) -> u64 {
        self.reward
    }
}

impl UpdatableObject for BasicEnemy {
    fn on_update(&mut self, game_model: &mut dyn GameModel, delta_time: Duration) {
        self.move_forward(delta_time, game_model.trajectory());
    }
}

impl BasicEnemy {
    fn move_forward(&mut self, delta_time: Duration, trajectory: &dyn Trajectory) {
        const INITIAL_STEP: f32 = 1e-3;
        const EPSILON_MULTIPLYER: f32 = 1e2;
        const EPSILON: f32 = f32::EPSILON * EPSILON_MULTIPLYER;

        let mut move_points = self.speed * delta_time.as_secs_f32();
        let mut step = INITIAL_STEP;
        while move_points > EPSILON {
            let t_to_move_to = self.position + step;
            let self_pos = trajectory.get_point(self.position);
            let point_to_move_to = trajectory.get_point(t_to_move_to);
            let distance = self_pos.distance(&point_to_move_to);

            if distance > move_points {
                step /= 2.0;
                continue;
            }

            move_points -= step;
            self.position += step;
        }
    }
}
