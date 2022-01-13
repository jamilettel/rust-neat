use crate::neat::Node;

pub struct Link {
    pub dst: *mut Node,
    pub weight: f64,
}

impl Link {
    /// (Optional) Creates and returns a new Link with specified arguments as attributes
    pub fn new(dst: *mut Node, weight: f64) -> Self {
        return Link { dst, weight };
    }
}
