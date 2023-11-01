use std::{cell::RefCell, rc::Rc, time::Duration};

use super::{
    clock::Clock,
    damage::{Damage, DamageType},
    enemy::Enemy,
    point::Point,
    road::Road,
    trajectory::Trajectory,
};

use rand::seq::IteratorRandom;

pub struct Aim {
    aim: Option<Rc<RefCell<Enemy>>>,
}

impl Aim {
    pub fn new(aim: Option<Rc<RefCell<Enemy>>>) -> Self {
        Self { aim: aim.clone() }
    }

    pub fn is_in_shoot_range(&self, tower: &dyn Tower, trajectory: &dyn Trajectory) -> bool {
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

pub trait Tower {
    fn position(&self) -> &Point;
    fn range(&self) -> f32;
    fn on_update(&mut self, road: &dyn Road);
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

    fn on_update(&mut self, road: &dyn Road) {
        self.update_aim(road);
        if self.cooldown_clock.elapsed() > Self::COOLDOWN {
            self.shoot();
            self.cooldown_clock.tick();
        }
    }
}
