use nice_plug::prelude::{Smoother, SmoothingStyle};

/// The plugin's DSP, apart from the host plumbing so tests can run it
/// offline. The plugin passes in parameter values once per block.
pub struct Engine {
    sample_rate: f32,
    gain: Smoother<f32>,
}

impl Engine {
    pub fn new(sample_rate: f32) -> Self {
        let gain = Smoother::new(SmoothingStyle::Logarithmic(50.0));
        gain.reset(1.0);
        Self { sample_rate, gain }
    }

    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    pub fn reset(&mut self, gain: f32) {
        self.gain.reset(gain);
    }

    pub fn set_gain(&mut self, gain: f32) {
        self.gain.set_target(self.sample_rate, gain);
    }

    pub fn process(&mut self, left: &mut [f32], right: &mut [f32]) {
        for (l, r) in left.iter_mut().zip(right.iter_mut()) {
            let gain = self.gain.next();
            *l *= gain;
            *r *= gain;
        }
    }
}
