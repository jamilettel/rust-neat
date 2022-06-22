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
    gen: i32,
}

impl fmt::Display for NEAT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[NEAT: {{gen: {}, pop_size: {}, inputs: {}, outputs: {}}}]",
            self.gen,
            self.pop.len(),
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
            gen: 0,
        }
        .populate(pop_size)
        .mutate_initial_pop()
    }

    fn __str__(&self) -> String {
        format!("{}", self)
    }

    #[args(fitness_func)]
    fn run_one_gen(&mut self, fitness_function: PyObject) {
        let gil = Python::acquire_gil();
        let py = gil.python();
        for i in 0..self.pop.len() {
            let genome = PyCell::new(py, self.pop.remove(i)).unwrap();
            let fitness: f64;
            {
                let genome_ref = genome.borrow_mut();
                fitness = fitness_function.call1(py, (genome_ref,)).unwrap().extract(py).unwrap();
            }
            let mut genome: Genome = genome.extract().unwrap();
            genome.fitness = fitness;
            self.pop.insert(i, genome);
        }

        self.gen += 1;
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

    /// This function is only used to mutate the initial population, don't user otherwise
    fn mutate_initial_pop(mut self) -> Self {
        for genome in &mut self.pop {
            genome.mutate_weights();
        }
        self
    }
}
