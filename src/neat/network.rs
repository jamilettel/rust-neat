use crate::neat::Gene;
use crate::neat::Node;

/**
Network represents an individual, a network of nodes.
 */
pub struct Network {
    pub inputs: Vec<Node>,
    hidden: Vec<Node>,
    pub outputs: Vec<Node>,
    /**
    Bias node, always has a value of 1, and is on layer 0 (input layer), with id = 0
    Will not have any links at the start.
     */
    bias: Node,
    genome: Vec<Gene>,
}

impl Network {
    /// Creates a new Network using the genome
    pub fn new(n_inputs: u32, n_outputs: u32, genome: Option<Vec<Gene>>) -> Self {
        let mut network = Network {
            inputs: Vec::new(),
            hidden: Vec::new(),
            outputs: Vec::new(),
            genome: genome.unwrap_or_else(|| Network::build_genome(n_inputs, n_outputs)),
            bias: Node::new(0, Some(0)),
        }
        .build(n_inputs, n_outputs);
        network.compute_layers();
        network.bias.value = 1.0;
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
    fn build(self, n_inputs: u32, n_outputs: u32) -> Self {
        self.build_inputs_outputs(n_inputs, n_outputs)
            .build_network()
    }

    /// Creates the nodes of the network
    fn build_inputs_outputs(mut self, n_inputs: u32, n_outputs: u32) -> Self {
        self.inputs.reserve(n_inputs as usize);
        self.outputs.reserve(n_outputs as usize);
        for i in 1..=n_inputs {
            self.inputs.push(Node::new(i, Some(0)));
        }
        for i in n_inputs + 1..=n_outputs + n_inputs {
            self.outputs.push(Node::new(i, None));
        }
        self
    }

    /// Recursively sets the layers on the nodes in the network
    fn compute_layers(&mut self) {
        for input in &mut self.inputs {
            input.set_layers();
        }
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
            // let from = self.get_or_create_node(self.genome[i].from);
            // let to = self.get_or_create_node(self.genome[i].to);
            // (*from).add_link(link_cell.get());
            // (*to).add_link(link_cell.get());
            // self.links.push(link_cell);
        }
        self
    }

    /// Creates and adds a hidden node in the network
    fn create_hidden_node(&mut self, id: u32, layer: Option<i32>) -> &Node {
        self.hidden.push(Node::new(id, layer));
        self.hidden.last().unwrap()
    }

    fn get_node(&self, id: u32) -> Option<&Node> {
        // TODO improve:
        // check if node ids are naturally sorted in ascending order
        // and return None accordingly
        if id == 0 {
            return Some(&self.bias);
        }
        self.inputs
            .iter()
            .chain(self.hidden.iter())
            .chain(self.outputs.iter())
            .find(|node| node.get_id() == id)
    }

    fn get_node_mut(&mut self, id: u32) -> Option<&mut Node> {
        if id == 0 {
            return Some(&mut self.bias);
        }
        self.inputs
            .iter_mut()
            .chain(self.hidden.iter_mut())
            .chain(self.outputs.iter_mut())
            .find(|node| node.get_id() == id)
    }

    fn get_or_create_node(&mut self, id: u32) -> &mut Node {
        if self.get_node_mut(id) != None {
            self.get_node_mut(id).unwrap()
        } else {
            self.hidden.push(Node::new(id, None));
            self.hidden.last_mut().unwrap()
        }
    }

    /// Computes the outputs using the network's inputs
    pub fn compute(&mut self) {
        for output in &mut self.outputs {
            output.compute(None);
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn can_be_built() {
        // let network = Network::new(5, 5, None);
        // assert_eq!(network.outputs.len(), 5);
        // assert_eq!(network.inputs.len(), 5);
        // unsafe {
        //     assert_eq!((*network.inputs[0].get()).get_id(), 1);
        //     assert_eq!((*network.inputs[1].get()).get_id(), 2);
        //     assert_eq!((*network.inputs[2].get()).get_id(), 3);
        //     assert_eq!((*network.inputs[3].get()).get_id(), 4);
        //     assert_eq!((*network.inputs[4].get()).get_id(), 5);
        //     assert_eq!((*network.outputs[0].get()).get_id(), 6);
        //     assert_eq!((*network.outputs[1].get()).get_id(), 7);
        //     assert_eq!((*network.outputs[2].get()).get_id(), 8);
        //     assert_eq!((*network.outputs[3].get()).get_id(), 9);
        //     assert_eq!((*network.outputs[4].get()).get_id(), 10);
        // }
    }

    #[test]
    fn genome_test() {
        // let genome = Network::build_genome(5, 5);
        // let mut i = 0;
        // for from in 1..=5 {
        //     for to in 6..=10 {
        //         assert_eq!(genome[i].from, from);
        //         assert_eq!(genome[i].to, to);
        //         i += 1;
        //     }
        // }
    }
}
