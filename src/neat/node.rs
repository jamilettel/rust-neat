use std::fmt;

use crate::neat::Link;

/**
### Defines a node in the Neural Network

Attributes :

- **id**
- **label** (public)
- **pred** : list of predecessors
- **succ** : list of successors
- **input** : input value of the Node
- **output** : output value of the Node
*/
pub struct Node<'a> {
    id: u32,

    /// All nodes x such that the link self --> x exists
    succ: Vec<Link<'a>>,

    pub value: f64,

    pub layer: i32,
}

impl<'a> fmt::Display for Node<'a> {
    // Allows us to print the Node with the default formatter
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Node: {}]", self.id)
    }
}

impl<'a> PartialEq for Node<'a> {
    // Defines equality operation between nodes
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<'a> Eq for Node<'a> {} // Do not remove

impl<'a> Node<'a> {
    pub fn new(
        id: u32,
        layer: Option<i32>,
    ) -> Self {
        return Node {
            id,
            succ: Vec::new(),
            value: 0.0,
            layer: layer.unwrap_or(0),
        };
    }

    /// Adds a new successor
    pub fn add_succ(mut self, new_succ: &'a Node<'a>, weight: f64) -> Self {
        self.succ.push(Link::new(new_succ, weight));
        self
    }

    /// Returns a reference to the vector of successors of the node
    pub fn get_succ(&self) -> &Vec<Link> {
        &self.succ
    }

    /// Get the node's id.
    pub fn get_id(&self) -> u32 {
        self.id
    }
}