use std::fmt;

mod node;
use node::Node;

pub struct Link<'a>{
    // Defines a directional Link in the neural network
    src: &'a Node, // Source Node
    dst: &'a Node, // Destination Node
    weight: f64, // Weight of the Link
  
} 


/// Allows us to print the Link with the default formatter 
impl<'a> fmt::Display for Link<'a> { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {} -- {} --> {} ", self.src, self.weight, self.dst)
    }
}

impl<'a> Link<'a> {

    /// Creates and returns a new Link with specified arguments as attributes
    pub fn new(src: &'a Node, dst: &'a Node, weight: f64) -> Link<'a> {
        return Link {
            src, 
            dst, 
            weight
        }
    }
  

    pub fn get_weight(&self) -> f64 { self.weight }
        
    pub fn get_src(&self) -> &Node { self.src }

    pub fn get_dst(&self) -> &Node { self.dst }

    pub fn set_weight(&mut self, new_weight: f64) -> bool { 
        self.weight = new_weight;
        true
    }

    pub fn set_src(&mut self, new_src: &'a Node) -> bool {
        self.src = new_src;
        true
    }

    pub fn set_dst(&mut self, new_dst: &'a Node) -> bool {
        self.dst = new_dst;
        true
    }

}
