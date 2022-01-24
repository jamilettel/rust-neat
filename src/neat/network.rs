use std::collections::HashMap;

use crate::neat::Gene;
use crate::neat::Node;

use super::sigmoid;
use super::NodeType;

/**
Network represents an individual, a network of nodes.
 */
pub struct Network {
    n_inputs: u32,
    n_outputs: u32,
    n_hidden: u32,
    nodes: HashMap<u32, Node>,
    genome: Vec<Gene>,
}

impl Network {
    /// Creates a new Network using the genome
    pub fn new(n_inputs: u32, n_outputs: u32, genome: Option<Vec<Gene>>) -> Self {
        let mut network = Network {
            n_inputs,
            n_hidden: 0,
            n_outputs,
            nodes: HashMap::new(),
            genome: genome.unwrap_or_else(|| Network::build_genome(n_inputs, n_outputs)),
        }
        .build();
        network.compute_layers();
        network
    }

    /// Builds an empty genome
    fn build_genome(n_inputs: u32, n_outputs: u32) -> Vec<Gene> {
        let mut genome = Vec::<Gene>::new();
        let mut historical_marking = 0;
        for i in 1..=n_inputs {
            for j in n_inputs + 1..=n_inputs + n_outputs {
                genome.push(Gene {
                    enabled: true,
                    from: i,
                    to: j,
                    historical_marking,
                    weight: 0.0,
                });
                historical_marking += 1;
            }
        }
        genome
    }

    /// Builds the inputs and outputs, and then the rest of the network using the genome
    fn build(self) -> Self {
        self.build_inputs_outputs().build_network()
    }

    /// Creates the nodes of the network
    fn build_inputs_outputs(mut self) -> Self {
        // outputs + inputs + hidden + 1
        self.nodes
            .reserve((self.n_outputs + self.n_hidden + self.n_inputs + 1) as usize);
        self.nodes.insert(0, Node::new(NodeType::BIAS, Some(0)));
        for i in 1..=self.n_inputs {
            self.nodes.insert(i, Node::new(NodeType::INPUT, Some(0)));
        }
        for i in self.n_inputs + 1..=self.n_outputs + self.n_inputs {
            self.nodes.insert(i, Node::new(NodeType::OUTPUT, None));
        }
        self
    }

    /// Recursively sets the layers on the nodes in the network starting from the inputs
    fn compute_layers(&mut self) {
        for input_id in 1..=self.n_inputs {
            self.compute_layers_rec(input_id);
        }
    }

    /// sets the layer for one node only
    fn compute_layers_rec(&mut self, id: u32) {
        // remove the node so that we don't have to fetch it each time
        // this won't be a problem since we only go deeper into the layers
        let node = self.nodes.remove(&id).unwrap();
        let layer = node.layer;
        for succ in &node.succ {
            let next_layer = &mut self.nodes.get_mut(&succ.to).unwrap().layer;
            if *next_layer <= layer {
                *next_layer = layer + 1;
                self.compute_layers_rec(succ.to);
            }
        }
        // add the node back to the map
        self.nodes.insert(id, node);
    }

    /// Creates the hidden nodes and links all the nodes using the genome
    fn build_network(mut self) -> Self {
        for i in 0..self.genome.len() {
            if !self.genome[i].enabled {
                continue;
            }

            let from = self.genome[i].from;
            let to = self.genome[i].to;
            let weight = self.genome[i].weight;
            self.get_or_create_node(from).add_link_to(to);
            self.get_or_create_node(to).add_link_from(from, weight);
        }
        self
    }

    fn get_node(&self, id: u32) -> Option<&Node> {
        // TODO improve:
        // check if node ids are naturally sorted in ascending order
        // and return None accordingly
        self.nodes.get(&id)
    }

    fn get_node_mut(&mut self, id: u32) -> Option<&mut Node> {
        self.nodes.get_mut(&id)
    }

    fn get_or_create_node(&mut self, id: u32) -> &mut Node {
        self.nodes
            .entry(id)
            .or_insert_with(|| Node::new(NodeType::HIDDEN, None))
    }

    /**
    Computes the outputs using the network's inputs
    We start by using computing the output nodes, and recursively computing everything else
    */
    pub fn compute(&mut self) {
        for id in self.n_inputs + 1..=self.n_inputs + self.n_outputs {
            self.compute_rec(id, None);
        }
    }

    pub fn compute_rec(&mut self, id: u32, compute_it: Option<u32>) {
        // remove the node so that we don't have to fetch it each time
        // this won't be a problem since we go from the highest layer to the lowest only
        let mut node = self.nodes.remove(&id).unwrap();
        let compute_iteration = compute_it.unwrap_or(node.compute_iteration + 1);

        // no need to recompute (or is an input/bias), we use the value stored in the node
        if compute_iteration > node.compute_iteration && !node.pred.is_empty() {
            for succ in &node.succ {
                if self.nodes.get(&succ.to).unwrap().compute_iteration < compute_iteration {
                    self.compute_rec(succ.to, compute_it);
                }
                node.value += self.nodes.get(&succ.to).unwrap().value;
            }
            node.compute_iteration = compute_iteration;
            node.value = sigmoid(node.value);
        }
        // add the node back to the map
        self.nodes.insert(id, node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_be_built() {
        let network = Network::new(5, 5, None);
        // 5 input + 5 output + 1 bias
        assert_eq!(network.nodes.len(), 11);
        assert_eq!(network.nodes[&0].node_type, NodeType::BIAS);
        assert_eq!(network.nodes[&1].node_type, NodeType::INPUT);
        assert_eq!(network.nodes[&2].node_type, NodeType::INPUT);
        assert_eq!(network.nodes[&3].node_type, NodeType::INPUT);
        assert_eq!(network.nodes[&4].node_type, NodeType::INPUT);
        assert_eq!(network.nodes[&5].node_type, NodeType::INPUT);
        assert_eq!(network.nodes[&6].node_type, NodeType::OUTPUT);
        assert_eq!(network.nodes[&7].node_type, NodeType::OUTPUT);
        assert_eq!(network.nodes[&8].node_type, NodeType::OUTPUT);
        assert_eq!(network.nodes[&9].node_type, NodeType::OUTPUT);
        assert_eq!(network.nodes[&10].node_type, NodeType::OUTPUT);
    }

    #[test]
    fn genome_test() {
        let genome = Network::build_genome(5, 5);
        let mut i = 0;
        for from in 1..=5 {
            for to in 6..=10 {
                assert_eq!(genome[i].from, from);
                assert_eq!(genome[i].to, to);
                i += 1;
            }
        }
    }
}
