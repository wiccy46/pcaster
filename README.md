# pcaster - An audio processing tool for podcasts

## Feature Overview

- Analyze your podcast audio and report statistics relevant to publishing major platforms
- Audio signal processing focused on complying to platforms' audio requirements
- Speech quality analysis and enhancement

## Library Overview

- io: Crate for File I/O
- analytic: Create for audio analysis, metrics
- process: Crate for audio processing and enhancement

## Usage

Check `examples` folder for example usage. e.g. run `cargo run --example calc_lufs` to 
calculate LUFS and true peaks of an audio file.

### Analyze LUFS

```rust
use pcaster::analytic::Meter;
use pcaster::io::AudioReader;
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