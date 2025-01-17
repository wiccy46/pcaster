//! Audio file reading functionality.
//! 
//! This module provides the ability to read various audio file formats using the Symphonia backend.
//! It supports reading common formats like WAV, FLAC, MP3, etc., and converts them to normalized
//! floating-point samples.

use std::fs::File;
use std::path::Path;

use symphonia::core::codecs::{Decoder, DecoderOptions, CODEC_TYPE_NULL, CodecParameters};
use symphonia::core::formats::{FormatOptions, FormatReader, Track};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::probe::Hint;
use symphonia::default::get_probe;

/// A high-level audio file reader that provides easy access to audio samples.
/// 
/// This struct handles the complexities of audio decoding and provides a simple interface
/// to read audio samples as normalized floating-point values.
/// 
/// # Example
/// 
/// ```no_run
/// use pcaster::io::AudioReader;
/// 
/// let mut reader = AudioReader::new("audio.wav").unwrap();
/// println!("Sample rate: {} Hz", reader.sample_rate());
/// println!("Channels: {}", reader.channels());
/// 
/// while let Ok(Some(samples)) = reader.read_packet() {
///     // Process samples...
/// }
/// ```
pub struct AudioReader {
    format: Box<dyn FormatReader>,
    decoder: Box<dyn Decoder>,
    track: Track,
    spec: CodecParameters,
}

impl AudioReader {
    /// Creates a new audio reader for the specified file path.
    /// 
    /// # Arguments
    /// 
    /// * `path` - Path to the audio file
    /// 
    /// # Returns
    /// 
    /// Returns a Result containing the AudioReader if successful, or a SymphoniaError if the file
    /// cannot be opened or decoded.
    /// 
    /// # Errors
    /// 
    /// Will return an error if:
    /// * The file cannot be opened
    /// * The format is not recognized
    /// * No supported audio tracks are found
    /// * The codec is not supported
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, SymphoniaError> {
        // Provide hints to the probe to speed up the process.
        let mut hint = Hint::new();
        if let Some(ext) = path.as_ref().extension().and_then(|s| s.to_str()) {
            hint.with_extension(ext);
        }

        let src = File::open(path).expect("failed to open file");

        let mss = MediaSourceStream::new(Box::new(src), Default::default());

        // Probe the media for a compatible format reader
        let probed = get_probe().format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )?;

        let format = probed.format;

        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .expect("no supported audio tracks")
            .clone();
        

        // Create a decoder
        let dec_opts: DecoderOptions = Default::default();

        let decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &dec_opts)
            .expect("unsupported codec");

        let spec = decoder.codec_params().clone();

        Ok(Self {
            format,
            decoder,
            track,
            spec,
        })
    }

    /// Returns the sample rate of the audio file in Hz.
    pub fn sample_rate(&self) -> u32 {
        self.spec.sample_rate.unwrap()
    }

    /// Returns the number of audio channels.
    pub fn channels(&self) -> usize {
        self.spec.channels.unwrap().count()
    }

    /// Reads and decodes the next packet of audio samples.
    /// 
    /// Returns a vector of interleaved floating-point samples normalized to the range [-1.0, 1.0].
    /// The samples are interleaved in the format [L,R,L,R,...] for stereo, or [L,R,C,L,R,C,...] 
    /// for multi-channel audio.
    /// 
    /// # Returns
    /// 
    /// * `Ok(Some(samples))` - Successfully read samples
    /// * `Ok(None)` - End of file reached
    /// * `Err(e)` - An error occurred during reading or decoding
    pub fn read_packet(&mut self) -> Result<Option<Vec<f32>>, SymphoniaError> {
        loop {
            let packet = match self.format.next_packet() {
                Ok(packet) => packet,
                Err(e) => match e {
                    SymphoniaError::IoError(_) => return Ok(None),
                    _ => return Err(e),
                },
            };

            // If the packet doesn't match our chosen track, skip it
            if packet.track_id() != self.track.id {
                continue;
            }

            let decoded = self.decoder.decode(&packet)?;
            // Create a sample buffer
            let spec = *decoded.spec();
            let duration = decoded.capacity() as u64;
            let mut sample_buf = SampleBuffer::<f32>::new(duration, spec);

            sample_buf.copy_interleaved_ref(decoded);

            return Ok(Some(sample_buf.samples().to_vec()));
        }
    }
}
