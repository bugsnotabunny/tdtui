use std::{thread, time::Duration};

mod damage;
mod enemy;
mod road;
mod spawner;
mod tower;

mod update;

extern crate ncurses;

use tower::Tower;

use crate::{road::Road, spawner::BasicSpawner, update::Update};

struct Game {
    over: bool,
    tick_duration: Duration,
    road: Road,
}

impl Game {
    fn new(tick_duration: Duration, frame_duration: Duration) -> Self {
        Self {
            over: false,
            tick_duration: tick_duration,
            road: Road::new(20, 5, Box::new(BasicSpawner {})), // !TODO change constant to be a param
        }
    }

    fn run(&mut self) {
        self.road.build_tower(2, Tower::new());
        while !self.over {
            thread::sleep(self.tick_duration);
            self.update();
            self.over = self.road.is_overrun();
        }
    }
}

impl Update for Game {
    fn update(&mut self) {
        self.road.update();
    }
}

fn main() {
    let mut game = Game::new(Duration::from_millis(500), Duration::from_millis(50));
    game.run();
    println!("game over!");
}
