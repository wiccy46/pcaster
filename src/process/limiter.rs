use std::collections::VecDeque;
use super::node::AudioNode;


#[derive(Clone)]
pub struct LimiterNode {
    threshold: f32,
    release_coeff: f32,
    envelope: f32,
    lookahead_buffer: VecDeque<f32>,
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

        LimiterNode {
            threshold,
            release_coeff,
            envelope: 0.0,
            lookahead_buffer: VecDeque::with_capacity(lookahead_samples),
            lookahead_samples,
        }
    }

    pub fn process_sample(&mut self, sample: f32) -> f32 {
        self.lookahead_buffer.push_back(sample);

        // If lookahead not full, return 0.0, TODO: Can do fade in instead
        if self.lookahead_buffer.len() > self.lookahead_samples {
            return 0.0;
        }
        let future_idx = self.lookahead_buffer.len() - 1;
        let future_sample = self.lookahead_buffer[future_idx];

        let input_lvl = future_sample.abs();
        if input_lvl > self.envelope {
            self.envelope = input_lvl;
        } else {
            // release
            self.envelope = self.release_coeff * self.envelope
                            + (1.0 - self.release_coeff) * input_lvl;
        }

        let gain = if self.envelope > self.threshold {
            self.threshold / self.envelope
        } else {
            1.0
        };

        let output_sample = self.lookahead_buffer.pop_front().unwrap() * gain;

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

