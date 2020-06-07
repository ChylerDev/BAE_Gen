//! # Zero
//!
//! A 0 generator.

use super::*;

/// Zero struct to allow for a Generator object that doesn't do anything.
#[derive(Default, Copy, Clone)]
pub struct Zero {}

impl Zero {
    /// Returns new Zero object.
    pub fn new() -> Self {
        Zero {}
    }
}

impl Generator for Zero {
    fn process(&mut self) -> SampleT {
        0.0
    }
}

impl BlockGenerator for Zero {
    fn process_block(&mut self, x: &mut[SampleT]) {
        for s in x {
            *s = 0.0;
        }
    }
}
