//! # Noise
//!
//! A white noise generator.

use super::*;
use rand;

#[derive(Default, Clone)]
/// Struct for generating white noise audio samples.
pub struct Noise {
    engine: rand::rngs::ThreadRng,
}

impl Noise {
    /// Creates a new white noise generator.
    pub fn new() -> Self {
        Noise {
            engine: rand::thread_rng(),
        }
    }
}

impl Generator for Noise {
    fn process(&mut self) -> Sample {
        use rand::Rng;

        self.engine.gen::<Sample>() * 2.0 - 1.0
    }
}

impl BlockGenerator for Noise {
    fn process_block(&mut self, x: &mut[Sample]) {
        use rand::Rng;

        for s in x {
            *s = self.engine.gen::<Sample>() * 2.0 - 1.0;
        }
    }
}
