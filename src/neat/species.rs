use super::Genome;

pub struct Species {
    /// Representative genome
    rep_genome: Genome,
    id: u32,
    individuals: Vec<u32>,
}

impl Species {
    pub fn new(rep_genome: &Genome) -> Self {
        Species {
            rep_genome: rep_genome.clone(),
            id: 0,
            individuals: Vec::new(),
        }
    }
}
