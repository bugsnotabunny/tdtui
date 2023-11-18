use ratatui::{
    style::{Color, Style},
    symbols,
    widgets::{Axis, Chart, Dataset, GraphType},
    Frame,
};

use crate::model::trajectory::Trajectory;

use super::core::{Camera, Drawable};

pub struct RoadDrawable {
    points: Vec<(f64, f64)>,
}

impl RoadDrawable {
    pub fn new(trajectory: &dyn Trajectory) -> RoadDrawable {
        let data = Vec::from_iter(
            (0..100)
                .map(|x| x as f32 * 1.0)
                .map(|t| trajectory.point_from_t(t))
                .map(|point| (point.x as f64, point.y as f64)),
        );

        Self { points: data }
    }
}

impl Drawable for RoadDrawable {
    fn draw(&self, frame: &mut Frame, camera: &Camera) {
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
