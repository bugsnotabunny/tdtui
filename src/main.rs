pub mod input;
pub mod model;
pub mod ui;

use std::{error::Error, io, time::Duration};

use input::core::{poll_event, HandleEvent, InputEvent, ScreenInfo};
use model::{
    clock::Clock,
    core::{ConcreteGameModel, GameModel},
    spawner::SpawnerWithCooldown,
    trajectory::NoiseTrajectory,
};
use rand::Rng;
use ui::core::{Camera, Screen};

use noise::Perlin;

struct App<G: GameModel + HandleEvent> {
    game_model: G,
    screen: Screen,
    camera: Camera,
    keep_going: bool,
    update_clock: Clock,
}

impl<G: GameModel + HandleEvent> App<G> {
    pub fn new(model: G, ui: Screen) -> Self {
        Self {
            game_model: model,
            screen: ui,
            camera: Camera::default(),
            update_clock: Clock::from_now(),
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
        while self.keep_going {
            while self.update_clock.elapsed() < tick_duration {
                let timeout = tick_duration.saturating_sub(self.update_clock.elapsed());
                let screen_info =
                    ScreenInfo::from_frame_size(self.camera.clone(), self.screen.size()?);
                let event = poll_event(timeout, screen_info)?;
                self.handle_event(event);
            }
            self.on_tick()?;
            self.keep_going &= !self.game_model.is_over();
        }
        Ok(())
    }

    fn on_tick(&mut self) -> io::Result<()> {
        let delta_time = self.update_clock.elapsed();
        self.game_model.update(delta_time);
        self.screen.draw_frame(&self.camera, &self.game_model)?;
        self.update_clock.tick();
        Ok(())
    }

    fn handle_event(&mut self, event: InputEvent) {
        match event {
            InputEvent::GameQuit => self.keep_going = false,
            _ => {}
        }
        let _ = self.camera.handle(event.clone());
        let _ = self.game_model.handle(event.clone());
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let target_fps = 60;
    let tick_duration = Duration::from_millis(1000) / target_fps;

    let perlin = Perlin::new(rand::thread_rng().gen());
    let spawner = SpawnerWithCooldown::new(Duration::from_secs_f32(1.0));
    let trajectory = NoiseTrajectory::new(&perlin);
    let model = ConcreteGameModel::new(spawner, trajectory, 1000);

    let ui = Screen::new()?;

    let mut app = App::new(model, ui);
    app.run(tick_duration)?;
    Ok(())
}
