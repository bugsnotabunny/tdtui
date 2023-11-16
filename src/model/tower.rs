use crate::ui::core::Drawable;
use std::{cell::RefCell, rc::Rc, time::Duration};

use super::{
    core::UpdatableObject, damage::Damage, enemy::Enemy, point::Point, trajectory::Trajectory,
};

#[derive(Default)]
pub struct Aim {
    aim: Option<Rc<RefCell<dyn Enemy>>>,
}

impl Aim {
    pub fn new(aim: Option<Rc<RefCell<dyn Enemy>>>) -> Self {
        Self { aim: aim.clone() }
    }

    pub fn is_in_shoot_range(&self, tower: &impl Tower, trajectory: &dyn Trajectory) -> bool {
        match self.aim.as_ref() {
            Some(aimcell) => {
                let aim = aimcell.borrow();
                let enemypos = trajectory.get_point(aim.position());
                enemypos.distance(&tower.position()) < tower.range()
            }
            None => false,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.aim.is_some() && !self.aim.as_ref().unwrap().borrow().is_dead()
    }

    pub fn try_shoot(&mut self, damage: Damage, on_death: impl FnOnce(u64)) {
        if !self.is_some() {
            return;
        }
        let aim = self.aim.as_ref().unwrap();
        aim.borrow_mut().take_damage(damage);
        if aim.borrow().is_dead() {
            on_death(aim.borrow().reward());
            self.aim = None;
        }
    }

    pub fn is_some(&self) -> bool {
        self.aim.is_some()
    }
}

pub trait Tower: UpdatableObject + Drawable {
    fn position(&self) -> &Point;
    fn range(&self) -> f32;
    fn cost(&self) -> u64;
}

pub trait TowerStats {
    const COOLDOWN: Duration;
    const COST: u64;
    const RANGE: f32;
    const DAMAGE: Damage;
}
