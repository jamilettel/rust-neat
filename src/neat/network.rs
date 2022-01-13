use std::cell::UnsafeCell;
use crate::neat::Gene;
use crate::neat::Node;

/**
Network represents an individual, a network of nodes.
*/
pub struct Network {
    pub inputs: Vec<UnsafeCell<Node>>,
    hidden: Vec<UnsafeCell<Node>>,
    pub outputs: Vec<UnsafeCell<Node>>,
    genome: Vec<Gene>,
}

impl Network {
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
            self.inputs.push(UnsafeCell::new(Node::new(i, Some(0))));
        }
        for i in inputs..=outputs {
            self.outputs.push(UnsafeCell::new(Node::new(i, None)));
        }
        for i in 0..self.genome.len() {
            if let Some(_) = self.get_node(self.genome[i].from) {
                continue;
            }
            self.hidden
                .push(UnsafeCell::new(Node::new(self.genome[i].from, None)));
        }
        self
    }

    fn build_links(mut self) -> Self {
        for i in 0..self.hidden.len() {
            for j in 0..self.outputs.len() {
                self.hidden[i].get_mut().add_succ(self.outputs[j].get(), 0.0);
            }
        }
        for i in 0..self.inputs.len() {
            for j in 0..self.hidden.len() {
            }
        }
        self
    }

    fn get_hidden_node(&self, id: u32) -> Option<*mut Node> {
        for node in &self.hidden {
            unsafe {
                if (*(node.get())).get_id() == id {
                    return Some(node.get());
                }
            }
        }
        None
    }

    fn get_input_node(&self, id: u32) -> Option<*mut Node> {
        for node in &self.inputs {
            unsafe {
                if (*(node.get())).get_id() == id {
                    return Some(node.get());
                }
            }
        }
        None
    }

    fn get_output_node(&self, id: u32) -> Option<*mut Node> {
        for node in &self.outputs {
            unsafe {
                if (*(node.get())).get_id() == id {
                    return Some(node.get());
                }
            }
        }
        None
    }

    fn get_node(&self, id: u32) -> Option<*mut Node> {
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
