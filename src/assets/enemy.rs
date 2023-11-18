use ratatui::{
    style::{Color, Modifier},
    symbols::Marker,
};

use crate::{
    model::{damage::DamageType, enemy::EnemyInfo},
    ui::pos_drawable::PosDrawInfo,
};

pub const COMMON_ENEMY_MARKER: Marker = Marker::HalfBlock;
pub const COMMON_ENEMY_MODIFIERS: Modifier = Modifier::BOLD;
pub const COMMON_ENEMY_FG_COLOR: Color = Color::Red;
pub const COMMON_ENEMY_BG_COLOR: Color = Color::Reset;

pub const BASIC_ENEMY_INFO: EnemyInfo = EnemyInfo {
    max_health: 20.0,
    speed: 1.0,
    reward: 3,
    damage_eval: |damage| damage.value,
    draw_info: PosDrawInfo {
        close_up_sprite: Some(""),
        marker: COMMON_ENEMY_MARKER,
        modifiers: COMMON_ENEMY_MODIFIERS,
        fg_color: COMMON_ENEMY_FG_COLOR,
        bg_color: COMMON_ENEMY_BG_COLOR,
    },
};

pub const KINNETIC_RESIST_ENEMY_INFO: EnemyInfo = EnemyInfo {
    max_health: 20.0,
    speed: 1.0,
    reward: 4,
    damage_eval: |damage| {
        let coef = match damage.kind {
            DamageType::Kinnetic => 0.3,
            _ => 1.0,
        };
        damage.value * coef
    },
    draw_info: PosDrawInfo {
        close_up_sprite: Some("󰟆"),
        marker: COMMON_ENEMY_MARKER,
        modifiers: COMMON_ENEMY_MODIFIERS,
        fg_color: COMMON_ENEMY_FG_COLOR,
        bg_color: COMMON_ENEMY_BG_COLOR,
    },
};

pub const MAGIC_RESIST_ENEMY_INFO: EnemyInfo = EnemyInfo {
    max_health: 20.0,
    speed: 1.0,
    reward: 4,
    damage_eval: |damage| {
        let coef = match damage.kind {
            DamageType::Magic => 0.6,
            _ => 1.0,
        };
        damage.value * coef
    },
    draw_info: PosDrawInfo {
        close_up_sprite: Some("󰐁"),
        marker: COMMON_ENEMY_MARKER,
        modifiers: COMMON_ENEMY_MODIFIERS,
        fg_color: COMMON_ENEMY_FG_COLOR,
        bg_color: COMMON_ENEMY_BG_COLOR,
    },
};
