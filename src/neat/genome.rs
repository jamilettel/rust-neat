use super::{Gene, SETTINGS};

#[derive(Clone)]
pub struct Genome {
    pub genes: Vec<Gene>,
    pub n_nodes: u32,
    pub fitness: f64,
    pub adj_fitness: f64,
}

/// General genome functions
impl Genome {
    pub fn new(n_inputs: u32, n_outputs: u32) -> Self {
        Genome {
            genes: Vec::new(),
            n_nodes: n_inputs + n_outputs + 1, // inputs + ouputs + bias
            fitness: 0.0,
            adj_fitness: 0.0,
        }
        .build_genome(n_inputs, n_outputs)
    }

    fn build_genome(mut self, n_inputs: u32, n_outputs: u32) -> Self {
        let mut historical_marking = 0;

        // we start at 1 because 0 is the bias node
        for i in 1..=n_inputs {
            for j in n_inputs + 1..=n_inputs + n_outputs {
                self.genes.push(Gene {
                    enabled: true,
                    from: i,
                    to: j,
                    hm: historical_marking,
                    weight: 0.0,
                });
                historical_marking += 1;
            }
        }
        self
    }

    pub fn mutate_weights(&mut self) {
        for gene in &mut self.genes {
            let r: f64 = rand::random();
            let w: f64 = (rand::random::<f64>() - 0.5) * 2.0; // w between -1 and 1
            if r < unsafe { SETTINGS.w_mut_reassign } {
                gene.weight = w * unsafe { SETTINGS.w_mut_reassign_max };
            } else {
                gene.weight += w * unsafe { SETTINGS.w_mut_change_max };
            }
        }
    }
}

/// This impl block contains code for computing differences
impl Genome {
    fn update_disjoint_excess(
        higher_hm: &Self,
        i: &mut usize,
        smaller_hm: &Self,
        j: &mut usize,
        disjoint: &mut u32,
        excess: &mut u32,
    ) {
        let mut offset = 0;
        while smaller_hm.genes.len() > *j + offset
            && higher_hm.genes[*i].hm > smaller_hm.genes[*j + offset].hm
        {
            offset += 1;
        }
        *j += offset;
        if *j >= smaller_hm.genes.len() {
            *excess += (higher_hm.genes.len() - *i) as u32;
            *i = higher_hm.genes.len();
        }
        *disjoint += offset as u32;
    }

    fn get_largest_smallest<'a>(a: &'a Self, b: &'a Self) -> (&'a Self, &'a Self) {
        if a.genes.len() > b.genes.len() {
            (a, b)
        } else {
            (b, a)
        }
    }

    fn get_n(a: &Self, b: &Self) -> u32 {
        let n = std::cmp::max(a.n_nodes, b.n_nodes);
        if n <= unsafe { SETTINGS.small_genome_size } {
            1
        } else {
            n
        }
    }

    /**
    returns a tuple with:
    - weight difference: f64
    - number of disjoint: u32
    - number of excess: u32
    */
    fn get_differnce_values(a: &Self, b: &Self) -> (f64, u32, u32) {
        let (largest, smallest) = Genome::get_largest_smallest(a, b);
        let mut w_diff = 0.0;
        let mut disjoint = 0;
        let mut excess = 0;

        let mut j = 0;
        let mut i = 0;
        while i < largest.genes.len() && j < smallest.genes.len() {
            if largest.genes[i].hm == smallest.genes[j].hm {
                w_diff += f64::abs(largest.genes[i].weight - smallest.genes[j].weight);
                j += 1;
                i += 1;
            } else if largest.genes[i].hm > smallest.genes[j].hm {
                Genome::update_disjoint_excess(
                    largest,
                    &mut i,
                    smallest,
                    &mut j,
                    &mut disjoint,
                    &mut excess,
                );
            } else {
                Genome::update_disjoint_excess(
                    smallest,
                    &mut j,
                    largest,
                    &mut i,
                    &mut disjoint,
                    &mut excess,
                );
            }
        }

        if i < largest.genes.len() {
            excess += (largest.genes.len() - i) as u32;
        } else if j < smallest.genes.len() {
            excess += (smallest.genes.len() - j) as u32;
        }
        (w_diff, disjoint, excess)
    }

    pub fn compute_difference(a: &Self, b: &Self) -> f64 {
        let n = Genome::get_n(a, b);
        let (weight_diff, disjoint, excess) = Genome::get_differnce_values(a, b);

        unsafe {
            (SETTINGS.similarity_c1 * f64::from(excess)
                + SETTINGS.similarity_c2 * f64::from(disjoint))
                / f64::from(n)
                + SETTINGS.similarity_c3 * weight_diff
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_genes_on_build() {
        let genome = Genome::new(5, 6);
        let mut i = 0;
        for from in 1..=5 {
            for to in 6..=11 {
                assert_eq!(genome.genes[i].from, from);
                assert_eq!(genome.genes[i].to, to);
                i += 1;
            }
        }
        assert_eq!(genome.n_nodes, 5 + 6 + 1);
    }

    #[test]
    fn compute_difference() {
        let mut g1 = Genome::new(2, 2);
        let mut g2 = Genome::new(2, 2);

        assert_eq!(Genome::compute_difference(&g1, &g2), 0.0);

        g2.genes[0].weight += 1.0;
        assert_eq!(Genome::compute_difference(&g1, &g2), unsafe {
            SETTINGS.similarity_c3
        });

        g2.genes[0].weight -= 1.0;
        g2.genes.push(Gene {
            enabled: true,
            from: 0,
            to: 1,
            hm: 100,
            weight: 1.0,
        });
        assert_eq!(Genome::compute_difference(&g1, &g2), unsafe {
            SETTINGS.similarity_c1
        });

        g1.genes.push(Gene {
            enabled: true,
            from: 0,
            to: 1,
            hm: 99,
            weight: 1.0,
        });
        assert_eq!(Genome::compute_difference(&g1, &g2), unsafe {
            SETTINGS.similarity_c1 + SETTINGS.similarity_c2
        });
    }
}
