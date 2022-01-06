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

    fn build_links(&'a mut self) {
        for i in 0..self.hidden.len() {
            for j in 0..self.outputs.len() {
                self.hidden[i].add_succ(&self.outputs[j], 0.0);
            }
        }
        for i in 0..self.inputs.len() {
            for j in 0..self.hidden.len() {
                self.inputs[i].add_succ(&self.hidden[j], 0.0);
            }
        }
    }

    fn get_hidden_node(&'a self, id: u32) -> Option<&'a Node<'a>> {
        for node in &self.hidden {
            if node.get_id() == id {
                return Some(node);
            }
        }
        None
    }

    fn get_input_node(&'a self, id: u32) -> Option<&'a Node<'a>> {
        for node in &self.inputs {
            if node.get_id() == id {
                return Some(node);
            }
        }
        None
    }

    fn get_output_node(&'a self, id: u32) -> Option<&'a Node<'a>> {
        for node in &self.outputs {
            if node.get_id() == id {
                return Some(node);
            }
        }
        None
    }

    fn get_node(&'a self, id: u32) -> Option<&'a Node<'a>> {
        // TODO improve:
        // check if node ids are naturally sorted in ascending order
        // and return None accordingly
        self.get_input_node(id)
            .or_else(|| self.get_hidden_node(id))
            .or_else(|| self.get_output_node(id))
    }

    /// Computes the outputs using the network's inputs
    pub fn compute(&mut self) {
        todo!();
    }
}
