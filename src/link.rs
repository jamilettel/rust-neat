use std::fmt;

mod node;
use node::Node;

pub struct Link {
    // Defines a directional Link in the neural network
    src: Node, // Source Node
    dst: Node, // Destination Node
    weight: f64, // Weight of the Link
  
} 


impl fmt::Display for Link { 
    // Allows us to print the Link with the default formatter 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {} -- {} --> {} ", self.src, self.weight, self.dst)
    }
}

impl Link {
    fn get_weight(&self) -> f64 {
        self.weight 
    }
        
    fn get_src(&self) -> Node {
        self.src
    }

    fn get_dst(&self) -> Node {
        self.dst
    }
}
