use ratatui::{
    style::{Color, Stylize},
    widgets::canvas::{Canvas, Points},
    Frame,
};

use crate::model::tower::{Projectile, Tower};

use super::core::{Camera, Drawable};

pub struct TowerDrawable<'a> {
    tower: &'a Tower,
}

impl<'a> TowerDrawable<'a> {
    pub fn new(tower: &'a Tower) -> Self {
        Self { tower: tower }
    }
}

impl<'a> Drawable for TowerDrawable<'a> {
    fn draw(
        &self,
        frame: &mut Frame,
        camera: &Camera,
        _game_model: &dyn crate::model::core::GameModel,
    ) {
        let frame_w = frame.size().width;
        let frame_h = frame.size().height;
        let self_pos = self.tower.position();
        let close_up = self.tower.type_info().close_up_sprite;

        let self_as_widget = Canvas::default()
            .marker(ratatui::symbols::Marker::Dot)
            .paint(|ctx| {
                if camera.allows_more_detail() {
                    ctx.print(
                        self_pos.x as f64,
                        self_pos.y as f64,
                        close_up.green().bold(),
                    );
                } else {
                    ctx.draw(&Points {
                        coords: &[(self_pos.x as f64, self_pos.y as f64)],
                        color: Color::Green,
                    })
                }
            })
            .x_bounds(camera.x_bounds(frame_w))
            .y_bounds(camera.y_bounds(frame_h));

        frame.render_widget(self_as_widget, camera.main_layout().split(frame.size())[0]);
    }
}

pub struct ProjectileDrawable<'a> {
    projectile: &'a Projectile,
}

impl<'a> ProjectileDrawable<'a> {
    pub fn new(projectile: &'a Projectile) -> Self {
        Self {
            projectile: projectile,
        }
    }
}

impl<'a> Drawable for ProjectileDrawable<'a> {
    fn draw(
        &self,
        frame: &mut Frame,
        camera: &Camera,
        _game_model: &dyn crate::model::core::GameModel,
    ) {
        let frame_w = frame.size().width;
        let frame_h = frame.size().height;
        let self_pos = self.projectile.position();

        let self_as_widget = Canvas::default()
            .marker(ratatui::symbols::Marker::Braille)
            .paint(|ctx| {
                ctx.draw(&Points {
                    coords: &[(self_pos.x as f64, self_pos.y as f64)],
                    color: Color::Blue,
                })
            })
            .x_bounds(camera.x_bounds(frame_w))
            .y_bounds(camera.y_bounds(frame_h));

        frame.render_widget(self_as_widget, camera.main_layout().split(frame.size())[0]);
    }
}
