use crate::neat::Gene;
use crate::neat::Node;
use std::cell::UnsafeCell;

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
        let mut network = Network {
            inputs: Vec::new(),
            hidden: Vec::new(),
            outputs: Vec::new(),
            genome,
        }
        .build(inputs, outputs);
        network.compute_layers();
        network
    }

    /// Builds the inputs and outputs, and then the rest of the network using the genome
    fn build(self, inputs: u32, outputs: u32) -> Self {
        self.build_inputs_outputs(inputs, outputs).build_network()
    }

    /// Creates the nodes of the network
    fn build_inputs_outputs(mut self, inputs: u32, outputs: u32) -> Self {
        self.inputs.reserve(inputs as usize);
        self.outputs.reserve(outputs as usize);
        for i in 0..=inputs {
            self.inputs.push(UnsafeCell::new(Node::new(i, Some(0))));
        }
        for i in inputs..=outputs {
            self.outputs.push(UnsafeCell::new(Node::new(i, None)));
        }
        self
    }

    /// Recursively sets the layers on the nodes in the network
    fn compute_layers(&mut self) {
        for input in &mut self.inputs {
            input.get_mut().set_layers();
        }
    }

    /// Creates the hidden nodes and links all the nodes using the genome
    fn build_network(mut self) -> Self {
        for i in 0..self.genome.len() {
            let from = self.get_or_create_node(self.genome[i].from);
            let to = self.get_or_create_node(self.genome[i].to);

            unsafe {
                (*from).add_succ(to, self.genome[i].weight);
            }
        }
        self
    }

    /// Creates and adds a hidden node in the network
    fn create_hidden_node(&mut self, id: u32, layer: Option<i32>) -> *mut Node {
        self.hidden.push(UnsafeCell::new(Node::new(id, layer)));
        self.hidden.last().unwrap().get()
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

    #[inline(always)]
    fn get_or_create_node(&mut self, id: u32) -> *mut Node {
        self.get_node(id).unwrap_or_else(|| {
            self.hidden.push(UnsafeCell::new(Node::new(id, None)));
            self.hidden.last().unwrap().get()
        })
    }

    /// Computes the outputs using the network's inputs
    pub fn compute(&mut self) {
        todo!();
    }
}
