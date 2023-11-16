use crate::ui::core::Drawable;

use crate::model::{core::UpdatableObject, damage::Damage};

pub trait EnemyCommon: UpdatableObject {
    fn position(&self) -> f32;
    fn is_dead(&self) -> bool;
    fn reward(&self) -> u64;
}

pub trait Enemy: EnemyCommon + Drawable {
    fn take_damage(&mut self, damage: Damage);
}
