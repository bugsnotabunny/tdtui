mod damage;
mod enemy;
mod game;
mod road;
mod spawner;
mod tower;
mod trajectory;
mod ui;
mod update;

use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode};
use noise::Perlin;
use road::Road;
use spawner::BasicSpawner;
use trajectory::NoiseTrajectory;

use crate::{damage::Damage, game::Game, tower::Tower, ui::UI, update::Update};

const SCROLL: f32 = 1.0;
const SCALE_SCROLL: f32 = 0.1;

fn main() -> io::Result<()> {
    let tick_duration = Duration::from_millis(100);

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

    let mut last_update = Instant::now();
    while !game.is_over() {
        let timeout = tick_duration.saturating_sub(last_update.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('d') => {
                        let mut pos = ui.camera().position();
                        pos.0 += SCROLL;
                        ui.camera_mut().set_position(pos);
                    }
                    KeyCode::Char('a') => {
                        let mut pos = ui.camera().position();
                        pos.0 -= SCROLL;
                        ui.camera_mut().set_position(pos);
                    }
                    KeyCode::Char('w') => {
                        let scale = ui.camera().scale();
                        if scale > SCALE_SCROLL {
                            ui.camera_mut().set_scale(scale - SCALE_SCROLL);
                        }
                    }
                    KeyCode::Char('s') => {
                        let scale = ui.camera().scale();
                        ui.camera_mut().set_scale(scale + SCALE_SCROLL);
                    }
                    _ => {}
                }
            }
        }

        if last_update.elapsed() >= tick_duration {
            game.update();
            ui.draw(&game)?;
            last_update = Instant::now();
        }
    }
    ui.kill()?;
    Ok(())
}
