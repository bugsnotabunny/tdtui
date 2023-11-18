use super::point::Point;

use noise::NoiseFn;

pub trait Trajectory {
    fn point_from_t(&self, t: f32) -> Point;
}

pub struct NoiseTrajectory<Noise: NoiseFn<f64, 2>> {
    gen: Noise,
}

impl<Noise: NoiseFn<f64, 2>> NoiseTrajectory<Noise> {
    pub fn new(noise: Noise) -> Self {
        Self { gen: noise }
    }
}

impl<Noise: NoiseFn<f64, 2>> Trajectory for NoiseTrajectory<Noise> {
    fn point_from_t(&self, t: f32) -> Point {
        const INCREASE_WAVE_L: f64 = 20.0;
        const INCREASE_AMPLITUDE: f64 = 5.0;
        Point {
            x: t,
            y: (self.gen.get([t as f64 / INCREASE_WAVE_L, 0.0]) * INCREASE_AMPLITUDE) as f32,
        }
    }
}
