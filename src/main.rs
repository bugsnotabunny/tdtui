use std::{io, time::Duration};
pub mod input;
pub mod model;
pub mod ui;

use input::core::{poll_events, HandleEvents, InputMask};
use model::{
    clock::Clock,
    core::{ConcreteGameModel, GameModel},
    damage::{Damage, DamageType},
    point::Point,
    road::ConcreteRoad,
    spawner::SpawnerWithCooldown,
    tower::Tower,
    trajectory::NoiseTrajectory,
};
use ui::core::{Camera, Screen};

use noise::Perlin;

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
        let mut clock = Clock::from_now();
        while self.keep_going {
            let delta_time = clock.elapsed();
            let timeout = tick_duration.saturating_sub(delta_time);
            self.handle_events(poll_events(timeout)?);

            if delta_time >= tick_duration {
                self.game_model.update(delta_time);
                self.screen.draw_frame(&self.camera, &self.game_model)?;
                clock.tick();
            }

            self.keep_going &= !self.game_model.is_over();
        }
        Ok(())
    }

    fn handle_events(&mut self, events: InputMask) {
        match events {
            InputMask::Quitted => self.keep_going = false,
            _ => {}
        }

        self.camera.handle(events);
    }
}

fn main() -> io::Result<()> {
    let tick_duration = Duration::from_millis(100);

    let perlin = Perlin::new(10);
    let spawner = SpawnerWithCooldown::new(Duration::from_secs_f32(1.0));
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
