use ratatui::{
    style::{Color, Stylize},
    widgets::canvas::{Canvas, Points},
};

use crate::model::{
    core::GameModel,
    tower::{ArcherTower, Tower},
};

use super::core::{Camera, Drawable};

impl Drawable for ArcherTower {
    fn draw(&self, frame: &mut ratatui::Frame, camera: &Camera, _: &dyn GameModel) {
        let frame_w = frame.size().width;
        let frame_h = frame.size().height;

        let self_pos = self.position();

        let self_as_widget = Canvas::default()
            .marker(ratatui::symbols::Marker::Dot)
            .paint(|ctx| {
                if camera.allows_more_detail() {
                    ctx.print(self_pos.x as f64, self_pos.y as f64, "".green())
                } else {
                    ctx.draw(&Points {
                        coords: &[(self_pos.x as f64, self_pos.y as f64)],
                        color: Color::Green,
                    })
                };
            })
            .x_bounds(camera.x_bounds(frame_w))
            .y_bounds(camera.y_bounds(frame_h));

        frame.render_widget(self_as_widget, camera.main_layout().split(frame.size())[0]);
    }
}
