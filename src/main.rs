use std::{
    io,
    time::{Duration, Instant},
};
pub mod input;
pub mod model;
pub mod ui;

use input::{poll_events, InputMask};
use model::{
    core::{ConcreteGameModel, GameModel},
    damage::{Damage, DamageType},
    point::Point,
    road::ConcreteRoad,
    spawner::BasicSpawner,
    tower::Tower,
    trajectory::NoiseTrajectory,
};
use ui::core::{Camera, Screen};

use noise::Perlin;

const SCROLL: f32 = 1.0;
const SCALE_SCROLL: f32 = 0.1;

struct App<G: GameModel> {
    game_model: G,
    screen: Screen,
    camera: Camera,
    keep_going: bool,
}

impl<G: GameModel> App<G> {
    pub fn new(model: G, ui: Screen) -> Self {
        Self {
            game_model: model,
            screen: ui,
            camera: Camera::default(),
            keep_going: true,
        }
    }

    pub fn run(&mut self, tick_duration: Duration) -> io::Result<()> {
        self.screen.init()?;
        let run_res = self.run_impl(tick_duration);
        self.screen.kill()?;
        run_res?;
        Ok(())
    }

    fn run_impl(&mut self, tick_duration: Duration) -> io::Result<()> {
        let mut last_update = Instant::now();
        while self.keep_going {
            let timeout = tick_duration.saturating_sub(last_update.elapsed());

            self.handle_events(poll_events(timeout)?);

            if last_update.elapsed() >= tick_duration {
                self.game_model.update();
                self.screen.draw_frame(&self.camera, &self.game_model)?;
                last_update = Instant::now();
            }
            self.keep_going &= !self.game_model.is_over();
        }
        Ok(())
    }

    fn handle_events(&mut self, inputs: InputMask) {
        match inputs {
            InputMask::Quitted => self.keep_going = false,
            InputMask::RightPressed => {
                let mut pos = self.camera.position();
                pos.0 += SCROLL;
                self.camera.set_position(pos);
            }
            InputMask::LeftPressed => {
                let mut pos = self.camera.position();
                pos.0 -= SCROLL;
                self.camera.set_position(pos);
            }
            InputMask::UpPressed => {}
            InputMask::DownPressed => {}
            InputMask::ScaleDownPressed => {
                let scale = self.camera.scale();
                if scale > SCALE_SCROLL {
                    self.camera.set_scale(scale - SCALE_SCROLL);
                }
            }
            InputMask::ScaleUpPressed => {
                let scale = self.camera.scale();
                self.camera.set_scale(scale + SCALE_SCROLL);
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
    let road = ConcreteRoad::new(trajectory, spawner);
    let mut model = ConcreteGameModel::new(road);

    model.build(Tower::new(
        Damage {
            value: 1,
            kind: DamageType::KINNETIC,
        },
        100.0,
        Point { x: 5.0, y: 5.0 },
    ));

    let ui = Screen::new()?;

    let mut app = App::new(model, ui);
    app.run(tick_duration)?;
    Ok(())
}
