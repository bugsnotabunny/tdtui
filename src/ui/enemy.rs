use ratatui::{
    style::{Color, Stylize},
    symbols::Marker,
    widgets::canvas::{Canvas, Context, Points},
};

use crate::model::{core::GameModel, enemy::Enemy};

use super::core::{Camera, Drawable};

pub struct EnemyDrawable<'a> {
    enemy: &'a Enemy,
}

impl<'a> EnemyDrawable<'a> {
    pub fn new(enemy: &'a Enemy) -> Self {
        Self { enemy }
    }
}

impl<'a> Drawable for EnemyDrawable<'a> {
    fn draw(&self, frame: &mut ratatui::Frame, camera: &Camera, game_model: &dyn GameModel) {
        let frame_w = frame.size().width;
        let frame_h = frame.size().height;

        let self_trajectory = game_model.trajectory();
        let self_pos = self_trajectory.get_point(self.enemy.t_position());

        let paint_strat = |ctx: &mut Context| {
            if camera.allows_more_detail() {
                ctx.print(
                    self_pos.x as f64,
                    self_pos.y as f64,
                    self.enemy.type_info().close_up_sprite.light_red(),
                )
            } else {
                ctx.draw(&Points {
                    coords: &[(self_pos.x as f64, self_pos.y as f64)],
                    color: Color::LightRed,
                });
            };
        };

        let self_as_widget = Canvas::default()
            .x_bounds(camera.x_bounds(frame_w))
            .y_bounds(camera.y_bounds(frame_h))
            .paint(paint_strat)
            .marker(Marker::HalfBlock);

        frame.render_widget(self_as_widget, camera.main_layout().split(frame.size())[0]);
    }
}
