use std::{cell::RefCell, ops::Range, rc::Rc, vec::Vec};

use crate::{
    enemy::Enemy,
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
        for enemy in self.enemies.iter_mut() {
            enemy.borrow_mut().update();
        }
        self.spawn_new_enemy();
    }
}

use ratatui::{prelude::*, widgets::*};

pub struct RoadDrawable {
    points: Vec<(f64, f64)>,
    x_bounds: Range<f64>,
}

impl RoadDrawable {
    pub fn new<T: Trajectory, S: Spawner>(road: &Road<T, S>) -> RoadDrawable {
        let data = Vec::from_iter(
            (0..100)
                .map(|x| x as f32 * 1.0)
                .map(|t| road.trajectory().get_point(t))
                .map(|(x, y)| (x as f64, y as f64)),
        );

        let max_x = data.iter().max_by(|a, b| a.0.total_cmp(&b.0)).unwrap().0;
        let min_x = data.iter().min_by(|a, b| a.0.total_cmp(&b.0)).unwrap().0;

        let x_bounds = min_x..max_x;

        Self {
            points: data,
            x_bounds: x_bounds,
        }
    }
}

impl Drawable for RoadDrawable {
    fn draw(
        &self,
        frame: &mut ratatui::Frame<ratatui::prelude::CrosstermBackend<std::io::Stdout>>,
        camera: &Camera,
    ) {
        let datasets = vec![Dataset::default()
            .marker(symbols::Marker::Block)
            .style(Style::default().fg(Color::Green))
            .graph_type(GraphType::Line)
            .data(&self.points)];

        let range = (frame.size().width as f32 / 10.0).floor();

        let chart = Chart::new(datasets)
            // .block(Block::default().borders(Borders::ALL))
            .x_axis(Axis::default().bounds([
                camera.position().0 as f64,
                (camera.position().0 + range * camera.scale()) as f64,
            ]))
            .y_axis(
                Axis::default().bounds([-5.0 * camera.scale() as f64, 5.0 * camera.scale() as f64]),
            );

        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100), Constraint::Percentage(100)])
            .split(frame.size());

        frame.render_widget(chart, main_layout[0]);
    }
}
