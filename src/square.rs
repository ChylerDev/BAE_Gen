//! # Square
//!
//! A square wave generator.

use super::*;

/// Struct for generating square wave samples at a specified frequency
pub struct Square {
    sample_rate: Math,
    ind: Math,
    inv: Math,
}

impl FreqMod for Square {
    fn new(f: Math, sample_rate: Math) -> Self {
        Square {
            sample_rate,
            ind: Default::default(),
            inv: Math(sample_rate.0 / (2.0 * f.0)),
        }
    }

    fn set_frequency(&mut self, f: Math) {
        self.inv.0 = self.sample_rate.0 / (2.0 * f.0);
    }

    fn get_frequency(&self) -> Math {
        Math(self.sample_rate.0 / (2.0 * self.inv.0))
    }
}

impl Generator for Square {
    fn process(&mut self) -> Sample {
        let y = if self.ind.0 >= self.inv.0 && self.ind.0 < 2.0 * self.inv.0 {
            -1.0
        } else {
            1.0
        };

        if self.ind.0 >= 2.0 * self.inv.0 {
            self.ind.0 -= 2.0 * self.inv.0;
        }

        self.ind.0 += 1.0;

        y
    }
}

impl BlockGenerator for Square {
    fn process_block(&mut self, x: &mut[Sample]) {
        for s in x {
            *s.0 = if self.ind.0 >= self.inv.0 && self.ind.0 < 2.0 * self.inv.0 {
                -1.0
            } else {
                1.0
            };

            if self.ind.0 >= 2.0 * self.inv.0 {
                self.ind.0 -= 2.0 * self.inv.0;
            }

            self.ind.0 += 1.0;
        }
    }
}

impl Clone for Square {
    fn clone(&self) -> Self {
        Square {
            sample_rate: self.sample_rate,
            ind: Default::default(),
            inv: self.inv,
        }
    }
}
