/// Represents an audio processing node that can be chained with other nodes
pub trait AudioNode {
    /// Process audio samples and return the processed result
    fn process(&self, input: &[f32]) -> Vec<f32>;
    
    /// Process audio samples in-place
    fn process_in_place(&self, buffer: &mut [f32]);
    
    /// Get a unique identifier for this type of audio node
    fn node_type(&self) -> &'static str;
    
    /// Clone the node into a box
    fn box_clone(&self) -> Box<dyn AudioNode>;
}

#[derive(Default)]
pub struct AudioNodeChain {
    nodes: Vec<Box<dyn AudioNode>>,
}

impl AudioNodeChain {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }
    
    pub fn add_node<T: AudioNode + 'static>(&mut self, node: T) {
        self.nodes.push(Box::new(node));
    }
    
    pub fn process(&self, input: &[f32]) -> Vec<f32> {
        let mut buffer = input.to_vec();
        for node in &self.nodes {
            buffer = node.process(&buffer);
        }
        buffer
    }
    
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
