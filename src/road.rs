use std::{cell::RefCell, rc::Rc, vec::Vec};

use crate::{enemy::Enemy, spawner::Spawner, update::Update};

const ROAD_LEN: f32 = 100.0;

pub struct Road {
    trajectory: Trajectory,
    spawner: Box<dyn Spawner>,
    enemies: Vec<Rc<RefCell<Enemy>>>,
}

impl Road {
    pub fn new(trajectory: Trajectory, spawner: Box<dyn Spawner>) -> Self {
        Self {
            trajectory: trajectory,
            spawner: spawner,
            enemies: Vec::new(),
        }
    }

    pub fn trajectory(&self) -> &Trajectory {
        &self.trajectory
    }

    pub fn is_overrun(&self) -> bool {
        self.enemies
            .iter()
            .any(|rc| rc.borrow().position() > ROAD_LEN)
    }

    pub fn enemies(&self) -> &Vec<Rc<RefCell<Enemy>>> {
        &self.enemies
    }

    // pub fn enemies_mut(&mut self) -> &mut Vec<Rc<RefCell<Enemy>>> {
    //     &mut self.enemies
    // }
}

impl Update for Road {
    fn update(&mut self) {
        self.enemies.retain(|enemy| !enemy.borrow().is_dead());
        for enemy in self.enemies.iter_mut() {
            enemy.borrow_mut().update();
        }
        self.spawn_new_enemy()
    }
}

impl Road {
    fn spawn_new_enemy(&mut self) {
        self.enemies
            .push(Rc::new(RefCell::new(self.spawner.spawn())))
    }
}

pub struct Trajectory {
    xf: fn(f32) -> f32,
    yf: fn(f32) -> f32,
}

impl Trajectory {
    pub fn new(xf: fn(f32) -> f32, yf: fn(f32) -> f32) -> Self {
        Self { xf: xf, yf: yf }
    }

    pub fn get_point(&self, t: f32) -> (f32, f32) {
        ((self.xf)(t), (self.yf)(t))
    }
}
