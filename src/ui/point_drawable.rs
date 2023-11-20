use ratatui::{
    style::{Color, Modifier, Stylize},
    symbols::Marker,
    widgets::canvas::{Canvas, Points},
    Frame,
};

use super::core::{Camera, Drawable};
use crate::{
    assets::tower::PROJECTILE_DRAW_INFO,
    model::{Enemy, Point, Positioned, Projectile, Tower, Trajectory},
};

#[derive(Debug, Clone, Copy)]
pub struct PointDrawInfo {
    pub marker: Marker,
    pub modifiers: Modifier,
    pub fg_color: Color,
    pub bg_color: Color,
    pub close_up_sprite: Option<&'static str>,
}

pub trait HasPointDrawInfo: Positioned {
    fn draw_info(&self) -> &'static PointDrawInfo;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PointDrawable<'a, T: HasPointDrawInfo> {
    instance: &'a T,
}

impl<'a, T: HasPointDrawInfo> PointDrawable<'a, T> {
    pub fn new(instance: &'a T) -> Self {
        Self { instance: instance }
    }
}

impl<'a, T: HasPointDrawInfo> Drawable for PointDrawable<'a, T> {
    fn draw(&self, frame: &mut Frame, camera: &Camera) {
        let frame_w = frame.size().width;
        let frame_h = frame.size().height;
        let self_pos = self.instance.position();
        let draw_info = self.instance.draw_info();

        let self_as_widget = Canvas::default()
            .marker(draw_info.marker)
            .paint(|ctx| {
                if draw_info.close_up_sprite.is_some() && camera.allows_more_detail() {
                    ctx.print(
                        self_pos.x as f64,
                        self_pos.y as f64,
                        draw_info
                            .close_up_sprite
                            .unwrap()
                            .fg(draw_info.fg_color)
                            .bg(draw_info.bg_color)
                            .add_modifier(draw_info.modifiers),
                    );
                } else {
                    ctx.draw(&Points {
                        coords: &[(self_pos.x as f64, self_pos.y as f64)],
                        color: draw_info.fg_color,
                    })
                }
            })
            .x_bounds(camera.x_bounds(frame_w))
            .y_bounds(camera.y_bounds(frame_h));

        frame.render_widget(self_as_widget, camera.main_layout().split(frame.size())[0]);
    }
}

#[derive(Clone, Copy)]
pub struct EnemyPositioned<'a, 'b> {
    enemy: &'a Enemy,
    trajectory: &'b dyn Trajectory,
}

impl<'a, 'b> EnemyPositioned<'a, 'b> {
    pub fn new(enemy: &'a Enemy, trajectory: &'b dyn Trajectory) -> Self {
        Self {
            enemy: enemy,
            trajectory: trajectory,
        }
    }
}

impl<'a, 'b> Positioned for EnemyPositioned<'a, 'b> {
    fn position(&self) -> Point {
        self.trajectory.point_from_t(self.enemy.t_position())
    }
}

impl<'a, 'b> HasPointDrawInfo for EnemyPositioned<'a, 'b> {
    fn draw_info(&self) -> &'static PointDrawInfo {
        &self.enemy.type_info().draw_info
    }
}

impl HasPointDrawInfo for Tower {
    fn draw_info(&self) -> &'static PointDrawInfo {
        &self.type_info().draw_info
    }
}

impl HasPointDrawInfo for Projectile {
    fn draw_info(&self) -> &'static PointDrawInfo {
        &PROJECTILE_DRAW_INFO
    }
}
