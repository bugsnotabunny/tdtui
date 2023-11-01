use ratatui::{
    style::Color,
    widgets::canvas::{Canvas, Points},
};

use crate::model::{core::GameModel, tower::Tower};

use super::core::{Camera, Drawable};

pub struct TowerDrawable<'a> {
    pub tower: &'a dyn Tower,
}

impl<'a> Drawable for TowerDrawable<'a> {
    fn draw(&self, frame: &mut ratatui::Frame, camera: &Camera, _: &impl GameModel) {
        let frame_w = frame.size().width;
        let frame_h = frame.size().height;

        let self_pos = self.tower.position();

        let self_as_widget = Canvas::default()
            .marker(ratatui::symbols::Marker::Dot)
            .paint(|ctx| {
                ctx.draw(&Points {
                    coords: &[(self_pos.x as f64, self_pos.y as f64)],
                    color: Color::Green,
                })
            })
            .x_bounds(camera.x_bounds(frame_w))
            .y_bounds(camera.y_bounds(frame_h));

        frame.render_widget(self_as_widget, camera.main_layout().split(frame.size())[0]);
    }
}
