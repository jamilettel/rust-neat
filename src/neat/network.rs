use crate::neat::Gene;
use crate::neat::Node;

/**
Network represents an individual, a network of nodes.
*/
pub struct Network<'a> {
    pub inputs: Vec<Node<'a>>,
    hidden: Vec<Node<'a>>,
    pub outputs: Vec<Node<'a>>,
    genome: Vec<Gene>,
}

impl<'a> Network<'a> {
    /// Creates a new Network using the genome
    pub fn new(inputs: u32, outputs: u32, genome: Vec<Gene>) -> Self {
        Network {
            inputs: Vec::new(),
            hidden: Vec::new(),
            outputs: Vec::new(),
            genome,
        }
        .build_nodes(inputs, outputs)
        .build_links()
    }

    /// Creates the nodes of the network
    fn build_nodes(mut self, inputs: u32, outputs: u32) -> Self {
        for i in 0..=inputs {
            self.inputs.push(Node::new(i, Some(0)));
        }
        for i in inputs..=outputs {
            self.outputs.push(Node::new(i, None));
        }
        for i in 0..self.genome.len() {
            if let Some(_) = self.get_node(self.genome[i].from) {
                continue;
            }
            self.hidden.push(Node::new(self.genome[i].from, None));
        }
        self
    }

    fn build_links(mut self) -> Self {
        self
    }

    fn get_node(&'a self, id: u32) -> Option<&'a Node<'a>> {
        // TODO maybe improve:
        // check if node ids are naturally sorted in ascending order
        // and return None accordingly
        for node in &self.inputs {
            if node.get_id() == id {
                return Some(node);
            }
        }
        for node in &self.hidden {
            if node.get_id() == id {
                return Some(node);
            }
        }
        for node in &self.hidden {
            if node.get_id() == id {
                return Some(node);
            }
        }
        None
    }

    /// Computes the outputs using the network's inputs
    pub fn compute(&mut self) {
        todo!();
    }
}
