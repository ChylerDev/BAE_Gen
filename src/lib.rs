//! # BAE_Gen
//!
//! Crate including many of the common/basic sound types including sine,
//! sawtooth, square, white noise, and more.
//!
//! This crate was created for use as a base of bare essentials from which to
//! build more complex systems from (e.g. more complex synthesizers and effects).

#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/bae_gen/0.14.0")]

use bae_types::*;

// pub mod mono_wav;
pub mod noise;
pub mod sawtooth;
pub mod sine;
pub mod square;
pub mod triangle;
pub mod zero;

// pub use mono_wav::*;
pub use noise::*;
pub use sawtooth::*;
pub use sine::*;
pub use square::*;
pub use triangle::*;
pub use zero::*;

/// Frequency Moderator. This trait defines types who take in a frequency as a
/// primary argument.
pub trait FreqMod {
    /// Creates a new object for the given frequency.
    ///
    /// # Parameters
    ///
    /// * `f` - The frequency for the new object
    /// * `sample_rate` - The sample rate that will be used.
    fn new(f: Math, sample_rate: Math) -> Self;

    /// Sets the frequency of the given object.
    ///
    /// # Parameters
    ///
    /// * `f` - The new frequency.
    fn set_frequency(&mut self, f: Math);

    /// Gets the current frequency of the given object.
    fn get_frequency(&self) -> Math;
}

/// The `Generator` trait defines types that create audio samples.
pub trait Generator {
    /// Generates a rendered audio sample
    fn process(&mut self) -> Sample;
}

/// The `BlockGenerator` trait defines types that create audio samples in blocks
/// or chunks.
pub trait BlockGenerator {
    /// Generates samples of rendered audio in blocks.
    fn process_block(&mut self, x: &mut[Sample]);
}
