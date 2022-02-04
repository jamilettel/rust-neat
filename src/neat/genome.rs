use super::{Gene, SETTINGS};

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
        }
        .build_genome()
    }

    pub fn build_genome(mut self) -> Self {
        let mut historical_marking = 0;
        for i in 1..=self.n_inputs {
            for j in self.n_inputs + 1..=self.n_inputs + self.n_outputs {
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

    pub fn compute_difference(a: &Self, b: &Self) -> f64 {
        let mut n = std::cmp::max(a.get_total_nodes(), b.get_total_nodes());
        let mut w_diff = 0.0;
        let mut disjoint = 0;
        let mut excess = 0;

        if n <= unsafe { SETTINGS.small_genome_size } {
            n = 1;
        }

        let largest: &Self;
        let smallest: &Self;
        if a.genes.len() > b.genes.len() {
            largest = a;
            smallest = b;
        } else {
            largest = b;
            smallest = a;
        };

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

        unsafe {
            (SETTINGS.similarity_c1 * f64::from(excess)
                + SETTINGS.similarity_c2 * f64::from(disjoint))
                / f64::from(n)
                + SETTINGS.similarity_c3 * w_diff
        }
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
