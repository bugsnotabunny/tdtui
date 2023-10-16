use std::{cell::RefCell, rc::Rc};

use rand::seq::IteratorRandom;

use crate::{damage::Damage, enemy::Enemy, road::Road, spawner::Spawner, trajectory::Trajectory};

pub fn distance(lhs: (f32, f32), rhs: (f32, f32)) -> f32 {
    ((lhs.0 - rhs.0).powi(2) + (lhs.1 - rhs.1).powi(2)).sqrt()
}

pub struct Aim {
    aim: Option<Rc<RefCell<Enemy>>>,
}

impl Aim {
    pub fn new(aim: Option<Rc<RefCell<Enemy>>>) -> Self {
        Self { aim: aim.clone() }
    }

    pub fn is_in_shoot_range(&self, tower: &Tower, trajectory: &impl Trajectory) -> bool {
        match self.aim.as_ref() {
            Some(aimcell) => {
                let aim = aimcell.borrow();
                let enemypos = trajectory.get_point(aim.position());
                !aim.is_dead() && distance(enemypos, tower.position()) < tower.range()
            }
            None => false,
        }
    }

    pub fn try_shoot(&mut self, damage: Damage) {
        if self.aim.is_none() {
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

pub struct Tower {
    damage: Damage,
    range: f32,
    aim: Aim,
    position: (f32, f32),
}

impl Tower {
    pub fn new(damage: Damage, radius: f32, position: (f32, f32)) -> Self {
        Self {
            damage: damage,
            range: radius,
            aim: Aim::new(None),
            position: position,
        }
    }

    pub fn position(&self) -> (f32, f32) {
        self.position
    }

    pub fn range(&self) -> f32 {
        self.range
    }

    pub fn shoot(&mut self) {
        self.aim.try_shoot(self.damage.clone())
    }

    pub fn update_aim(&mut self, road: &Road<impl Trajectory, impl Spawner>) {
        if !self.aim.is_in_shoot_range(self, road.trajectory()) {
            self.aim = Aim::new(None);
        }

        if self.aim.is_some() {
            return;
        }

        let random_chosen_enemy = road
            .enemies()
            .iter()
            .filter(|enemy| {
                let enemypos = road.trajectory().get_point(enemy.borrow().position());
                distance(enemypos, self.position) < self.range
            })
            .map(|rc| rc.clone())
            .choose(&mut rand::thread_rng());

        self.aim = Aim::new(random_chosen_enemy)
    }
}
