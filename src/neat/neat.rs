use std::fmt;

use super::Network;

/**
Main class.
This class allows you to train an AI.
 */
pub struct NEAT {
    pop: Vec<Network>,
    inputs: usize,
    outputs: usize,
}

impl fmt::Display for NEAT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[NEAT: {{pop_size: {}, ouputs: {}, inputs: {}}}]",
            self.pop_size(),
            self.outputs,
            self.inputs
        )
    }
}

impl NEAT {
    pub fn new(pop_size: usize, inputs: usize, outputs: usize) -> Self {
        let mut neat = NEAT {
            pop: Vec::new(),
            inputs,
            outputs,
        };
        neat.pop.reserve(pop_size);
        neat
    }

    pub fn pop_size(&self) -> usize {
        self.pop.len()
    }

    fn __str__(&self) -> String {
        format!("{}", self)
    }
}
