use ratatui::{
    style::{Color, Modifier, Stylize},
    symbols::Marker,
    widgets::canvas::{Canvas, Points},
    Frame,
};

use crate::{
    assets::tower::PROJECTILE_DRAW_INFO,
    model::{
        enemy::Enemy,
        point::{Point, Positioned},
        tower::{Projectile, Tower},
        trajectory::Trajectory,
    },
};

use super::core::{Camera, Drawable};

pub struct PosDrawInfo {
    pub marker: Marker,
    pub modifiers: Modifier,
    pub fg_color: Color,
    pub bg_color: Color,
    pub close_up_sprite: Option<&'static str>,
}

pub trait HasPosDrawInfo: Positioned {
    fn draw_info(&self) -> &'static PosDrawInfo;
}

pub struct PosDrawable<'a, T: HasPosDrawInfo> {
    instance: &'a T,
}

impl<'a, T: HasPosDrawInfo> PosDrawable<'a, T> {
    pub fn new(instance: &'a T) -> Self {
        Self { instance: instance }
    }
}

impl<'a, T: HasPosDrawInfo> Drawable for PosDrawable<'a, T> {
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

impl<'a, 'b> HasPosDrawInfo for EnemyPositioned<'a, 'b> {
    fn draw_info(&self) -> &'static PosDrawInfo {
        &self.enemy.type_info().draw_info
    }
}

impl HasPosDrawInfo for Tower {
    fn draw_info(&self) -> &'static PosDrawInfo {
        &self.type_info().draw_info
    }
}

impl HasPosDrawInfo for Projectile {
    fn draw_info(&self) -> &'static PosDrawInfo {
        &PROJECTILE_DRAW_INFO
    }
}
