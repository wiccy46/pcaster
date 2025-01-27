//! Audio processing node system.
//! 
//! This module provides a flexible audio processing framework based on nodes that can be
//! chained together. Each node implements a specific audio processing function (like gain,
//! delay, EQ, etc.) and can be combined in various ways to create complex audio
//! processing chains.
//! 
//! # Example
//! 
//! ```no_run
//! use pcaster::process::{AudioNode, AudioNodeChain, GainNode};
//! 
//! // Create a processing chain
//! let mut chain = AudioNodeChain::new();
//! 
//! // Add processing nodes
//! chain.add_node(GainNode::new(6.0));  // +6 dB gain
//! 
//! // Process audio
//! let input = vec![0.5f32; 1000];
//! let output = chain.process(&input);
//! ```

/// Represents an audio processing node that can be chained with other nodes.
/// 
/// This trait defines the interface for all audio processing nodes in the system.
/// Implementing this trait allows a node to be used in an [`AudioNodeChain`].
/// 
/// # Examples
/// 
/// ```no_run
/// use pcaster::process::AudioNode;
/// 
/// struct MyNode {
///     gain: f32,
/// }
/// 
/// impl AudioNode for MyNode {
///     fn process(&self, input: &[f32]) -> Vec<f32> {
///         input.iter().map(|&x| x * self.gain).collect()
///     }
///     
///     fn process_in_place(&self, buffer: &mut [f32]) {
///         buffer.iter_mut().for_each(|x| *x *= self.gain);
///     }
///     
///     fn node_type(&self) -> &'static str {
///         "my_node"
///     }
///     
///     fn box_clone(&self) -> Box<dyn AudioNode> {
///         Box::new(Self { gain: self.gain })
///     }
/// }
/// ```
pub trait AudioNode {
    /// Process audio samples and return the processed result.
    /// 
    /// This method takes a slice of input samples and returns a new vector
    /// containing the processed samples. The samples are expected to be
    /// in the range [-1.0, 1.0].
    /// 
    /// # Arguments
    /// 
    /// * `input` - Slice of input samples to process
    /// 
    /// # Returns
    /// 
    /// A new vector containing the processed samples
    fn process(&self, input: &[f32]) -> Vec<f32>;
    
    /// Process audio samples in-place.
    /// 
    /// This method modifies the input buffer directly instead of allocating
    /// a new vector. This can be more efficient when processing large amounts
    /// of audio data.
    /// 
    /// # Arguments
    /// 
    /// * `buffer` - Mutable slice of samples to process in-place
    fn process_in_place(&self, buffer: &mut [f32]);
    
    /// Get a unique identifier for this type of audio node.
    /// 
    /// This identifier can be used for debugging, logging, or serialization.
    /// It should be a constant string that uniquely identifies the node type.
    fn node_type(&self) -> &'static str;
    
    /// Create a boxed clone of this node.
    /// 
    /// This method is required because trait objects cannot use the standard
    /// Clone trait. It allows nodes to be cloned when needed by the processing
    /// chain.
    fn box_clone(&self) -> Box<dyn AudioNode>;
}

/// A chain of audio processing nodes that can be executed sequentially.
/// 
/// This struct allows multiple audio processing nodes to be connected together
/// and executed in sequence. Each node's output becomes the input for the next
/// node in the chain.
/// 
/// # Example
/// 
/// ```no_run
/// use pcaster::process::{AudioNodeChain, GainNode};
/// 
/// let mut chain = AudioNodeChain::new();
/// 
/// // Add multiple processing nodes
/// chain.add_node(GainNode::new(6.0));   // First apply +6 dB gain
/// chain.add_node(GainNode::new(-3.0));  // Then apply -3 dB gain
/// 
/// // Process audio through the entire chain
/// let input = vec![0.5f32; 1000];
/// let output = chain.process(&input);
/// ```
#[derive(Default)]
pub struct AudioNodeChain {
    nodes: Vec<Box<dyn AudioNode>>,
}

