use crate::neat::Node;

pub struct Link {
    pub node: *mut Node,
    pub weight: f64,
}

impl Link {
    /// (Optional) Creates and returns a new Link with specified arguments as attributes
    pub fn new(node: *mut Node, weight: f64) -> Self {
        return Link { node, weight };
    }
}
