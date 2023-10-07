use std::{cell::RefCell, rc::Rc};

use rand::seq::IteratorRandom;

use crate::{damage::Damage, enemy::Enemy, road::Road};

pub fn distance(lhs: (f32, f32), rhs: (f32, f32)) -> f32 {
    ((lhs.0 - rhs.0).powi(2) + (lhs.1 - rhs.1).powi(2)).sqrt()
}

pub struct TowerPool {
    towers: Vec<Tower>,
}

impl TowerPool {
    pub fn new() -> Self {
        Self { towers: Vec::new() }
    }

    pub fn build_tower(&mut self, tower: Tower) {
        self.towers.push(tower);
    }

    pub fn choose_targets(&mut self, road: &Road) {
        for tower in self.towers.iter_mut() {
            if tower.is_prev_aim_in_radius(road) {
                continue;
            }

            let random_chosen_enemy = road
                .enemies()
                .iter()
                .filter(|enemy| {
                    let enemypos = road.trajectory().get_point(enemy.borrow().position());
                    distance(enemypos, tower.position()) < tower.radius()
                })
                .choose(&mut rand::thread_rng());

            match random_chosen_enemy {
                Some(enemy) => tower.set_aim(enemy.clone()),
                None => continue,
            }
        }
    }

    pub fn shoot_all(&mut self) {
        for tower in self.towers.iter_mut() {
            tower.shoot();
        }
    }
}

pub struct Tower {
    damage: Damage,
    radius: f32,
    aim: Rc<RefCell<Enemy>>,
    position: (f32, f32),
}

thread_local! {
    static BILLY: RefCell<Rc<RefCell<Enemy>>> = RefCell::new(Rc::new(RefCell::new( Enemy::new(u8::MIN, f32::MIN, f32::MIN))));
}

impl Tower {
    pub fn new(damage: Damage, radius: f32, position: (f32, f32)) -> Self {
        Self {
            damage: damage,
            radius: radius,
            aim: BILLY.with(|cell| cell.borrow_mut().clone()),
            position: position,
        }
    }

    pub fn is_prev_aim_in_radius(&self, road: &Road) -> bool {
        let enemy = self.aim();
        let enemypos = road.trajectory().get_point(enemy.borrow().position());
        !enemy.borrow().is_dead() && distance(enemypos, self.position()) < self.radius()
    }

    pub fn aim(&self) -> Rc<RefCell<Enemy>> {
        self.aim.clone()
    }

    pub fn position(&self) -> (f32, f32) {
        self.position
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn set_aim(&mut self, enemy: Rc<RefCell<Enemy>>) {
        self.aim = enemy
    }

    pub fn shoot(&mut self) {
        self.aim().borrow_mut().take_damage(self.damage.clone())
    }
}
