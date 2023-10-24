use std::{cell::RefCell, rc::Rc, vec::Vec};

use crate::{
    enemy::Enemy,
    game::GameModel,
    spawner::Spawner,
    trajectory::Trajectory,
    ui::{Camera, Drawable},
    update::Update,
};

const ROAD_LEN: f32 = 100.0;

pub struct Road<T: Trajectory, S: Spawner> {
    trajectory: T,
    spawner: S,
    enemies: Vec<Rc<RefCell<Enemy>>>,
}

impl<T: Trajectory, S: Spawner> Road<T, S> {
    pub fn new(trajectory: T, spawner: S) -> Self {
        Self {
            trajectory: trajectory,
            spawner: spawner,
            enemies: Vec::new(),
        }
    }

    pub fn trajectory(&self) -> &T {
        &self.trajectory
    }

    pub fn is_overrun(&self) -> bool {
        self.enemies
            .iter()
            .any(|rc| rc.borrow().position() > ROAD_LEN)
    }

    pub fn enemies(&self) -> &Vec<Rc<RefCell<Enemy>>> {
        &self.enemies
    }
}

impl<T: Trajectory, S: Spawner> Road<T, S> {
    fn spawn_new_enemy(&mut self) {
        let enemy = Rc::new(RefCell::new(self.spawner.spawn()));
        self.enemies.push(enemy);
    }
}

impl<T: Trajectory, S: Spawner> Update for Road<T, S> {
    fn update(&mut self) {
        self.enemies.retain(|enemy| !enemy.borrow().is_dead());

        for enemy in self.enemies.iter() {
            enemy.borrow_mut().move_forward(self.trajectory());
        }
        self.spawn_new_enemy();
    }
}

use ratatui::{prelude::*, widgets::*};

pub struct RoadDrawable {
    points: Vec<(f64, f64)>,
}

impl RoadDrawable {
    pub fn new<T: Trajectory, S: Spawner>(road: &Road<T, S>) -> RoadDrawable {
        let data = Vec::from_iter(
            (0..100)
                .map(|x| x as f32 * 1.0)
                .map(|t| road.trajectory().get_point(t))
                .map(|point| (point.x as f64, point.y as f64)),
        );

        Self { points: data }
    }
}

impl<T: Trajectory, S: Spawner> Drawable<T, S> for RoadDrawable {
    fn draw(
        &self,
        frame: &mut ratatui::Frame<ratatui::prelude::CrosstermBackend<std::io::Stdout>>,
        camera: &Camera,
        _: &GameModel<T, S>,
    ) {
        let datasets = vec![Dataset::default()
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Green))
            .graph_type(GraphType::Line)
            .data(&self.points)];

        let frame_w = frame.size().width;
        let frame_h = frame.size().height;

        let chart = Chart::new(datasets)
            .x_axis(Axis::default().bounds(camera.x_bounds(frame_w)))
            .y_axis(Axis::default().bounds(camera.y_bounds(frame_h)));

        frame.render_widget(chart, camera.main_layout().split(frame.size())[0]);
    }
}
