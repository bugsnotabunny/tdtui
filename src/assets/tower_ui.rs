use ratatui::{style::Color, symbols::Marker::Braille};

use crate::ui::circle_drawable::CircleDrawInfo;

pub const TOWER_GAP_DRAW_INFO: CircleDrawInfo = CircleDrawInfo {
    marker: Braille,
    fg_color: Color::Gray,
};

pub const TOWER_RADIUS_DRAW_INFO: CircleDrawInfo = CircleDrawInfo {
    marker: Braille,
    fg_color: Color::Gray,
};
