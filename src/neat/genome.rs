use super::{Gene, Network, SETTINGS};

pub struct Genome {
    pub genes: Vec<Gene>,
    pub n_nodes: u32,
    n_inputs: u32,
    n_outputs: u32,
    pub fitness: f64,
    pub adj_fitness: f64,
    pub network: Option<Network>,
}

static MAX_TRIES_MUTATIONS: i32 = 10;

impl Clone for Genome {
    fn clone(&self) -> Self {
        Genome {
            genes: self.genes.clone(),
            n_nodes: self.n_nodes,
            n_inputs: self.n_inputs,
            n_outputs: self.n_outputs,
            fitness: 0.0,
            adj_fitness: 0.0,
            network: None,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.genes.clone_from(&source.genes);
        self.n_nodes = source.n_nodes;
        self.n_inputs = source.n_inputs;
        self.n_outputs = source.n_outputs;
        self.fitness = 0.0;
        self.adj_fitness = 0.0;
        self.network.take();
    }
}

/// General genome functions
impl Genome {
    pub fn new(n_inputs: u32, n_outputs: u32) -> Self {
        Genome {
            genes: Vec::new(),
            n_nodes: n_inputs + n_outputs + 1, // inputs + ouputs + bias
            n_inputs,
            n_outputs,
            fitness: 0.0,
            adj_fitness: 0.0,
            network: None,
        }
        .build_genome()
    }

    fn build_genome(mut self) -> Self {
        let mut historical_marking = 0;

        // we start at 1 because 0 is the bias node
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

    pub fn build_network(&mut self) {
        if self.network.is_none() {
            self.network = Some(Network::new(&self, self.n_inputs, self.n_outputs));
            self.n_nodes = self
                .network
                .as_ref()
                .unwrap()
                .nodes
                .len()
                .try_into()
                .unwrap();
        }
    }

    pub fn get_network(&self) -> &Network {
        self.network.as_ref().unwrap()
    }
}

/// Mutate weights
impl Genome {
    /**
    Mutates the genome's weights.
    Always returns true
    */
    pub fn mutate_weights(&mut self) -> bool {
        for gene in &mut self.genes {
            let r: f64 = rand::random();
            let w: f64 = (rand::random::<f64>() - 0.5) * 2.0; // w between -1 and 1
            if r < unsafe { SETTINGS.w_mut_reassign } {
                gene.weight = w * unsafe { SETTINGS.w_mut_reassign_max };
            } else {
                gene.weight += w * unsafe { SETTINGS.w_mut_change_max };
            }
        }
        true
    }
}

/// Adding links & nodes
impl Genome {

    /// Returns the Nth node's ID
    pub fn get_nth_node(&self, node_order: u32) -> u32 {
        // cases where node is: input || output || bias
        if node_order < 1 + self.n_inputs + self.n_outputs {
            return node_order;
        }
        let mut node_order = node_order - self.n_inputs - 1 - self.n_outputs;
        for node in self.get_network().nodes.iter() {
            if node.0 < &(1 + self.n_inputs + self.n_outputs) {
                continue;
            }
            node_order -= 1;
            if node_order == 0 {
                return *node.0;
            }
        }
        node_order
    }

    /**
    Returns two linkable nodes, or None if tries run out.
    Call this function with `tries` as None.
    */
    pub fn get_linkable_nodes(&mut self, tries: Option<i32>) -> Option<(u32, u32)> {
        self.build_network();
        let tries = tries.unwrap_or(MAX_TRIES_MUTATIONS);
        let mut subtries = MAX_TRIES_MUTATIONS;
        let mut from = rand::random::<u32>() % (self.n_nodes - self.n_outputs); // can't link from outputs
        if from > self.n_inputs {
            from += self.n_outputs;
        }
        from = self.get_nth_node(from);
        println!("FROM: {}", from);
        let min_layer = self.get_network().nodes[&from].layer;
        let mut to: u32;
        while {
            if subtries <= 0 {
                if tries <= 0 {
                    return None;
                }
                return self.get_linkable_nodes(Some(tries - 1));
            }
            subtries -= 1;
            to = self.get_nth_node(rand::random::<u32>() % (self.n_nodes - self.n_inputs - 1) + self.n_inputs + 1); // bias + inputs
            println!("TO: {}", to);
            self.get_network().nodes[&to].layer < min_layer && to != from
        } {}
        Some((from, to))
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

    #[test]
    fn get_linkable_nodes_basic() {
        let mut g1 = Genome::new(5, 5);

        let nodes = g1.get_linkable_nodes(None);
        assert!(nodes.is_some());
        let nodes = nodes.unwrap();
        assert!(nodes.0 != nodes.1);
        assert!(nodes.0 < nodes.1); // this is okay because genome only has inputs and outputs
    }

    #[test]
    fn get_linkable_nodes_advanced() {
        for _ in 0..10 {
            let mut g1 = Genome::new(5, 5);

            for i in 117..200 {
                g1.genes.push(Gene {
                    enabled: true,
                    from: 0,
                    to: i,
                    hm: i,
                    weight: 0.1,
                });

                g1.genes.push(Gene {
                    enabled: true,
                    from: i,
                    to: 7,
                    hm: i * 2,
                    weight: 0.1,
                });
            }

            let nodes = g1.get_linkable_nodes(None);
            assert!(nodes.is_some());
            let nodes = nodes.unwrap();
            assert!(nodes.0 != nodes.1);
            let layer1 = g1.network.as_ref().unwrap().nodes[&nodes.0].layer;
            let layer2 = g1.network.as_ref().unwrap().nodes[&nodes.1].layer;
            assert!(layer1 <= layer2);
        }
    }
}
