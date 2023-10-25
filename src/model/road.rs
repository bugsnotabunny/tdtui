use std::{cell::RefCell, rc::Rc, time::Duration, vec::Vec};

use super::{enemy::Enemy, spawner::Spawner, trajectory::Trajectory};

const ROAD_LEN: f32 = 100.0;

pub trait Road {
    fn on_update(&mut self, delta_time: Duration);
    fn is_overrun(&self) -> bool;
    fn trajectory(&self) -> &dyn Trajectory;
    fn enemies(&self) -> &Vec<Rc<RefCell<Enemy>>>;
}

pub struct ConcreteRoad<T: Trajectory, S: Spawner> {
    trajectory: T,
    spawner: S,
    enemies: Vec<Rc<RefCell<Enemy>>>,
}

impl<T: Trajectory, S: Spawner> ConcreteRoad<T, S> {
    pub fn new(trajectory: T, spawner: S) -> Self {
        Self {
            trajectory: trajectory,
            spawner: spawner,
            enemies: Vec::new(),
        }
    }
}

impl<T: Trajectory, S: Spawner> Road for ConcreteRoad<T, S> {
    fn on_update(&mut self, delta_time: Duration) {
        self.enemies.retain(|enemy| !enemy.borrow().is_dead());
        for enemy in self.enemies.iter_mut() {
            enemy.borrow_mut().on_update(delta_time, &self.trajectory);
        }

        self.spawn_new_enemy();
    }

    fn trajectory(&self) -> &dyn Trajectory {
        &self.trajectory
    }

    fn is_overrun(&self) -> bool {
        self.enemies
            .iter()
            .any(|rc| rc.borrow().position() > ROAD_LEN)
    }

    fn enemies(&self) -> &Vec<Rc<RefCell<Enemy>>> {
        &self.enemies
    }
}

impl<T: Trajectory, S: Spawner> ConcreteRoad<T, S> {
    fn spawn_new_enemy(&mut self) {
        let enemy = Rc::new(RefCell::new(self.spawner.spawn()));
        self.enemies.push(enemy);
    }
}
