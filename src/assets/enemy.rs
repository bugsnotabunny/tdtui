use crate::model::{damage::DamageType, enemy::EnemyInfo};

pub const BASIC_ENEMY_INFO: EnemyInfo = EnemyInfo {
    max_health: 20.0,
    speed: 1.0,
    reward: 3,
    damage_eval: |damage| damage.value,
    close_up_sprite: "",
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
    close_up_sprite: "󰟆",
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
    close_up_sprite: "󰐁",
};
