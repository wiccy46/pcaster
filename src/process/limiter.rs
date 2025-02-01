use std::collections::VecDeque;
use std::cell::{Cell, RefCell};
use super::node::AudioNode;


#[derive(Clone)]
pub struct LimiterNode {
    threshold: f32,
    release_coeff: f32,
    envelope: Cell<f32>,
    lookahead_buffer: RefCell<VecDeque<f32>>,
    lookahead_samples: usize,
}

impl LimiterNode {
    pub fn new(
        threshold: f32,
        release_time_sec: f32,
        lookahead_sec: f32,
        sample_rate: f32
    ) -> Self {
        let release_coeff = (-1.0 / (sample_rate * release_time_sec)).exp();
        let lookahead_samples = (lookahead_sec * sample_rate) as usize;

        Self {
            threshold,
            release_coeff,
            envelope: Cell::new(0.0),
            lookahead_buffer: RefCell::new(VecDeque::with_capacity(lookahead_samples)),
            lookahead_samples,
        }
    }

    pub fn process_sample(&self, sample: f32) -> f32 {
        let mut buffer = self.lookahead_buffer.borrow_mut();
        buffer.push_back(sample);

        if buffer.len() < self.lookahead_samples {
            return sample;  // Pass through input while filling buffer
        }

        let future_idx = buffer.len() - 1;
        let future_sample = buffer[future_idx];
        
        let input_lvl = future_sample.abs();
        let mut envelope = self.envelope.get();
        
        if input_lvl > envelope {
            envelope = input_lvl;
        } else {
            envelope = self.release_coeff * envelope + (1.0 - self.release_coeff) * input_lvl;
        }
        self.envelope.set(envelope);

        let gain = if envelope > self.threshold {
            10.0_f32.powf(self.threshold/20.0) / envelope  // Convert threshold to gain factor
        } else {
            1.0
        };

        let output_sample = buffer.pop_front().unwrap() * gain;
        output_sample
    }


}

impl AudioNode for LimiterNode {
    fn process(&self, input_buffer: &[f32]) -> Vec<f32> {
        let mut out = Vec::with_capacity(input_buffer.len());
        for &sample in input_buffer {
            let limited = self.process_sample(sample);
            out.push(limited);
        }
        out
    }
    
    fn process_in_place(&self, buffer: &mut [f32]) {
        buffer.iter_mut().for_each(|sample| {
            *sample = self.process_sample(*sample);
        });
    }
    
    fn node_type(&self) -> &'static str {
        "limiter"
    }
    
    fn box_clone(&self) -> Box<dyn AudioNode> {
        Box::new(self.clone())
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn test_limiter() -> LimiterNode {
        LimiterNode::new(
            -6.0,      // -6 dB threshold
            0.1,       // 100ms release
            0.001,     // 1ms lookahead
            44100.0    // Standard sample rate
        )
    }

    #[rstest]
    fn test_initial_state(test_limiter: LimiterNode) {
        assert_eq!(test_limiter.threshold, -6.0);
        assert_eq!(test_limiter.envelope.get(), 0.0);
        assert_eq!(test_limiter.lookahead_buffer.borrow().len(), 0);
    }

    #[rstest]
    fn test_lookahead_buffer(test_limiter: LimiterNode) {
        let lookahead_samples = (0.001 * 44100.0) as usize;
        
        // First phase: Buffer filling
        // Should pass through input samples while buffer fills
        for _ in 0..lookahead_samples-1 {
            let output = test_limiter.process_sample(1.0);
            assert_eq!(output, 1.0, "Should pass through input while buffer is filling");
        }
        
        // Second phase: Test actual limiting behavior
        // Feed a large peak and verify the limiter starts reducing gain before the peak
        let peak_value = 2.0; // Above threshold
        let output_at_peak_start = test_limiter.process_sample(peak_value);
        assert!(output_at_peak_start < peak_value, "Limiter should start reducing gain before peak");
        assert!(output_at_peak_start > 0.0, "Output should not be completely silenced");
    }

    #[rstest]
    fn test_limiting_threshold(test_limiter: LimiterNode) {
        let lookahead_samples = (0.001 * 44100.0) as usize;
        
        // Fill the buffer
        for _ in 0..lookahead_samples {
            test_limiter.process_sample(1.0);
        }
        
        // Test with sample above threshold
        let output = test_limiter.process_sample(2.0);  // Should be limited
        assert!(output.abs() <= 10.0_f32.powf(-6.0/20.0), 
            "Output should not exceed threshold");
    }

    #[rstest]
    fn test_release_behavior(test_limiter: LimiterNode) {
        let lookahead_samples = (0.001 * 44100.0) as usize;
        
        // Fill buffer with silence
        for _ in 0..lookahead_samples {
            test_limiter.process_sample(0.0);
        }
        
        // Send one loud sample
        test_limiter.process_sample(2.0);
        let envelope_peak = test_limiter.envelope.get();
        
        // Process more samples and check envelope decreases
        for _ in 0..100 {
            test_limiter.process_sample(0.0);
        }
        
        assert!(test_limiter.envelope.get() < envelope_peak, 
            "Envelope should decrease during release phase");
    }

    #[rstest]
    fn test_process_methods(test_limiter: LimiterNode) {
        let input = vec![0.5f32; 1000];
        
        // Create two identical limiters
        let limiter1 = test_limiter.clone();
        let limiter2 = test_limiter.clone();
        
        // Test process method
        let output1 = limiter1.process(&input);
        
        // Test process_in_place method
        let mut buffer = input.clone();
        limiter2.process_in_place(&mut buffer);
        
        // Both methods should produce identical results
        assert_eq!(output1, buffer);
    }

    #[rstest]
    fn test_node_type_and_clone(test_limiter: LimiterNode) {
        assert_eq!(test_limiter.node_type(), "limiter");
        
        let cloned = test_limiter.box_clone();
        assert_eq!(cloned.node_type(), "limiter");
    }
}

