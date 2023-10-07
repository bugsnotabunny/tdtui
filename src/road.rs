use std::{cell::RefCell, rc::Rc, vec::Vec};

use rand::seq::IteratorRandom;

use crate::{enemy::Enemy, spawner::Spawner, tower::Tower, update::Update};

pub struct Road {
    len: u16,
    tower_step: u16,
    spawner: Box<dyn Spawner>,
    enemies: Vec<Rc<RefCell<(u16, Enemy)>>>,
    towers: Vec<(u16, Tower)>,
}

impl Road {
    pub fn new(len: u16, tower_step: u16, spawner: Box<dyn Spawner>) -> Self {
        Self {
            len: len,
            tower_step: tower_step,
            enemies: Vec::new(),
            towers: Vec::with_capacity((len / tower_step - 1) as usize),
            spawner: spawner,
        }
    }

    pub fn size(&self) -> usize {
        self.enemies.len()
    }

    pub fn is_overrun(&self) -> bool {
        self.enemies.iter().any(|rc| rc.borrow().0 > self.len)
    }

    pub fn build_tower(&mut self, idx: u16, tower: Tower) {
        assert!(idx < self.towers.capacity() as u16);
        self.towers.push(((idx + 1) * self.tower_step, tower))
    }
}

impl Update for Road {
    fn update(&mut self) {
        self.set_towers_aims();
        self.towers_shoot();
        self.enemies.retain(|enemy| !enemy.borrow().1.is_dead());
        self.move_enemies();
        self.spawn_new_enemy()
    }
}

impl Road {
    fn move_enemies(&mut self) {
        for rc in self.enemies.iter_mut() {
            let mut enemy = rc.borrow_mut();
            enemy.0 += enemy.1.speed() as u16;
        }
    }

    fn spawn_new_enemy(&mut self) {
        self.enemies
            .push(Rc::new(RefCell::new((0, self.spawner.spawn()))))
    }

    fn set_towers_aims(&mut self) {
        for (towerpos, tower) in self.towers.iter_mut() {
            {
                let enemy = tower.aim();
                if !enemy.borrow().1.is_dead()
                    && enemy.borrow().0.abs_diff(*towerpos) < tower.radius() as u16
                {
                    continue;
                }
            }

            let random_chosen_enemy = self
                .enemies
                .iter_mut()
                .filter(|enemy| enemy.borrow().0.abs_diff(*towerpos) < tower.radius() as u16)
                .choose(&mut rand::thread_rng());

            match random_chosen_enemy {
                Some(enemy) => tower.set_aim(enemy.clone()),
                None => {}
            }
        }
    }

    fn towers_shoot(&mut self) {
        for (_, tower) in self.towers.iter_mut() {
            tower.shoot();
        }
    }
}
