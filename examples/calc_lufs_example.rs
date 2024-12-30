use pcaster::analytic::Meter;
use pcaster::io::AudioReader;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from("audio/sin_100Hz_-3dBFS_3s.wav");

    let mut reader = AudioReader::new(path).expect("Failed to create audio reader");

    println!("Audio file info:");
    println!("Sample rate: {} Hz", reader.sample_rate());
    println!("Channels: {}", reader.channels());

    let mut all_samples = Vec::new();
    while let Ok(Some(samples)) = reader.read_packet() {
        all_samples.extend(samples);
    }

    println!("\nTotal samples read: {}", all_samples.len());

    let mut meter = Meter::new(reader.channels() as u32, reader.sample_rate());

    meter.add_frames_f32(&all_samples);

    let integrated_lufs = meter.lufs_integrated().unwrap();
    let short_term_lufs = meter.lufs_shortterm().unwrap();
    let true_peaks = meter.true_peaks().unwrap();

    println!("Integrated LUFS: {:?}", integrated_lufs);
    println!("Short-term LUFS: {:?}", short_term_lufs);
    println!("True peaks: {:?}", true_peaks);  // This is a vec per channel
    Ok(())
}
