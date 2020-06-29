//! # Triangle
//!
//! A triangle wave generator.

use super::*;

/// Struct for generating triangle waves at a given frequency.
pub struct Triangle {
    sample_rate: Math,
    irate: Math,
    inc: Math,
}

impl FreqMod for Triangle {
    fn new(f: Math, sample_rate: Math) -> Self {
        Triangle {
            sample_rate,
            irate: Math(4.0 * f.0 / sample_rate.0),
            inc: Default::default(),
        }
    }

    fn set_frequency(&mut self, f: Math) {
        self.irate.0 = 4.0 * f.0 / self.sample_rate.0;
    }

    fn get_frequency(&self) -> Math {
        Math(self.irate.0 / (4.0 / self.sample_rate.0))
    }
}

impl Generator for Triangle {
    fn process(&mut self) -> Sample {
        let y = self.inc.0;

        self.inc.0 += self.irate.0;

        if self.inc.0 >= 1.0 || self.inc.0 <= -1.0 {
            self.irate.0 = -self.irate.0;

            self.inc.0 = if self.inc.0 >= 1.0 {
                2.0 - self.inc.0
            } else {
                -2.0 - self.inc.0
            };
        }

        Sample(y as FastMath)
    }
}

impl BlockGenerator for Triangle {
    fn process_block(&mut self, x: &mut[Sample]) {
        for s in x {
            (*s).0 = self.inc.0 as FastMath;

            self.inc.0 += self.irate.0;

            if self.inc.0 >= 1.0 || self.inc.0 <= -1.0 {
                self.irate.0 = -self.irate.0;
    
                self.inc.0 = if self.inc.0 >= 1.0 {
                    2.0 - self.inc.0
                } else {
                    -2.0 - self.inc.0
                };
            }
        }
    }
}

impl Clone for Triangle {
    fn clone(&self) -> Self {
        Triangle {
            sample_rate: self.sample_rate,
            irate: self.irate,
            inc: Default::default(),
        }
    }
}
