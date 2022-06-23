use super::Genome;
use super::Species;
use pyo3::*;
use std::fmt;

/**
 * Main class.
 * This class allows you to train an AI.
 */
#[pyclass]
pub struct NEAT {
    pop: Vec<Genome>,
    species: Vec<Species>,
    n_inputs: usize,
    n_outputs: usize,
    genome_next_id: u32,
    species_next_id: u32,
    generation: i32,
}

impl fmt::Display for NEAT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[NEAT: {{gen: {}, pop_size: {}, inputs: {}, outputs: {}}}]",
            self.generation,
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
        let mut neat = NEAT {
            pop: Vec::new(),
            species: Vec::new(),
            n_inputs: inputs,
            n_outputs: outputs,
            genome_next_id: 0,
            species_next_id: 0,
            generation: 0,
        }
        .populate(pop_size)
        .mutate_initial_pop();

        neat.compute_new_pop_species();

        neat
    }

    fn __str__(&self) -> String {
        format!("{}", self)
    }

    #[args(fitness_func)]
    pub fn run_one_gen(&mut self, fitness_function: PyObject) {
        let gil = Python::acquire_gil();
        let py = gil.python();
        for i in 0..self.pop.len() {
            let genome = PyCell::new(py, self.pop.remove(i)).unwrap();
            let fitness: f64;
            {
                let genome_ref = genome.borrow_mut();
                fitness = fitness_function
                    .call1(py, (genome_ref,))
                    .unwrap()
                    .extract(py)
                    .unwrap();
            }
            let mut genome: Genome = genome.extract().unwrap();
            genome.fitness = fitness;
            self.pop.insert(i, genome);
        }

        self.generation += 1;
    }

    pub fn print_species_info(&self) {
        println!("N° of Species {}", self.species.len());
        for species in &self.species {
            println!(
                "Species id: {}, population: {}",
                species.get_id(),
                species.population.len()
            );
        }
    }
}

/// General utils
impl NEAT {
    fn get_next_genome_id(&mut self) -> u32 {
        self.genome_next_id += 1;
        self.genome_next_id - 1
    }

    fn get_next_species_id(&mut self) -> u32 {
        self.species_next_id += 1;
        self.species_next_id - 1
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

/// Speciation
impl NEAT {
    fn get_genome_species(&self, genome: &Genome) -> Option<usize> {
        for i in 0..self.species.len() {
            if self.species[i].belongs(genome) {
                return Some(i);
            }
        }
        None
    }

    /**
    Adds the new population to the species
    Use after creating the new population
    */
    fn compute_new_pop_species(&mut self) {
        for species in &mut self.species {
            species.prep_new_generation();
        }
        for i in 0..self.pop.len() {
            if let Some(index) = self.get_genome_species(&self.pop[i]) {
                self.species[index].population.push(i as u32);
            } else {
                let species_id = self.get_next_genome_id();
                let mut species = Species::new(&self.pop[i], species_id);
                species.population.push(i as u32);
                self.species.push(species);
            }
        }
    }

    /**
    Prepares the species for the next generation.
    This will set the adjusted fitness & the new representative genomes
    */
    fn prepare_species_next_gen(&mut self) {
        for species in &mut self.species {
            if species.population.len() == 0 {
                continue;
            }

            let mut best_index: u32 = species.population[0];
            let mut best_fitness: f64 = self.pop[species.population[0] as usize].fitness;
            for individual in &species.population {
                if best_fitness < self.pop[*individual as usize].fitness {
                    best_index = *individual;
                    best_fitness = self.pop[*individual as usize].fitness;
                }
                self.pop[*individual as usize].adj_fitness =
                    self.pop[*individual as usize].fitness / (species.population.len() as f64);
                species.total_shared_fitness += self.pop[*individual as usize].adj_fitness;
            }

            species.set_new_rep_genome(&self.pop[best_index as usize]);
        }
    }
}
