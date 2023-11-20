use ratatui::{
    style::Color,
    symbols::Marker,
    widgets::canvas::{Canvas, Circle},
    Frame,
};

use crate::{input::tower_radius::TowerRadius, model::point::Positioned};

use super::core::{Camera, Drawable};

impl Drawable for TowerRadius {
    fn draw(&self, frame: &mut Frame, camera: &Camera) {
        let frame_w = frame.size().width;
        let frame_h = frame.size().height;

        let position = self.position();

        let self_as_widget = Canvas::default()
            .marker(Marker::Braille)
            .paint(|ctx| {
                ctx.draw(&Circle {
                    x: position.x as f64,
                    y: position.y as f64,
                    radius: self.radius() as f64,
                    color: Color::Gray,
                })
            })
            .x_bounds(camera.x_bounds(frame_w))
            .y_bounds(camera.y_bounds(frame_h));

        frame.render_widget(self_as_widget, camera.main_layout().split(frame.size())[0]);
    }
}
