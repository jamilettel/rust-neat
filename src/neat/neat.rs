use super::Genome;
use super::Species;
use pyo3::*;
use std::fmt;

/**
Main class.
This class allows you to train an AI.
 */
#[pyclass]
pub struct NEAT {
    pop: Vec<Genome>,
    species: Vec<Species>,
    n_inputs: usize,
    n_outputs: usize,
    genome_next_id: u32,
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
            species: Vec::new(),
            n_inputs: inputs,
            n_outputs: outputs,
            genome_next_id: 0,
        }
        .populate(pop_size)
    }

    pub fn pop_size(&self) -> usize {
        self.pop.len()
    }

    fn __str__(&self) -> String {
        format!("{}", self)
    }
}

impl NEAT {
    fn get_next_genome_id(&mut self) -> u32 {
        self.genome_next_id += 1;
        self.genome_next_id - 1
    }

    fn populate(mut self, pop_size: usize) -> Self {
        self.pop.reserve(pop_size);
        for _ in 0..pop_size {
            let next_genome_id = self.get_next_genome_id();
            self.pop.push(Genome::new(
                next_genome_id,
                self.n_inputs as u32,
                self.n_outputs as u32,
            ));
        }
        self
    }
}
