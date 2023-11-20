use ratatui::{
    style::Color,
    symbols::Marker,
    widgets::canvas::{Canvas, Circle},
    Frame,
};

use super::core::{Camera, Drawable};
use crate::{
    assets::tower_ui::{TOWER_GAP_DRAW_INFO, TOWER_RADIUS_DRAW_INFO},
    input::{TowerGap, TowerRadius},
    model::Positioned,
};

#[derive(Debug, Clone, Copy)]
pub struct CircleDrawInfo {
    pub marker: Marker,
    pub fg_color: Color,
}

pub trait HasCircleDrawInfo: Positioned {
    fn draw_info(&self) -> &'static CircleDrawInfo;
    fn radius(&self) -> f32;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CircleDrawable<'a, T: HasCircleDrawInfo> {
    instance: &'a T,
}

impl<'a, T: HasCircleDrawInfo> CircleDrawable<'a, T> {
    pub fn new(instance: &'a T) -> Self {
        Self { instance: instance }
    }

    pub fn radius(&self) -> f32 {
        self.instance.radius()
    }
}

impl<'a, T: HasCircleDrawInfo> Drawable for CircleDrawable<'a, T> {
    fn draw(&self, frame: &mut Frame, camera: &Camera) {
        let frame_w = frame.size().width;
        let frame_h = frame.size().height;
        let self_pos = self.instance.position();
        let draw_info = self.instance.draw_info();

        let self_as_widget = Canvas::default()
            .marker(draw_info.marker)
            .paint(|ctx| {
                {
                    ctx.draw(&Circle {
                        x: self_pos.x as f64,
                        y: self_pos.y as f64,
                        radius: self.radius() as f64,
                        color: draw_info.fg_color,
                    })
                }
            })
            .x_bounds(camera.x_bounds(frame_w))
            .y_bounds(camera.y_bounds(frame_h));

        frame.render_widget(self_as_widget, camera.main_layout().split(frame.size())[0]);
    }
}

impl HasCircleDrawInfo for TowerGap {
    fn radius(&self) -> f32 {
        self.gap()
    }

    fn draw_info(&self) -> &'static CircleDrawInfo {
        &TOWER_GAP_DRAW_INFO
    }
}

impl HasCircleDrawInfo for TowerRadius {
    fn radius(&self) -> f32 {
        self.range()
    }

    fn draw_info(&self) -> &'static CircleDrawInfo {
        &TOWER_RADIUS_DRAW_INFO
    }
}
