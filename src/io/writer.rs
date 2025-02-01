//! Audio file writing functionality.
//! 
//! This module provides the ability to write audio data to WAV files.
//! It supports writing 32-bit floating-point samples and can create files
//! with various channel configurations and sample rates.

use std::path::Path;
use hound::{WavWriter, WavSpec, SampleFormat};
use crate::io::AudioReader;

/// A high-level audio file writer for WAV files.
/// 
/// This struct provides a simple interface for writing audio samples to WAV files.
/// It supports 32-bit floating-point samples and can be configured for different
/// channel counts and sample rates.
/// 
/// # Example
/// 
/// ```no_run
/// use sonex::io::AudioWriter;
/// 
/// let mut writer = AudioWriter::new("output.wav", 2, 44100).unwrap();
/// let samples = vec![0.0f32; 1000];
/// writer.write_samples(&samples).unwrap();
/// writer.finalize().unwrap();
/// ```
pub struct AudioWriter {
    writer: WavWriter<std::io::BufWriter<std::fs::File>>,
}

impl AudioWriter {
    /// Creates a new AudioWriter from an existing AudioReader.
    /// 
    /// This is useful when you want to write processed audio with the same
    /// specifications as the input file.
    /// 
    /// # Arguments
    /// 
    /// * `path` - Output file path
    /// * `reader` - Reference to an AudioReader to copy specifications from
    /// 
    /// # Returns
    /// 
    /// Returns a Result containing the AudioWriter if successful.
    pub fn from_reader<P: AsRef<Path>>(path: P, reader: &AudioReader) -> Result<Self, hound::Error> {
        let spec = WavSpec {
            channels: reader.channels() as u16,
            sample_rate: reader.sample_rate(),
            bits_per_sample: 32,
            sample_format: SampleFormat::Float,
        };
        let writer = WavWriter::create(path, spec)?;
        Ok(Self { writer })
    }

    /// Creates a new AudioWriter with specified parameters.
    /// 
    /// # Arguments
    /// 
    /// * `path` - Output file path
    /// * `channels` - Number of audio channels
    /// * `sample_rate` - Sample rate in Hz
    /// 
    /// # Returns
    /// 
    /// Returns a Result containing the AudioWriter if successful.
    pub fn new<P: AsRef<Path>>(
        path: P,
        channels: u16,
        sample_rate: u32,
    ) -> Result<Self, hound::Error> {
        let spec = WavSpec {
            channels,
            sample_rate,
            bits_per_sample: 32,
            sample_format: SampleFormat::Float,
        };
        let writer = WavWriter::create(path, spec)?;
        Ok(Self { writer })
    }

    /// Writes a slice of audio samples to the file.
    /// 
    /// The samples should be interleaved if multi-channel (e.g., [L,R,L,R,...] for stereo).
    /// Each sample should be in the range [-1.0, 1.0].
    /// 
    /// # Arguments
    /// 
    /// * `samples` - Slice of floating-point samples to write
    /// 
    /// # Returns
    /// 
    /// Returns Ok(()) if successful, or an error if the write failed.
    pub fn write_samples(&mut self, samples: &[f32]) -> Result<(), hound::Error> {
        for &sample in samples {
            self.writer.write_sample(sample)?;
        }
        Ok(())
    }

    /// Finalizes the WAV file and ensures all data is written.
    /// 
    /// This method must be called when you're done writing samples to ensure
    /// the WAV file is properly formatted. The writer cannot be used after calling
    /// this method.
    /// 
    /// # Returns
    /// 
    /// Returns Ok(()) if successful, or an error if the finalization failed.
    pub fn finalize(self) -> Result<(), hound::Error> {
        self.writer.finalize()
    }
}
