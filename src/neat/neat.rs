use std::fmt;
use pyo3::*;
use super::Network;

/**
Main class.
This class allows you to train an AI.
 */
#[pyclass]
pub struct NEAT {
    pop: Vec<Network>,
    inputs: usize,
    outputs: usize,
}

impl fmt::Display for NEAT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[NEAT: {{pop_size: {}, inputs: {}, outputs: {}}}]",
            self.pop_size(),
            self.inputs,
            self.outputs
        )
    }
}

#[pymethods]
impl NEAT {

    #[new]
    #[args(pop_size, inputs, outputs)]
    pub fn new(pop_size: usize, inputs: usize, outputs: usize) -> Self {
        NEAT {
            pop: Vec::new(),
            inputs,
            outputs,
        }.populate(pop_size)
    }

    pub fn pop_size(&self) -> usize {
        self.pop.len()
    }

    fn __str__(&self) -> String {
        format!("{}", self)
    }
}

impl NEAT {
    fn populate(mut self, pop_size: usize) -> Self {
        self.pop.reserve(pop_size);
        for _ in 0..pop_size {
            self.pop.push(Network::new(self.inputs as u32, self.outputs as u32, None));
        }
        self
    }
}
