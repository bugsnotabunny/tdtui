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

use crate::{damage::Damage, game::GameData, tower::Tower, ui::UI, update::Update};

fn main() -> io::Result<()> {
    let tick_duration = Duration::from_millis(1000);

    let perlin = Perlin::new(10);
    let spawner = BasicSpawner::default();
    let trajectory = NoiseTrajectory::new(&perlin);
    let road = Road::new(trajectory, spawner);
    let mut data = GameData::new(road);
    let mut ui = UI::new()?;

    data.build(Tower::new(
        Damage {
            value: 100,
            kind: damage::DamageType::KINNETIC,
        },
        100.0,
        (5.0, 5.0),
    ));

    while !data.is_over() {
        data.update();
        ui.draw(&data);
        thread::sleep(tick_duration);
    }
    ui.kill()?;
    Ok(())
}
