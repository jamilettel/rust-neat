use core::fmt;

use super::{Gene, LinkTo, Network, SETTINGS};
use pyo3::*;

#[pyclass]
pub struct Genome {
    pub id: u32,
    pub genes: Vec<Gene>,
    pub n_nodes: u32,
    n_inputs: u32,
    n_outputs: u32,
    pub fitness: f64,
    pub adj_fitness: f64,
    pub network: Option<Network>,
}

static MAX_TRIES_MUTATIONS: i32 = 10;

impl fmt::Display for Genome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[Genome: {{ id: {}, fitness: {}, adj_fitness: {} }}]",
            self.id, self.fitness, self.adj_fitness
        )
    }
}

#[pymethods]
impl Genome {
    fn __str__(&self) -> String {
        format!("{}", self)
    }

    pub fn compute(&mut self, inputs: Vec<f64>) -> Vec<f64> {
        self.build_network();
        let network = self.get_network_mut();
        network.set_inputs(inputs);
        network.compute();
        network.get_outputs()
    }
}

impl Clone for Genome {
    fn clone(&self) -> Self {
        Genome {
            id: self.id,
            genes: self.genes.clone(),
            n_nodes: self.n_nodes,
            n_inputs: self.n_inputs,
            n_outputs: self.n_outputs,
            fitness: 0.0,
            adj_fitness: 0.0,
            network: self.network.clone(),
        }
    }
}

/// General genome functions
impl Genome {
    pub fn new(id: u32, n_inputs: u32, n_outputs: u32) -> Self {
        Genome {
            id,
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

    fn build_network(&mut self) {
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

    fn get_network_mut(&mut self) -> &mut Network {
        self.network.as_mut().unwrap()
    }

    fn get_network(&self) -> &Network {
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
        let mut node_order: i32 =
            node_order as i32 - self.n_inputs as i32 - 1 - self.n_outputs as i32;
        for node in self.get_network().nodes.iter() {
            if node.0 < &(1 + self.n_inputs + self.n_outputs) {
                continue;
            }
            node_order -= 1;
            if node_order == 0 {
                return *node.0;
            }
        }
        node_order.try_into().unwrap_or(0)
    }

    /**
    Returns two linkable nodes, or None if tries run out.
    Call this function with `tries` as None.
    */
    pub fn get_linkable_nodes(&mut self, tries: Option<i32>) -> Option<(u32, u32)> {
        self.build_network();
        let tries = tries.unwrap_or(MAX_TRIES_MUTATIONS);
        let mut from = rand::random::<u32>() % (self.n_nodes - self.n_outputs); // can't link from outputs
        if from > self.n_inputs {
            from += self.n_outputs;
        }
        from = self.get_nth_node(from);
        let mut min_layer = self.get_network().nodes[&from].layer;
        if min_layer == 0 {
            min_layer = 1;
        }
        let mut nb_linkable_nodes = 0;

        for ele in self.get_network().nodes.iter() {
            if ele.1.layer >= min_layer
                && !self.get_network().nodes[&from]
                    .succ
                    .contains(&LinkTo { to: *ele.0 })
                && from != *ele.0
            {
                nb_linkable_nodes += 1;
            }
        }

        if nb_linkable_nodes == 0 {
            return self.get_linkable_nodes(Some(tries - 1));
        }

        let mut pos_linkable_node = rand::random::<u32>() % nb_linkable_nodes;

        for ele in self.get_network().nodes.iter() {
            if ele.1.layer >= min_layer
                && !self.get_network().nodes[&from]
                    .succ
                    .contains(&LinkTo { to: *ele.0 })
                && from != *ele.0
            {
                if pos_linkable_node == 0 {
                    return Some((from, *ele.0));
                }
                pos_linkable_node -= 1;
            }
        }

        return self.get_linkable_nodes(Some(tries - 1));
    }

    pub fn get_linked_nodes(&mut self) -> Option<(u32, u32)> {
        let gene_nb = rand::random::<usize>() % self.genes.len();
        for i in 0..self.genes.len() {
            let gene = &self.genes[(gene_nb + i) % self.genes.len()];
            if gene.enabled == true {
                return Some((gene.from, gene.to));
            }
        }
        None
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
        let genome = Genome::new(0, 5, 6);
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
        let mut g1 = Genome::new(0, 2, 2);
        let mut g2 = Genome::new(1, 2, 2);

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
        let mut g1 = Genome::new(0, 5, 5);

        let nodes = g1.get_linkable_nodes(None);
        assert!(nodes.is_some());
        let nodes = nodes.unwrap();
        assert!(nodes.0 != nodes.1);
        assert!(nodes.0 < nodes.1); // this is okay because genome only has inputs and outputs
    }

    #[test]
    fn get_linkable_nodes_advanced() {
        for _ in 0..10 {
            let mut g1 = Genome::new(0, 5, 5);

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
