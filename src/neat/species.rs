use super::{Genome, SETTINGS};

pub struct Species {
    /// Representative genome
    pub rep_genome: Genome,
    id: u32,
    pub population: Vec<u32>,
    pub total_shared_fitness: f64,
}

impl Species {
    pub fn new(rep_genome: &Genome, id: u32) -> Self {
        Species {
            rep_genome: rep_genome.clone(),
            id,
            population: Vec::new(),
            total_shared_fitness: 0.0,
        }
    }

    pub fn belongs(&self, other: &Genome) -> bool {
        Genome::compute_difference(&self.rep_genome, other) < unsafe { SETTINGS.max_difference }
    }

    pub fn set_new_rep_genome(mut self, new_rep: &Genome) -> Self {
        let _genome = self.rep_genome;
        self.rep_genome = new_rep.clone();
        self
    }

    pub fn prep_new_generation(&mut self) {
        self.population.clear();
        self.total_shared_fitness = 0.0;
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

#[cfg(test)]
mod species_test {
    // use super::*;
}
