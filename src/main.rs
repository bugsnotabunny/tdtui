pub mod app;
pub mod assets;
pub mod input;
pub mod model;
pub mod ui;

use std::{error::Error, time::Duration};

use app::App;
use model::{
    core::ConcreteGameModel, point::Point, spawner::RandomizedSpawnerWithCooldown,
    trajectory::NoiseTrajectory,
};
use rand::Rng;
use ui::core::{Camera, Screen};

use noise::Perlin;

fn main() -> Result<(), Box<dyn Error>> {
    let target_fps = 60;
    let tick_duration = Duration::from_millis(1000) / target_fps;

    let perlin = Perlin::new(rand::thread_rng().gen());
    let spawner = RandomizedSpawnerWithCooldown::new(Duration::from_secs_f32(1.0));
    let trajectory = NoiseTrajectory::new(&perlin);
    let model = ConcreteGameModel::new(spawner, trajectory, 1000, 2.0);

    let camera = Camera::new(Point::default(), 0.3, 1.0);
    let ui = Screen::new()?;

    let mut app = App::new(model, ui, camera);
    app.run(tick_duration)?;
    Ok(())
}
