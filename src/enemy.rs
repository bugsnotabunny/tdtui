use crate::{
    damage::Damage, game::GameModel, spawner::Spawner, trajectory::Trajectory, ui::Drawable,
};

use ratatui::{
    style::Color,
    widgets::canvas::{Canvas, Points},
};

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

    pub fn move_forward(&mut self, trajectory: &impl Trajectory) {
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

    pub fn take_damage(&mut self, damage: Damage) {
        match self.health.checked_sub(damage.value) {
            Some(health) => self.health = health,
            None => self.health = 0,
        }
    }
}

impl<T: Trajectory, S: Spawner> Drawable<T, S> for Enemy {
    fn draw(
        &self,
        frame: &mut ratatui::Frame<ratatui::prelude::CrosstermBackend<std::io::Stdout>>,
        camera: &crate::ui::Camera,
        game_model: &GameModel<T, S>,
    ) {
        let frame_w = frame.size().width;
        let frame_h = frame.size().height;

        let self_trajectory = game_model.road().trajectory();
        let self_pos = self_trajectory.get_point(self.position());

        let self_as_widget = Canvas::default()
            .paint(|ctx| {
                ctx.draw(&Points {
                    coords: &[(self_pos.x as f64, self_pos.y as f64)],
                    color: Color::LightRed,
                });
            })
            .x_bounds(camera.x_bounds(frame_w))
            .y_bounds(camera.y_bounds(frame_h));

        frame.render_widget(self_as_widget, camera.main_layout().split(frame.size())[0]);
    }
}
