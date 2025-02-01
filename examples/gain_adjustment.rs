extern crate plotly;
use std::path::PathBuf;
use sonex::io::AudioReader;
use sonex::process::{AudioNodeChain, GainNode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from("audio/sin_100Hz_-3dBFS_3s.wav");
    
    let mut reader = AudioReader::new(path)?;
    println!("Audio file info:");
    println!("Sample rate: {} Hz", reader.sample_rate());
    println!("Channels: {}", reader.channels());
    
    let mut original_samples = Vec::new();
    while let Ok(Some(samples)) = reader.read_packet() {
        original_samples.extend(samples);
    }
    
    let mut chain_amplify = AudioNodeChain::new();
    chain_amplify.add_node(GainNode::new(6.0));  // +6 dB gain
    
    let mut chain_attenuate = AudioNodeChain::new();
    chain_attenuate.add_node(GainNode::new(-6.0));  // -6 dB gain
    
    let amplified_samples = chain_amplify.process(&original_samples);
    let attenuated_samples = chain_attenuate.process(&original_samples);
    println!("\nFirst 10 samples of each signal:");
    println!("Original:");
    for sample in original_samples.iter().take(10) {
        println!("{:.6}", sample);
    }
    
    println!("\nAmplified (+6 dB):");
    for sample in amplified_samples.iter().take(10) {
        println!("{:.6}", sample);
    }
    
    println!("\nAttenuated (-6 dB):");
    for sample in attenuated_samples.iter().take(10) {
        println!("{:.6}", sample);
    }
    
    Ok(())
} 