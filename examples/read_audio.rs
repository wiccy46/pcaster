extern crate plotly;
use std::path::PathBuf;
use pcaster::io::AudioReader;
use plotly::common::Mode;
use plotly::{Plot, Scatter};

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

    // Create x-axis values (sample indices)
    let x: Vec<f64> = (0..all_samples.len()).map(|i| i as f64).collect();
    
    let trace = Scatter::new(x, all_samples)
        .mode(Mode::Lines)
        .name("Audio Waveform");

    let mut plot = Plot::new();
    plot.add_trace(trace);
    
    plot.set_layout(plotly::Layout::default()
        .y_axis(plotly::layout::Axis::new().range(vec![-1.0, 1.0]))
    );

    // Show the plot in a web browser
    plot.show();

    Ok(())
} 