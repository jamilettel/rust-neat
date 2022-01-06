use std::fmt;

use crate::neat::Node;

/**
### Defines a directional Link in the neural network
Instance can be created using the constructor or not

Attributes (all public):
- **src**:     Reference to Source Node
- **dst**:     Reference to Destination Node
- **weight**:  Weight of the Link

*/
pub struct Link<'a> {
    pub dst: &'a Node<'a>,
    pub weight: f64,
}

// /// Allows us to print the Link with the default formatter
// impl<'a> fmt::Display for Link<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "[Link: {} -- {} --> {}]", self.src, self.weight, self.dst)
//     }
// }

// impl<'a> PartialEq for Link<'a> {
//     /// Defines equality operation between links
//     fn eq(&self, other: &Self) -> bool {
//         self.src == other.src && self.dst == other.dst && self.weight == other.weight
//     }
// }

// impl<'a> Eq for Link<'a> {} // Do not remove

// impl<'a> Clone for Link<'a> {
//     fn clone(&self) -> Self {
//         Link::new(self.src, self.dst, self.weight)
//     }
// }

impl<'a> Link<'a> {
    /// (Optional) Creates and returns a new Link with specified arguments as attributes
    pub fn new(dst: &'a Node<'a>, weight: f64) -> Self {
        return Link { dst, weight };
    }
}