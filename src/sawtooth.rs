//! # Sawtooth
//!
//! A sawtooth sample generator.

use super::FreqMod;
use super::Generator;
use super::*;

/// Struct for generating sawtooth samples.
pub struct Sawtooth {
    r: Math,
    irate: Math,
    inc: Math,
}

impl FreqMod for Sawtooth {
    fn new(f: Math, sample_rate: Math) -> Self {
        Sawtooth {
            r: sample_rate,
            irate: Math(2.0 * f.0 / sample_rate.0),
            inc: Default::default(),
        }
    }

    fn set_frequency(&mut self, f: Math) {
        self.irate.0 = 2.0 * f.0 / self.r.0;
    }

    fn get_frequency(&self) -> Math {
        Math(self.irate.0 * self.r.0 / 2.0)
    }
}

impl Generator for Sawtooth {
    fn process(&mut self) -> Sample {
        let y = self.inc.0;

        self.inc.0 += self.irate.0;

        if self.inc.0 >= 1.0 {
            self.inc.0 -= 2.0;
        }

        Sample(y as FastMath)
    }
}

impl BlockGenerator for Sawtooth {
    fn process_block(&mut self, x: &mut[Sample]) {
        for s in x {
            (*s).0 = self.inc.0 as FastMath;

            self.inc.0 += self.irate.0;
            if self.inc.0 >= 1.0 {
                self.inc.0 -= 2.0;
            }
        }
    }
}

impl Clone for Sawtooth {
    fn clone(&self) -> Self {
        Sawtooth {
            r: self.r,
            irate: self.irate,
            inc: Default::default(),
        }
    }
}
