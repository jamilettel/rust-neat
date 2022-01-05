use std::fmt;

use crate::Link;

/// ### Defines a node in the Neural Network
///
/// Attributes :
///
/// - **id**
/// - **label** (public)
/// - **pred** : list of predecessors
/// - **succ** : list of successors
/// - **input** : input value of the Node
/// - **output** : output value of the Node
///
pub struct Node<'a> {
    id: u32,

    /// All nodes x such that the link x --> self exists
    pred: Vec<&'a Link<'a>>,
    /// All nodes x such that the link self --> x exists
    succ: Vec<&'a Link<'a>>,

    pub input: f64,
    pub output: f64,

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
        pred: Vec<&'a Link<'a>>,
        succ: Vec<&'a Link<'a>>,
        layer: Option<i32>,
    ) -> Self {
        return Node {
            id,
            pred,
            succ,
            input: 0.0,
            output: 0.0,
            layer: layer.unwrap_or(0),
        };
    }

    /// Adds a new predecessor, returns true if successful false otherwise
    pub fn add_pred(&mut self, new_pred: &'a Link) -> bool {
        self.pred.push(new_pred);
        true
    }

    /// Adds a new successor, returns true if successful false otherwise
    pub fn add_succ(&mut self, new_succ: &'a Link) -> bool {
        self.succ.push(new_succ);
        true
    }

    /// Returns a reference to the vector of predecessors of the node
    pub fn get_pred(&self) -> &Vec<&'a Link> {
        &self.pred
    }

    /// Returns a reference to the vector of successors of the node
    pub fn get_succ(&self) -> &Vec<&'a Link> {
        &self.succ
    }
}

impl<'a> Clone for Node<'a> {
    fn clone(&self) -> Self {
        Node::new(
            self.id,
            self.pred.clone(),
            self.succ.clone(),
            Some(self.layer),
        )
    }
}
