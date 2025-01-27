//! Loudness measurement functionality based on the EBU R128 standard.
//! 
//! This module provides tools for measuring audio loudness according to the EBU R128 standard,
//! which includes integrated LUFS, short-term LUFS, and true peak measurements.
//! 
//! The EBU R128 standard is widely used in broadcast and streaming to ensure consistent
//! loudness levels across different audio content.

use ebur128::{EbuR128, Mode};

/// A loudness meter implementing the EBU R128 standard.
/// 
/// This struct provides methods to measure various aspects of audio loudness:
/// - Integrated LUFS (overall loudness)
/// - Short-term LUFS (3-second window)
/// - True peak levels
/// 
/// # Example
/// 
/// ```no_run
/// use pcaster::analytic::Meter;
/// 
/// let samples = vec![0.0f32; 1000];
/// let meter = Meter::new(&samples, 2, 44100);
/// 
/// if let Some(lufs) = meter.lufs_integrated() {
///     println!("Integrated LUFS: {}", lufs);
/// }
/// ```
#[derive(Debug)]
pub struct Meter {
    meter: EbuR128,
    channels: u32,
    #[allow(dead_code)]
    sample_rate: u32
}

impl Meter {
    /// Creates a new loudness meter for the given audio data.
    /// 
    /// # Arguments
    /// 
    /// * `samples` - Interleaved audio samples
    /// * `channels` - Number of audio channels
    /// * `sample_rate` - Sample rate in Hz
    /// 
    /// # Returns
    /// 
    /// Returns a new Meter instance configured for the given audio parameters.
    pub fn new(samples: &[f32], channels: u32, sample_rate: u32) -> Self {
        let modes = Mode::I | Mode::S | Mode::TRUE_PEAK;
        let mut meter = EbuR128::new(channels, sample_rate, modes)
            .expect("Failed to create EBU R128 meter");
        meter.add_frames_f32(samples).expect("Failed to add frames to meter");
        Self {
            meter,
            channels,
            sample_rate
        }
    }

    /// Measures the integrated loudness (LUFS) of the entire audio.
    /// 
    /// This is the overall loudness value as defined by EBU R128.
    /// The measurement is gated and normalized according to the standard.
    /// 
    /// # Returns
    /// 
    /// Returns Some(value) with the LUFS value if successful, or None if the measurement failed.
    pub fn lufs_integrated(&self) -> Option<f64> {
        self.meter.loudness_global().ok()
    }

    /// Measures the short-term loudness (LUFS) using a 3-second sliding window.
    /// 
    /// This measurement reflects more recent changes in loudness compared to the
    /// integrated measurement.
    /// 
    /// # Returns
    /// 
    /// Returns Some(value) with the LUFS value if successful, or None if the measurement failed.
    pub fn lufs_shortterm(&self) -> Option<f64> {
        self.meter.loudness_shortterm().ok()
    }

    /// Measures the true peak values for each channel.
    /// 
    /// True peak measurements take into account inter-sample peaks that may occur
    /// when the digital signal is converted to analog.
    /// 
    /// # Returns
    /// 
    /// Returns Some(Vec) containing true peak values in dBTP for each channel,
    /// or None if the measurement failed.
    pub fn true_peaks(&self) -> Option<Vec<f64>> {
        let true_peaks: Option<Vec<f64>> = (0..self.channels)
            .map(|ch| self.meter.true_peak(ch as u32).ok())
            .collect::<Option<Vec<f64>>>();
        true_peaks
    }
}
