use pyo3::prelude::*;
use std::fmt;

use super::Network;

/**
Wrapper for the main class.
This class allows you to train an AI from python.
*/
#[pyclass(name = "NEAT")]
pub struct NEATPyWrapper {
    pub neat: NEAT<'static>,
}

#[pymethods]
impl NEATPyWrapper {
    #[new]
    #[args(pop_size, inputs, outputs)]
    pub fn new(pop_size: usize, inputs: usize, outputs: usize) -> Self {
        NEATPyWrapper {
            neat: NEAT::new(pop_size, inputs, outputs),
        }
    }

    pub fn pop_size(&self) -> usize {
        self.neat.pop_size()
    }

    fn __str__(&self) -> String {
        self.neat.__str__()
    }
}

/**
Main class.
This class allows you to train an AI.
 */
pub struct NEAT<'a> {
    pop: Vec<Network<'a>>,
    inputs: usize,
    outputs: usize,
}

impl<'a> fmt::Display for NEAT<'a> {
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

impl<'a> NEAT<'a> {
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