impl AudioNodeChain {
    /// Creates a new empty audio processing chain.
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }
    
    /// Adds a node to the end of the processing chain.
    /// 
    /// The node will be executed after all previously added nodes.
    /// 
    /// # Arguments
    /// 
    /// * `node` - The node to add to the chain
    pub fn add_node<T: AudioNode + 'static>(&mut self, node: T) {
        self.nodes.push(Box::new(node));
    }
    
    /// Processes audio through the entire chain.
    /// 
    /// Each node in the chain processes the audio sequentially, with each node's
    /// output becoming the input for the next node.
    /// 
    /// # Arguments
    /// 
    /// * `input` - The input samples to process
    /// 
    /// # Returns
    /// 
    /// A new vector containing the processed samples
    pub fn process(&self, input: &[f32]) -> Vec<f32> {
        let mut buffer = input.to_vec();
        for node in &self.nodes {
            buffer = node.process(&buffer);
        }
        buffer
    }
    
    /// Processes audio through the entire chain in-place.
    /// 
    /// Similar to `process`, but modifies the input buffer directly instead
    /// of allocating new vectors. This can be more efficient for large
    /// audio files.
    /// 
    /// # Arguments
    /// 
    /// * `buffer` - Mutable slice of samples to process
    pub fn process_in_place(&self, buffer: &mut [f32]) {
        for node in &self.nodes {
            node.process_in_place(buffer);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[derive(Clone)]
    struct TestNode {
        multiplier: f32,
    }

    impl TestNode {
        fn new(multiplier: f32) -> Self {
            Self { multiplier }
        }
    }

    impl AudioNode for TestNode {
        fn process(&self, input: &[f32]) -> Vec<f32> {
            input.iter().map(|&x| x * self.multiplier).collect()
        }

        fn process_in_place(&self, buffer: &mut [f32]) {
            buffer.iter_mut().for_each(|x| *x *= self.multiplier);
        }

        fn node_type(&self) -> &'static str {
            "test"
        }

        fn box_clone(&self) -> Box<dyn AudioNode> {
            Box::new(self.clone())
        }
    }

    #[fixture]
    fn test_input() -> Vec<f32> {
        vec![1.0, 2.0, 3.0]
    }

    #[fixture]
    fn test_node() -> TestNode {
        TestNode::new(2.0)
    }

    #[rstest]
    #[case(2.0, vec![1.0, 2.0, 3.0], vec![2.0, 4.0, 6.0])]
    #[case(3.0, vec![1.0, 2.0, 3.0], vec![3.0, 6.0, 9.0])]
    #[case(0.5, vec![2.0, 4.0, 6.0], vec![1.0, 2.0, 3.0])]
    fn test_node_processing(#[case] multiplier: f32, #[case] input: Vec<f32>, #[case] expected: Vec<f32>) {
        let node = TestNode::new(multiplier);
        
        // Test normal processing
        let output = node.process(&input);
        assert_eq!(output, expected);
        
        // Test in-place processing
        let mut buffer = input.clone();
        node.process_in_place(&mut buffer);
        assert_eq!(buffer, expected);
    }

    #[rstest]
    fn test_chain_empty(test_input: Vec<f32>) {
        let chain = AudioNodeChain::new();
        let output = chain.process(&test_input);
        assert_eq!(output, test_input);
    }

    #[rstest]
    #[case(vec![TestNode::new(2.0)], vec![1.0, 2.0, 3.0], vec![2.0, 4.0, 6.0])]
    #[case(vec![TestNode::new(2.0), TestNode::new(3.0)], vec![1.0, 2.0, 3.0], vec![6.0, 12.0, 18.0])]
    fn test_chain_processing(
        #[case] nodes: Vec<TestNode>,
        #[case] input: Vec<f32>,
        #[case] expected: Vec<f32>
    ) {
        let mut chain = AudioNodeChain::new();
        for node in nodes {
            chain.add_node(node);
        }
        
        // Test normal processing
        let output = chain.process(&input);
        assert_eq!(output, expected);
        
        // Test in-place processing
        let mut buffer = input.clone();
        chain.process_in_place(&mut buffer);
        assert_eq!(buffer, expected);
    }

    #[rstest]
    fn test_box_clone(test_node: TestNode, test_input: Vec<f32>) {
        let cloned = test_node.box_clone();
        let output1 = test_node.process(&test_input);
        let output2 = cloned.process(&test_input);
        assert_eq!(output1, output2);
    }

}
