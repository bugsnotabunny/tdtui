use std::{thread, time::Duration};

mod damage;
mod enemy;
mod road;
mod spawner;
mod tower;
mod update;

extern crate ncurses;

use damage::Damage;

use crate::{
    road::{Road, Trajectory},
    spawner::BasicSpawner,
    tower::{Tower, TowerPool},
    update::Update,
};

struct Game {
    over: bool,
    tick_duration: Duration,
    road: Road,
    towers: TowerPool,
}

impl Game {
    fn new(tick_duration: Duration) -> Self {
        Self {
            over: false,
            tick_duration: tick_duration,
            road: Road::new(
                Trajectory::new(|t| t.powi(2), |t| t.cos().powi(2)),
                Box::new(BasicSpawner {}),
            ),
            towers: TowerPool::new(),
        }
    }

    fn run(&mut self) {
        self.towers.build_tower(Tower::new(
            Damage {
                value: 100,
                kind: damage::DamageType::KINNETIC,
            },
            100.0,
            (5.0, 5.0),
        ));
        while !self.over {
            thread::sleep(self.tick_duration);
            self.update();
            self.over = self.road.is_overrun();
        }
    }
}

impl Update for Game {
    fn update(&mut self) {
        self.towers.choose_targets(&self.road);
        self.towers.shoot_all();
        self.road.update();
    }
}

fn main() {
    let mut game = Game::new(Duration::from_millis(1000));
    game.run();
    println!("game over!");
}
