use ratatui::{
    style::Color,
    widgets::canvas::{Canvas, Points},
};

use crate::model::{core::GameModel, enemy::Enemy};

use super::core::{Camera, Drawable};

impl Drawable for Enemy {
    fn draw(&self, frame: &mut ratatui::Frame, camera: &Camera, game_model: &dyn GameModel) {
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
