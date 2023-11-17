pub mod app;
pub mod assets;
pub mod input;
pub mod model;
pub mod ui;

use std::{error::Error, time::Duration};

use app::App;
use model::{core::ConcreteGameModel, spawner::SpawnerWithCooldown, trajectory::NoiseTrajectory};
use rand::Rng;
use ui::core::Screen;

use noise::Perlin;

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
