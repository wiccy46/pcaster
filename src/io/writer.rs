use std::path::Path;
use hound::{WavWriter, WavSpec, SampleFormat};
use crate::io::AudioReader;

pub struct AudioWriter {
    writer: WavWriter<std::io::BufWriter<std::fs::File>>,
}

impl AudioWriter {
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

    pub fn write_samples(&mut self, samples: &[f32]) -> Result<(), hound::Error> {
        for &sample in samples {
            self.writer.write_sample(sample)?;
        }
        Ok(())
    }

    /// When all samples done writing, finalize it to file.
    pub fn finalize(self) -> Result<(), hound::Error> {
        self.writer.finalize()
    }
}
