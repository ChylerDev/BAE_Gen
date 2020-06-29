//! # Sine
//!
//! A sinusoidal sample generator.

use super::*;
use lazy_static::lazy_static;

/// The number of elements in the wavetable.
const WAVETABLE_SIZE: usize = 1024 * 1024; // 1024^2 = 1_048_576

lazy_static! {
    /// Lazy static initialization of the static WAVETABLE object.
    static ref WAVETABLE: Vec<AccurateMath> = {
        let mut wt = Vec::new();
        for i in 0..WAVETABLE_SIZE {
            wt.push((2.0*std::f64::consts::PI*(i as AccurateMath)/(WAVETABLE_SIZE as AccurateMath)).sin());
        }
        wt
    };
}

/// Struct for generating sinusoidal samples.
pub struct Sine {
    ind: Math,
    inc: Math,
    sample_rate: Math,
    table: &'static [AccurateMath],
}

impl FreqMod for Sine {
    fn new(f: Math, sample_rate: Math) -> Self {
        Sine {
            ind: Default::default(),
            inc: Math((f.0 * WAVETABLE_SIZE as AccurateMath) / sample_rate.0),
            sample_rate,
            table: WAVETABLE.as_slice(),
        }
    }

    fn set_frequency(&mut self, f: Math) {
        self.inc.0 = (f.0 * WAVETABLE_SIZE as AccurateMath) / self.sample_rate.0;
    }

    fn get_frequency(&self) -> Math {
        Math(self.inc.0 * self.sample_rate.0 / WAVETABLE_SIZE as AccurateMath)
    }
}

impl Generator for Sine {
    fn process(&mut self) -> Sample {
        let k = self.ind.0.floor();
        let k1 = if k + 1.0 >= WAVETABLE_SIZE as AccurateMath {
            0.0
        } else {
            k + 1.0
        } as usize;
        let k = k as usize;
        let g = self.ind.0 - k as AccurateMath;

        let y = ((1.0 - g) * self.table[k] + g * self.table[k1]) as FastMath;

        self.ind.0 += self.inc.0;

        if self.ind.0 >= (WAVETABLE_SIZE as AccurateMath) - 1.0 {
            self.ind.0 -= WAVETABLE_SIZE as AccurateMath - 1.0;
        }

        Sample(y)
    }
}

impl BlockGenerator for Sine {
    fn process_block(&mut self, x: &mut[Sample]) {
        for s in x {
            let k = self.ind.0.floor();
            let k1 = if k + 1.0 >= WAVETABLE_SIZE as AccurateMath {
                0.0
            } else {
                k + 1.0
            } as usize;
            let k = k as usize;
            let g = self.ind.0 - k as AccurateMath;

            (*s).0 = ((1.0 - g) * self.table[k] + g * self.table[k1]) as FastMath;

            self.ind.0 += self.inc.0;

            if self.ind.0 >= (WAVETABLE_SIZE as AccurateMath) - 1.0 {
                self.ind.0 -= WAVETABLE_SIZE as AccurateMath - 1.0;
            }
        }
    }
}

impl Clone for Sine {
    fn clone(&self) -> Self {
        Sine {
            ind: Default::default(),
            inc: self.inc,
            sample_rate: self.sample_rate,
            table: self.table,
        }
    }
}
