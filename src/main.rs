mod damage;
mod enemy;
mod game;
mod input;
mod point;
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

use input::InputMask;
use noise::Perlin;
use point::Point;

use crate::{
    damage::Damage,
    game::GameModel,
    input::poll_events,
    road::Road,
    spawner::{BasicSpawner, Spawner},
    tower::Tower,
    trajectory::{NoiseTrajectory, Trajectory},
    ui::UI,
    update::Update,
};

const SCROLL: f32 = 1.0;
const SCALE_SCROLL: f32 = 0.1;

struct App<T: Trajectory, S: Spawner> {
    model: GameModel<T, S>,
    ui: UI,
    keep_going: bool,
}

impl<T: Trajectory, S: Spawner> App<T, S> {
    pub fn new(model: GameModel<T, S>, ui: UI) -> Self {
        Self {
            model,
            ui: ui,
            keep_going: true,
        }
    }

    pub fn run(&mut self, tick_duration: Duration) -> io::Result<()> {
        let run_res = self.run_impl(tick_duration);
        self.ui.kill()?;
        run_res?;
        Ok(())
    }

    fn run_impl(&mut self, tick_duration: Duration) -> io::Result<()> {
        self.ui.init()?;
        let mut last_update = Instant::now();
        while self.keep_going {
            let timeout = tick_duration.saturating_sub(last_update.elapsed());

            self.handle_events(poll_events(timeout)?);

            if last_update.elapsed() >= tick_duration {
                self.model.update();
                self.ui.draw(&self.model)?;
                last_update = Instant::now();
            }
            self.keep_going &= !self.model.is_over();
        }
        Ok(())
    }

    fn handle_events(&mut self, inputs: InputMask) {
        match inputs {
            InputMask::Quitted => self.keep_going = false,
            InputMask::RightPressed => {
                let mut pos = self.ui.camera().position();
                pos.0 += SCROLL;
                self.ui.camera_mut().set_position(pos);
            }
            InputMask::LeftPressed => {
                let mut pos = self.ui.camera().position();
                pos.0 -= SCROLL;
                self.ui.camera_mut().set_position(pos);
            }
            InputMask::UpPressed => {}
            InputMask::DownPressed => {}
            InputMask::ScaleDownPressed => {
                let scale = self.ui.camera().scale();
                if scale > SCALE_SCROLL {
                    self.ui.camera_mut().set_scale(scale - SCALE_SCROLL);
                }
            }
            InputMask::ScaleUpPressed => {
                let scale = self.ui.camera().scale();
                self.ui.camera_mut().set_scale(scale + SCALE_SCROLL);
            }
            _ => {}
        }
    }
}

fn main() -> io::Result<()> {
    let tick_duration = Duration::from_millis(100);

    let perlin = Perlin::new(10);
    let spawner = BasicSpawner::default();
    let trajectory = NoiseTrajectory::new(&perlin);
    let road = Road::new(trajectory, spawner);
    let mut model = GameModel::new(road);

    model.build(Tower::new(
        Damage {
            value: 1,
            kind: damage::DamageType::KINNETIC,
        },
        100.0,
        Point { x: 5.0, y: 5.0 },
    ));

    let ui = UI::new()?;

    let mut app = App::new(model, ui);
    app.run(tick_duration)?;
    Ok(())
}
