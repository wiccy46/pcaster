//! Gain adjustment processing node.
//! 
//! This module provides functionality for adjusting audio gain (volume) using decibels (dB).
//! The gain adjustment is performed using the standard audio industry formula:
//! linear_gain = 10^(dB/20)
//! 
//! # Example
//! 
//! ```no_run
//! use pcaster::process::{AudioNode, GainNode};
//! 
//! // Create a gain node that amplifies by 6 dB
//! let node = GainNode::new(6.0);
//! 
//! // Process some audio
//! let input = vec![0.5f32; 1000];
//! let output = node.process(&input);  // Samples will be amplified by ~1.995
//! ```

use super::node::AudioNode;

/// An audio processing node that applies gain adjustment in decibels.
/// 
/// This node can both amplify (positive dB) and attenuate (negative dB) audio signals.
/// The gain is applied uniformly to all samples, maintaining the relative dynamics
/// of the audio.
/// 
/// # Examples
/// 
/// ```no_run
/// use pcaster::process::{AudioNode, GainNode};
/// 
/// // Amplify by 6 dB (approximately doubles the amplitude)
/// let amplify = GainNode::new(6.0);
/// 
/// // Attenuate by 6 dB (approximately halves the amplitude)
/// let attenuate = GainNode::new(-6.0);
/// 
/// let input = vec![0.5f32; 1000];
/// let louder = amplify.process(&input);
/// let quieter = attenuate.process(&input);
/// ```
#[derive(Clone)]
pub struct GainNode {
    db: f32,
}

impl GainNode {
    /// Creates a new gain node with the specified dB value.
    /// 
    /// # Arguments
    /// 
    /// * `db` - Gain in decibels (positive for amplification, negative for attenuation)
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// use pcaster::process::GainNode;
    /// 
    /// let node = GainNode::new(6.0);  // +6 dB gain
    /// ```
    pub fn new(db: f32) -> Self {
        Self { db }
    }
    
    /// Returns the current gain setting in dB.
    pub fn db(&self) -> f32 {
        self.db
    }
    
    /// Sets a new gain value in dB.
    /// 
    /// # Arguments
    /// 
    /// * `db` - New gain value in decibels
    pub fn set_db(&mut self, db: f32) {
        self.db = db;
    }
}

impl AudioNode for GainNode {
    fn process(&self, input: &[f32]) -> Vec<f32> {
        let linear_gain = 10.0_f32.powf(self.db / 20.0);
        input.iter()
            .map(|&sample| sample * linear_gain)
            .collect()
    }
    
    fn process_in_place(&self, buffer: &mut [f32]) {
        let linear_gain = 10.0_f32.powf(self.db / 20.0);
        buffer.iter_mut().for_each(|sample| {
            *sample *= linear_gain;
        });
    }
    
    fn node_type(&self) -> &'static str {
        "gain"
    }
    
    fn box_clone(&self) -> Box<dyn AudioNode> {
        Box::new(self.clone())
    }
}

/// Convenience function to apply gain adjustment to samples.
/// 
/// This function creates a temporary GainNode and processes the samples.
/// 
/// # Arguments
/// 
/// * `samples` - Input audio samples
/// * `db` - Gain adjustment in decibels
/// 
/// # Returns
/// 
/// A new vector containing the gain-adjusted samples
pub fn gain_db(samples: &[f32], db: f32) -> Vec<f32> {
    let node = GainNode::new(db);
    node.process(samples)
}

/// Convenience function to apply gain adjustment to samples in-place.
/// 
/// This function creates a temporary GainNode and processes the samples in-place.
/// 
/// # Arguments
/// 
/// * `samples` - Mutable slice of audio samples to adjust
/// * `db` - Gain adjustment in decibels
pub fn gain_db_in_place(samples: &mut [f32], db: f32) {
    let node = GainNode::new(db);
    node.process_in_place(samples);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use std::f32::EPSILON;

    #[fixture]
    fn test_input() -> Vec<f32> {
        vec![1.0, 0.5, -0.5, -1.0]
    }

    #[rstest]
    #[case(0.0, 1.0)]      // 0 dB = unity gain
    #[case(6.0, 1.995262)] // +6 dB ≈ 1.995262
    #[case(-6.0, 0.501187)] // -6 dB ≈ 0.501187
    fn test_gain_conversion(#[case] db: f32, #[case] expected_multiplier: f32) {
        let node = GainNode::new(db);
        let input = vec![1.0];
        let output = node.process(&input);
        assert!((output[0] - expected_multiplier).abs() < 0.0001);
    }

    #[rstest]
    #[case(0.0)]  // Unity gain
    #[case(6.0)]  // Amplification
    #[case(-6.0)] // Attenuation
    fn test_processing_methods(#[case] db: f32, test_input: Vec<f32>) {
        let node = GainNode::new(db);
        
        let output = node.process(&test_input);
        
        let mut buffer = test_input.clone();
        node.process_in_place(&mut buffer);
        
        assert_eq!(output, buffer);
        
        let linear_gain = 10.0_f32.powf(db / 20.0);
        for (i, &sample) in test_input.iter().enumerate() {
            assert!((output[i] - sample * linear_gain).abs() < EPSILON);
        }
    }

    #[rstest]
    fn test_node_properties() {
        let node = GainNode::new(6.0);
        assert_eq!(node.node_type(), "gain");
        assert!((node.db() - 6.0).abs() < EPSILON);
    }

    #[rstest]
    fn test_gain_mutation(test_input: Vec<f32>) {
        let mut node = GainNode::new(0.0);
        
        // Test initial state
        let initial_output = node.process(&test_input);
        assert!(initial_output.iter().zip(test_input.iter())
            .all(|(&a, &b)| (a - b).abs() < EPSILON));
        
        // Test after mutation
        node.set_db(6.0);
        let modified_output = node.process(&test_input);
        let expected: Vec<f32> = test_input.iter()
            .map(|&x| x * 1.995262)
            .collect();
        
        for (actual, expected) in modified_output.iter().zip(expected.iter()) {
            assert!((actual - expected).abs() < 0.0001);
        }
    }

    #[rstest]
    fn test_convenience_functions(test_input: Vec<f32>) {
        let db = 6.0;
        let output1 = gain_db(&test_input, db);
        
        let mut input2 = test_input.clone();
        gain_db_in_place(&mut input2, db);
        
        assert_eq!(output1, input2);
    }

    #[rstest]
    fn test_box_clone(test_input: Vec<f32>) {
        let node = GainNode::new(6.0);
        let cloned = node.box_clone();
        
        let output1 = node.process(&test_input);
        let output2 = cloned.process(&test_input);
        
        assert_eq!(output1, output2);
    }
}