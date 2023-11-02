use ratatui::{
    style::Color,
    widgets::canvas::{Canvas, Context, Points},
};

use crate::model::{
    core::GameModel,
    enemy::{BasicEnemy, Enemy},
};

use super::core::{Camera, Drawable};

impl Drawable for BasicEnemy {
    fn draw(&self, frame: &mut ratatui::Frame, camera: &Camera, game_model: &dyn GameModel) {
        let frame_w = frame.size().width;
        let frame_h = frame.size().height;

        let self_trajectory = game_model.trajectory();
        let self_pos = self_trajectory.get_point(self.position());

        let paint_strat = |ctx: &mut Context| {
            if camera.allows_more_detail() {
                ctx.print(self_pos.x as f64, self_pos.y as f64, " ".trim())
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
            .paint(paint_strat);

        frame.render_widget(self_as_widget, camera.main_layout().split(frame.size())[0]);
    }
}
