use ratatui::{
    style::Stylize,
    widgets::{Block, Paragraph, Wrap},
    Frame,
};

use crate::model::wallet::Wallet;

use super::core::{Camera, Drawable};

impl Drawable for Wallet {
    fn draw(&self, frame: &mut Frame, camera: &Camera) {
        let text = format!("Balance: {}$", self.balance());
        let layout = camera.ui_layout().split(frame.size());

        let paragraph = Paragraph::new(text.dark_gray()).wrap(Wrap { trim: true });
        let block = Block::new();
        frame.render_widget(paragraph.block(block), layout[0]);
    }
}
