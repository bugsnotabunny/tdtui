use std::time::Duration;

use crate::ui::PointDrawInfo;

use super::{
    core::{GameModel, UpdatableObject},
    damage::Damage,
    trajectory::Trajectory,
};

#[derive(Debug, Clone)]
pub struct EnemyInfo {
    pub max_health: f32,
    pub speed: f32,
    pub reward: u64,
    pub damage_eval: fn(Damage) -> f32,
    pub draw_info: PointDrawInfo,
}

#[derive(Debug, Clone, Copy)]
pub struct Enemy {
    health: f32,
    t_position: f32,
    type_info: &'static EnemyInfo,
}

impl Enemy {
    pub fn new(type_info: &'static EnemyInfo) -> Self {
        Self {
            health: type_info.max_health,
            t_position: 0.0,
            type_info: type_info,
        }
    }

    pub fn t_position(&self) -> f32 {
        self.t_position
    }

    pub fn is_dead(&self) -> bool {
        self.health <= 0.0
    }

    pub fn reward(&self) -> u64 {
        self.type_info.reward
    }

    pub fn take_damage(&mut self, damage: Damage) {
        self.health -= (self.type_info.damage_eval)(damage);
    }

    pub fn type_info(&self) -> &'static EnemyInfo {
        &self.type_info
    }
}

impl UpdatableObject for Enemy {
    fn on_update(&mut self, game_model: &mut impl GameModel, delta_time: Duration) {
        self.move_forward(delta_time, game_model.trajectory());
    }
}

impl Enemy {
    fn move_forward(&mut self, delta_time: Duration, trajectory: &dyn Trajectory) {
        const INITIAL_STEP: f32 = 1e-3;
        const EPSILON_MULTIPLYER: f32 = 1e2;
        const EPSILON: f32 = f32::EPSILON * EPSILON_MULTIPLYER;

        let mut move_points = self.type_info.speed * delta_time.as_secs_f32();
        let mut step = INITIAL_STEP;
        while move_points > EPSILON {
            let t_to_move_to = self.t_position + step;
            let self_pos = trajectory.point_from_t(self.t_position);
            let point_to_move_to = trajectory.point_from_t(t_to_move_to);
            let distance = self_pos.distance(point_to_move_to);

            if distance > move_points {
                step /= 2.0;
                continue;
            }

            move_points -= step;
            self.t_position += step;
        }
    }
}
