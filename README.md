# pcaster - An audio processing tool for podcasts

## Feature Overview

- Read and write audio files in various formats to floating point vector
  - `sonnex::io::AudioReader` for reading audio files
  - `sonnex::io::AudioWriter` for writing audio files
- Analyze your podcast audio and report statistics relevant to publishing major platforms
  - `sonnex::analytic::Meter` for LUFS, true peaks
- Audio signal processing focused on complying to platforms' audio requirements
  - `sonnex::process::LimiterNode` for limiting audio amplitude
  - `sonnex::process::GainNode` for adjusting audio level
- Speech quality analysis and enhancement
  - TODO.

The tool focuses on podcast audio but in a nutshell it is for general audio processing. 
So you if you find it useful it can work on other audio contents as well.

## Library Overview

- `sonnex::io`: Crate for File I/O
- `sonnex::analytic`: Create for audio analysis, metrics
- `sonnex::process`: Crate for audio processing and enhancement

## Usage

Check `examples` folder for example usage. e.g. run `cargo run --example calc_lufs` to 
calculate LUFS and true peaks of an audio file.

### Analyze LUFS

```rust
use sonnex::analytic::Meter;
use sonnex::io::AudioReader;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from("audio/sin_100Hz_-3dBFS_3s.wav");
    let mut reader = AudioReader::new(path).expect("Failed to create audio reader");

    let mut all_samples = Vec::new();
    while let Ok(Some(samples)) = reader.read_packet() {
        all_samples.extend(samples);
    }

    let meter = Meter::new(&all_samples, reader.channels() as u32, reader.sample_rate());
    let integrated_lufs: f64 = meter.lufs_integrated().unwrap();
    let short_term_lufs: f64 = meter.lufs_shortterm().unwrap();
    let true_peaks: Vec<f64> = meter.true_peaks().unwrap();

    println!("Integrated LUFS: {:?}", integrated_lufs);
    println!("Short-term LUFS: {:?}", short_term_lufs);
    println!("True peaks: {:?}", true_peaks); // per channel peak
    Ok(())
}
```