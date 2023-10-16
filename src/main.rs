mod damage;
mod enemy;
mod game;
mod road;
mod spawner;
mod tower;
mod trajectory;
mod ui;
mod update;

use std::{io, thread, time::Duration};

use noise::Perlin;
use road::Road;
use spawner::BasicSpawner;
use trajectory::NoiseTrajectory;

use crate::{damage::Damage, game::Game, tower::Tower, ui::UI, update::Update};

fn main() -> io::Result<()> {
    let tick_duration = Duration::from_millis(1000);

    let perlin = Perlin::new(10);
    let spawner = BasicSpawner::default();
    let trajectory = NoiseTrajectory::new(&perlin);
    let road = Road::new(trajectory, spawner);

    let mut game = Game::new(road);
    let mut ui = UI::new()?;

    game.build(Tower::new(
        Damage {
            value: 100,
            kind: damage::DamageType::KINNETIC,
        },
        100.0,
        (5.0, 5.0),
    ));

    while !game.is_over() {
        game.update();
        ui.draw(&game);
        thread::sleep(tick_duration);
    }
    ui.kill()?;
    Ok(())
}
