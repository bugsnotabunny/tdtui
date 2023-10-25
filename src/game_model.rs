use crate::{road::Road, tower::Tower};

pub trait GameModel {
    fn update(&mut self);
    fn is_over(&self) -> bool;
    fn road(&self) -> &dyn Road;
    fn towers(&self) -> &Vec<Tower>;
}

pub struct ConcreteGameModel<R: Road> {
    road: R,
    towers: Vec<Tower>,
}

impl<R: Road> ConcreteGameModel<R> {
    pub fn new(road: R) -> Self {
        Self {
            road: road,
            towers: Vec::new(),
        }
    }

    pub fn build(&mut self, tower: Tower) {
        self.towers.push(tower);
    }
}

impl<R: Road> GameModel for ConcreteGameModel<R> {
    fn update(&mut self) {
        self.road.on_update();
        for tower in self.towers.iter_mut() {
            tower.on_update(&self.road);
        }
    }

    fn is_over(&self) -> bool {
        self.road.is_overrun()
    }

    fn road(&self) -> &dyn Road {
        &self.road
    }

    fn towers(&self) -> &Vec<Tower> {
        &self.towers
    }
}
