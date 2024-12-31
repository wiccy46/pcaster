use std::path::PathBuf;
use pcaster::io::AudioReader;
use pcaster::io::AudioWriter;

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
    // Process the samples here
    all_samples.iter_mut().for_each(|sample| *sample *= 1.2);

    let output_path = PathBuf::from("tmp/output.wav");
    let mut writer = AudioWriter::from_reader(output_path, &reader)?;

    writer.write_samples(&all_samples)?;
    writer.finalize()?;

    Ok(())
} 