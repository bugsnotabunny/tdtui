use super::{damage::Damage, trajectory::Trajectory};

pub struct Enemy {
    health: u8,
    speed: f32,
    position: f32,
}

impl Enemy {
    pub fn new(health: u8, speed: f32, position: f32) -> Self {
        Self {
            health: health,
            speed: speed,
            position: position,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.health == 0
    }

    pub fn position(&self) -> f32 {
        self.position
    }

    pub fn take_damage(&mut self, damage: Damage) {
        match self.health.checked_sub(damage.value) {
            Some(health) => self.health = health,
            None => self.health = 0,
        }
    }

    fn move_forward(&mut self, trajectory: &dyn Trajectory) {
        const INITIAL_STEP: f32 = 1e-2;
        const EPSILON_MULTIPLYER: f32 = 1e2;
        const EPSILON: f32 = f32::EPSILON * EPSILON_MULTIPLYER;

        let mut move_points = self.speed;
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

    pub fn on_update(&mut self, self_trajectory: &dyn Trajectory) {
        self.move_forward(self_trajectory);
    }
}
