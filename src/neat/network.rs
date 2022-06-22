use super::sigmoid;
use super::Genome;
use super::Node;
use super::NodeType;
use std::collections::HashMap;

/// Network represents an individual, a network of nodes.
pub struct Network {
    pub nodes: HashMap<u32, Node>,
    n_inputs: u32,
    n_outputs: u32,
}

impl Clone for Network {
    fn clone(&self) -> Self {
        let mut map: HashMap<u32, Node> = HashMap::new();
        map.clone_from(&self.nodes);

        Network {
            n_inputs: self.n_inputs,
            n_outputs: self.n_outputs,
            nodes: map,
        }
    }
}

impl Network {
    /// Creates a new Network using the genome
    pub fn new(genome: &Genome, n_inputs: u32, n_outputs: u32) -> Self {
        let mut network = Network {
            nodes: HashMap::new(),
            n_inputs,
            n_outputs,
        }
        .build(genome);
        network.compute_layers();
        network
    }

    /// Builds the inputs and outputs, and then the rest of the network using the genome
    fn build(self, genome: &Genome) -> Self {
        self.build_inputs_outputs(genome).build_network(genome)
    }

    /// Creates the nodes of the network
    fn build_inputs_outputs(mut self, genome: &Genome) -> Self {
        self.nodes.reserve((genome.n_nodes) as usize);
        self.nodes.insert(0, Node::new(NodeType::BIAS, Some(0)));
        // bias node's value is always set to 1
        self.nodes.get_mut(&0).unwrap().value = 1.0;
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
        for input_id in 0..=self.n_inputs {
            self.compute_layers_rec(input_id);
        }
    }

    /// sets the layer for one node only
    fn compute_layers_rec(&mut self, id: u32) {
        // remove the node so that we don't have to fetch it each time
        // this won't be a problem since we only go deeper into the layers
        if let Some(node) = self.nodes.remove(&id) {
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
    }

    /// Creates the hidden nodes and links all the nodes using the genome
    fn build_network(mut self, genome: &Genome) -> Self {
        for i in 0..genome.genes.len() {
            if !genome.genes[i].enabled {
                continue;
            }

            let from = genome.genes[i].from;
            let to = genome.genes[i].to;
            let weight = genome.genes[i].weight;
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
            node.value = 0.0;
            for pred in &node.pred {
                if self.nodes.get(&pred.from).unwrap().compute_iteration < compute_iteration {
                    self.compute_rec(pred.from, compute_it);
                }
                let pred_value = self.nodes.get(&pred.from).unwrap().value;
                node.value += pred_value * pred.weight;
            }
            node.compute_iteration = compute_iteration;
            node.value = sigmoid(node.value);
        }
        // add the node back to the map
        self.nodes.insert(id, node);
    }

    pub fn set_inputs(&mut self, inputs: Vec<f64>) {
        for i in 1..=self.n_inputs {
            self.nodes.get_mut(&i).unwrap().value = inputs[(i - 1) as usize];
        }
    }

    pub fn get_outputs(&self) -> Vec<f64> {
        let mut outputs: Vec<f64> = Vec::new();
        for i in self.n_inputs + 1..=self.n_inputs + self.n_outputs {
            outputs.push(self.nodes[&i].value);
        }
        outputs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_nodes_on_build() {
        let genome = Genome::new(0, 5, 5);
        let network = Network::new(&genome, 5, 5);
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
}
