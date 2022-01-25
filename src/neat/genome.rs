use super::Gene;

#[derive(Clone)]
pub struct Genome {
    pub genes: Vec<Gene>,
    pub n_inputs: u32,
    pub n_hidden: u32,
    pub n_outputs: u32,
}

impl Genome {

    pub fn new(n_inputs: u32, n_outputs: u32) -> Self {
        Genome {
            genes: Vec::new(),
            n_hidden: 0,
            n_inputs,
            n_outputs,
        }.build_genome()
    }

    pub fn build_genome(mut self) -> Self {
        let mut historical_marking = 0;
        for i in 1..=self.n_inputs {
            for j in self.n_inputs + 1..=self.n_inputs + self.n_outputs {
                self.genes.push(Gene {
                    enabled: true,
                    from: i,
                    to: j,
                    historical_marking,
                    weight: 0.0,
                });
                historical_marking += 1;
            }
        }
        self
    }

    #[inline(always)]
    pub fn get_total_nodes(&self) -> u32 {
        // inputs + hidden + outputs + bias node
        self.n_inputs + self.n_hidden + self.n_outputs + 1
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_be_built() {
        let genome = Genome::new(5, 6);
        let mut i = 0;
        for from in 1..=5 {
            for to in 6..=11 {
                assert_eq!(genome.genes[i].from, from);
                assert_eq!(genome.genes[i].to, to);
                i += 1;
            }
        }
        assert_eq!(genome.n_inputs, 5);
        assert_eq!(genome.n_outputs, 6);
        assert_eq!(genome.n_hidden, 0);
    }

}
