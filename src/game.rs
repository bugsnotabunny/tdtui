use crate::{
    road::{Road, RoadDrawable},
    spawner::Spawner,
    tower::Tower,
    trajectory::Trajectory,
    ui::Drawable,
    update::Update,
};

pub struct GameData<T: Trajectory, S: Spawner> {
    road: Road<T, S>,
    towers: Vec<Tower>,
}

impl<T: Trajectory, S: Spawner> GameData<T, S> {
    pub fn new(road: Road<T, S>) -> Self {
        Self {
            road: road,
            towers: Vec::new(),
        }
    }

    pub fn build(&mut self, tower: Tower) {
        self.towers.push(tower);
    }

    pub fn is_over(&self) -> bool {
        self.road.is_overrun()
    }
}

impl<T: Trajectory, S: Spawner> Update for GameData<T, S> {
    fn update(&mut self) {
        self.road.update();
        for tower in self.towers.iter_mut() {
            tower.update_aim(&self.road);
            tower.shoot();
        }
    }
}

use ratatui::{prelude::CrosstermBackend, Frame};
use std::io::Stdout;

impl<T: Trajectory, S: Spawner> Drawable for GameData<T, S> {
    fn draw(&self, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let road_drawable = RoadDrawable::new(&self.road);
        road_drawable.draw(frame);
        // self.towers().draw(frame);
    }
}
