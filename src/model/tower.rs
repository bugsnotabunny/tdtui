use std::{cell::RefCell, rc::Rc, time::Duration};

use crate::ui::core::Drawable;

use super::{
    clock::Clock,
    core::{GameModel, UpdatableObject},
    damage::{Damage, DamageType},
    enemy::Enemy,
    point::Point,
    trajectory::Trajectory,
};

use rand::seq::IteratorRandom;

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

    pub fn try_shoot(&mut self, damage: Damage) {
        if !self.is_some() {
            return;
        }
        let aim = self.aim.as_ref().unwrap();
        aim.borrow_mut().take_damage(damage);
        if aim.borrow().is_dead() {
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

pub struct ArcherTower {
    aim: Aim,
    position: Point,
    cooldown_clock: Clock,
}

impl ArcherTower {
    const RANGE: f32 = 100.0;
    const COST: u64 = 10;
    const COOLDOWN: Duration = Duration::from_millis(1500);
    const DAMAGE: Damage = Damage {
        value: 10,
        kind: DamageType::KINNETIC,
    };

    pub fn new(position: Point) -> Self {
        Self {
            aim: Aim::new(None),
            position: position,
            cooldown_clock: Clock::from_now(),
        }
    }

    fn shoot(&mut self) {
        self.aim.try_shoot(Self::DAMAGE.clone())
    }

    fn update_aim(&mut self, game_model: &dyn GameModel) {
        if !self.aim.is_in_shoot_range(self, game_model.trajectory()) {
            self.aim = Aim::new(None);
        }

        if self.aim.is_some() {
            return;
        }

        let random_chosen_enemy = game_model
            .enemies()
            .iter()
            .filter(|enemy| {
                let enemypos = game_model.trajectory().get_point(enemy.borrow().position());
                enemypos.distance(&self.position) < self.range()
            })
            .map(|rc| rc.clone())
            .choose(&mut rand::thread_rng());

        self.aim = Aim::new(random_chosen_enemy)
    }
}

impl Tower for ArcherTower {
    fn position(&self) -> &Point {
        &self.position
    }

    fn range(&self) -> f32 {
        Self::RANGE
    }

    fn cost(&self) -> u64 {
        Self::COST
    }
}

impl UpdatableObject for ArcherTower {
    fn on_update(&mut self, game_model: &dyn GameModel, _: Duration) {
        self.update_aim(game_model);
        if self.cooldown_clock.elapsed() > Self::COOLDOWN {
            self.shoot();
            self.cooldown_clock.tick();
        }
    }
}
