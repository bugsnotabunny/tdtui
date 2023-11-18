use std::time::Duration;

use ratatui::{
    style::{Color, Modifier},
    symbols::Marker,
};

use crate::{
    model::{
        damage::{Damage, DamageType},
        tower::{ProjectileInfo, TowerInfo},
    },
    ui::pos_drawable::PosDrawInfo,
};

pub const PROJECTILE_DRAW_INFO: PosDrawInfo = PosDrawInfo {
    close_up_sprite: None,
    marker: Marker::Braille,
    modifiers: Modifier::empty(),
    fg_color: Color::Blue,
    bg_color: Color::Reset,
};

pub const COMMON_TOWER_MARKER: Marker = Marker::Dot;
pub const COMMON_TOWER_MODIFIERS: Modifier = Modifier::BOLD;
pub const COMMON_TOWER_FG_COLOR: Color = Color::Green;
pub const COMMON_TOWER_BG_COLOR: Color = Color::Reset;

pub const ARCHER_TOWER_INFO: TowerInfo = TowerInfo {
    cooldown: Duration::from_millis(1500),
    cost: 10,
    range: 50.0,
    name: "Archer tower",
    description: "",
    projectile_info: ProjectileInfo {
        speed: 100.0,
        damage: Damage {
            value: 10.0,
            kind: DamageType::Kinnetic,
        },
    },
    draw_info: PosDrawInfo {
        close_up_sprite: Some(""),
        marker: COMMON_TOWER_MARKER,
        modifiers: COMMON_TOWER_MODIFIERS,
        fg_color: COMMON_TOWER_FG_COLOR,
        bg_color: COMMON_TOWER_BG_COLOR,
    },
};

pub const MAGE_TOWER_INFO: TowerInfo = TowerInfo {
    cooldown: Duration::from_millis(2000),
    cost: 10,
    range: 100.0,
    name: "Mage tower",
    description: "",
    projectile_info: ProjectileInfo {
        speed: 50.0,
        damage: Damage {
            value: 5.0,
            kind: DamageType::Magic,
        },
    },
    draw_info: PosDrawInfo {
        close_up_sprite: Some(""),
        marker: COMMON_TOWER_MARKER,
        modifiers: COMMON_TOWER_MODIFIERS,
        fg_color: COMMON_TOWER_FG_COLOR,
        bg_color: COMMON_TOWER_BG_COLOR,
    },
};
