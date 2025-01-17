//! # PCaster
//! 
//! `pcaster` is an audio processing library written in Rust that provides tools for reading, 
//! processing, and analyzing audio data.
//! 
//! ## Features
//! 
//! - Audio I/O: Read and write various audio file formats
//! - Audio Processing: Apply various audio effects and transformations
//!   - Gain adjustment (dB-based volume control)
//!   - More effects coming soon...
//! - Audio Analysis: Analyze audio characteristics
//!   - Loudness measurement (LUFS)
//!   - More analysis tools coming soon...
//! 
//! ## Example
//! 
//! ```rust
//! use pcaster::io::AudioReader;
//! use pcaster::process::{AudioNodeChain, GainNode};
//! 
//! // Read audio file
//! let mut reader = AudioReader::new("audio.wav")?;
//! let mut samples = Vec::new();
//! while let Ok(Some(packet)) = reader.read_packet() {
//!     samples.extend(packet);
//! }
//! 
//! // Create a processing chain with gain adjustment
//! let mut chain = AudioNodeChain::new();
//! chain.add_node(GainNode::new(6.0));  // +6 dB gain
//! 
//! // Process the audio
//! let processed = chain.process(&samples);
//! ```
//! 
//! ## Modules
//! 
//! - [`io`]: Audio input/output operations
//! - [`process`]: Audio processing nodes and effects
//! - [`analytic`]: Audio analysis tools

pub mod io;
pub mod analytic;
pub mod process;