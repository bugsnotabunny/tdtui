use ratatui::{
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph, Wrap},
    Frame,
};

use crate::model::{core::GameModel, tower_selector::TowerSelector};

use super::core::{Camera, Drawable};

impl Drawable for TowerSelector {
    fn draw(&self, frame: &mut Frame, camera: &Camera, _: &dyn GameModel) {
        let stats = self.current();

        let text = vec![
            Line::from(format!("Type: {}", stats.name).dark_gray()),
            Line::from(format!("Cost: {}$", stats.cost).dark_gray()),
            Line::from(format!("Damage: {}", stats.damage).dark_gray()),
            Line::from(format!("Cooldown: {}ms", stats.cooldown.as_millis()).dark_gray()),
            Line::from(format!("Range: {}", stats.range).dark_gray()),
            Line::from(format!("Description: {}", stats.description).dark_gray()),
        ];

        let layout = camera.ui_layout().split(frame.size());

        let paragraph = Paragraph::new(text).wrap(Wrap { trim: true });
        let block = Block::new();
        frame.render_widget(paragraph.clone().block(block), layout[1]);
    }
}
