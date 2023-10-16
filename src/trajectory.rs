use noise::NoiseFn;

pub trait Trajectory {
    fn get_point(&self, t: f32) -> (f32, f32);
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
    fn get_point(&self, t: f32) -> (f32, f32) {
        const INCREASE_WAVE_L: f64 = 20.0;
        const INCREASE_AMPLITUDE: f64 = 5.0;
        (
            t,
            (self.gen.get([t as f64 / INCREASE_WAVE_L, 0.0]) * INCREASE_AMPLITUDE) as f32,
        )
    }
}
