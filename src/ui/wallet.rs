use ratatui::{
    prelude::{Constraint, Direction, Layout},
    style::Stylize,
    widgets::{Block, Paragraph, Wrap},
    Frame,
};

use crate::model::{core::GameModel, wallet::Wallet};

use super::core::{Camera, Drawable};

impl Drawable for Wallet {
    fn draw(&self, frame: &mut Frame, _: &Camera, _: &dyn GameModel) {
        let text = format!("Balance: {}$", self.balance());
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(2), Constraint::Min(1)])
            .split(frame.size());

        let paragraph = Paragraph::new(text.dark_gray()).wrap(Wrap { trim: true });
        let block = Block::new();
        frame.render_widget(paragraph.clone().block(block), layout[0]);
    }
}
