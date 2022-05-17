use std::fmt;
use pyo3::*;
use super::Genome;

/**
Main class.
This class allows you to train an AI.
 */
#[pyclass]
pub struct NEAT {
    pop: Vec<Genome>,
    n_inputs: usize,
    n_outputs: usize,
}

impl fmt::Display for NEAT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[NEAT: {{pop_size: {}, inputs: {}, outputs: {}}}]",
            self.pop_size(),
            self.n_inputs,
            self.n_outputs
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
            n_inputs: inputs,
            n_outputs: outputs,
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
            self.pop.push(Genome::new(self.n_inputs as u32, self.n_outputs as u32));
        }
        self
    }
}
