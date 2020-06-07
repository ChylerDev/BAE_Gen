//! # Sawtooth
//!
//! A sawtooth sample generator.

use super::FreqMod;
use super::Generator;
use super::*;

/// Struct for generating sawtooth samples.
pub struct Sawtooth {
    r: MathT,
    irate: MathT,
    inc: MathT,
}

impl FreqMod for Sawtooth {
    fn new(f: MathT, sample_rate: MathT) -> Self {
        Sawtooth {
            r: sample_rate,
            irate: 2.0 * f / sample_rate,
            inc: 0.0,
        }
    }

    fn set_frequency(&mut self, f: MathT) {
        self.irate = 2.0 * f / self.r;
    }

    fn get_frequency(&self) -> MathT {
        self.irate * self.r / 2.0
    }
}

impl Generator for Sawtooth {
    fn process(&mut self) -> SampleT {
        let y = self.inc;

        self.inc += self.irate;

        if self.inc >= 1.0 {
            self.inc -= 2.0;
        }

        y as SampleT
    }
}

impl BlockGenerator for Sawtooth {
    fn process(&mut self, x: &mut[SampleT]) {
        for s in x {
            *s = self.inc as SampleT;

            self.inc += self.irate;
            if self.inc >= 1.0 {
                self.inc -= 2.0;
            }
        }
    }
}

impl Clone for Sawtooth {
    fn clone(&self) -> Self {
        Sawtooth {
            r: self.r,
            irate: self.irate,
            inc: 0.0,
        }
    }
}