use std::{cell::RefCell, rc::Rc, time::Duration};

use super::{damage::Damage, enemy::Enemy, point::Point, road::Road, trajectory::Trajectory};

use rand::seq::IteratorRandom;

pub struct Aim {
    aim: Option<Rc<RefCell<Enemy>>>,
}

impl Aim {
    pub fn new(aim: Option<Rc<RefCell<Enemy>>>) -> Self {
        Self { aim: aim.clone() }
    }

    pub fn is_in_shoot_range(&self, tower: &Tower, trajectory: &dyn Trajectory) -> bool {
        match self.aim.as_ref() {
            Some(aimcell) => {
                let aim = aimcell.borrow();
                let enemypos = trajectory.get_point(aim.position());
                !aim.is_dead() && enemypos.distance(&tower.position()) < tower.range()
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
    position: Point,
}

impl Tower {
    pub fn new(damage: Damage, radius: f32, position: Point) -> Self {
        Self {
            damage: damage,
            range: radius,
            aim: Aim::new(None),
            position: position,
        }
    }

    pub fn position(&self) -> &Point {
        &self.position
    }

    pub fn range(&self) -> f32 {
        self.range
    }

    fn shoot(&mut self) {
        self.aim.try_shoot(self.damage.clone())
    }

    fn update_aim(&mut self, road: &dyn Road) {
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
                enemypos.distance(&self.position) < self.range
            })
            .map(|rc| rc.clone())
            .choose(&mut rand::thread_rng());

        self.aim = Aim::new(random_chosen_enemy)
    }

    pub fn on_update(&mut self, _: Duration, road: &dyn Road) {
        self.update_aim(road);
        self.shoot();
    }
}
