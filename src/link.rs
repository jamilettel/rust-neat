use std::fmt;

// mod node;
use node::Node;

/// ### Defines a directional Link in the neural network
/// Instance can be created using the constructor or not
/// 
/// Attributes (all public) :
/// 
/// - **src** :     Reference to Source Node
/// - **dst** :     Reference to Destination Node
/// - **weight** :  Weight of the Link\
/// 
pub struct Link<'a>{

    pub src: &'a Node,   
    pub dst: &'a Node, 
    pub weight: f64, 
    
}

/// Allows us to print the Link with the default formatter 
impl<'a> fmt::Display for Link<'a> { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {} -- {} --> {} ", self.src, self.weight, self.dst)
    }
}

impl<'a> PartialEq for Link<'a> {
    // Defines equality operation between links
    fn eq(&self, other: &Self) -> bool {
        self.src == other.src && self.dst == other.dst && self.weight == other.weight
    }
}

impl Eq for Link<'a> {} // Do not remove


impl<'a> Link<'a> {

    /// (Optional) Creates and returns a new Link with specified arguments as attributes
    pub fn new(src: &'a Node, dst: &'a Node, weight: f64) -> Link<'a> {
        return Link {
            src, 
            dst, 
            weight
        }
    }
}
