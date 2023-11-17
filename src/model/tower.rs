use crate::ui::core::Drawable;
use std::{cell::RefCell, rc::Rc, time::Duration};

use super::{
    clock::Clock,
    core::{GameModel, UpdatableObject},
    damage::{Damage, DamageType},
    enemy::Enemy,
    point::Point,
    trajectory::Trajectory,
};

use macros::*;

use rand::seq::IteratorRandom;

#[derive(Default)]
struct Aim {
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

#[derive(Tower)]
pub struct ArcherTower {
    aim: Aim,
    position: Point,
    cooldown_clock: Clock,
}

impl TowerStats for ArcherTower {
    const COOLDOWN: Duration = Duration::from_millis(1500);
    const COST: u64 = 10;
    const RANGE: f32 = 50.0;
    const DAMAGE: Damage = Damage {
        value: 10.0,
        kind: DamageType::Kinnetic,
    };
}

#[derive(Tower)]
pub struct MageTower {
    aim: Aim,
    position: Point,
    cooldown_clock: Clock,
}

impl TowerStats for MageTower {
    const COOLDOWN: Duration = Duration::from_millis(2000);
    const COST: u64 = 20;
    const RANGE: f32 = 100.0;
    const DAMAGE: Damage = Damage {
        value: 5.0,
        kind: DamageType::Magic,
    };
}
