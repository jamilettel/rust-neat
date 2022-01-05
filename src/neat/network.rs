use crate::neat::Gene;
use crate::neat::Link;
use crate::neat::Node;

/**
Network represents an individual, a network of nodes.
*/
pub struct Network<'a> {
    pub inputs: Vec<Node<'a>>,
    hidden: Vec<Node<'a>>,
    pub outputs: Vec<Node<'a>>,
    links: Vec<Link<'a>>,
    genome: Vec<Gene>,
}

impl<'a> Network<'a> {
    /// Creates a new Network using the genome
    pub fn new(genome: Vec<Gene>) -> Self {
        Network {
            inputs: Vec::new(),
            hidden: Vec::new(),
            outputs: Vec::new(),
            links: Vec::new(),
            genome,
        }
        .build()
    }

    /// Builds the network using the genome
    fn build(self) -> Self {
        todo!();
    }

    pub fn compute() {
        todo!();
    }
}
