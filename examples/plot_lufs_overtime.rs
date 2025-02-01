use std::path::PathBuf;
use sonex::{io::AudioReader, analytic::Meter};
use plotly::common::Mode;
use plotly::{Plot, Scatter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from("audio/speech_37s_freesound.wav");
    let mut reader = AudioReader::new(path)?;
    
    let sample_rate = reader.sample_rate();
    let channels = reader.channels();
    let samples_per_window = (3.0 * sample_rate as f32) as usize;
    
    println!("Audio file info:");
    println!("Sample rate: {} Hz", sample_rate);
    println!("Channels: {}", channels);
    
    let mut all_samples = Vec::new();
    while let Ok(Some(samples)) = reader.read_packet() {
        all_samples.extend(samples);
    }
    
    let mut lufs_values = Vec::new();
    let mut time_points = Vec::new();
    
    for (i, chunk) in all_samples.chunks(samples_per_window).enumerate() {
        let meter = Meter::new(chunk, channels as u32, sample_rate);
        if let Some(lufs) = meter.lufs_shortterm() {
            if lufs.is_finite() {  // LUFS can be -inf if the last window is too short
                lufs_values.push(lufs);
                time_points.push(i as f64 * 3.0);
            }
        }
    }
    println!("\nLUFS values:");
    for (time, lufs) in time_points.iter().zip(lufs_values.iter()) {
        println!("Time: {:.1}s, LUFS: {:.1}", time, lufs);
    }

    // Create waveform time points (in seconds)
    let waveform_times: Vec<f64> = (0..all_samples.len())
        .map(|i| i as f64 / sample_rate as f64)
        .collect();

    // Create waveform trace with black color and transparency
    let waveform_trace = Scatter::new(waveform_times, all_samples)
        .mode(Mode::Lines)
        .line(plotly::common::Line::new().color("rgba(0, 0, 0, 0.3)"))
        .name("Waveform");
    
    // Create LUFS trace
    let lufs_trace = Scatter::new(time_points, lufs_values)
        .mode(Mode::LinesMarkers)
        .marker(plotly::common::Marker::new().size(10))
        .name("Short-term LUFS");

    let mut plot = Plot::new();
    plot.add_trace(waveform_trace);
    plot.add_trace(lufs_trace);
    
    plot.set_layout(plotly::Layout::default()
        .title("Short-term LUFS and Waveform over Time")
        .x_axis(plotly::layout::Axis::new().title("Time (seconds)"))
        .y_axis(plotly::layout::Axis::new().title("LUFS / Amplitude"))
    );

    plot.show();

    Ok(())
} 