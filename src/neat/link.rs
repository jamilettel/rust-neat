use crate::neat::Node;

pub struct Link<'a> {
    pub dst: &'a Node<'a>,
    pub weight: f64,
}

impl<'a> Link<'a> {
    /// (Optional) Creates and returns a new Link with specified arguments as attributes
    pub fn new(dst: &'a Node<'a>, weight: f64) -> Self {
        return Link { dst, weight };
    }
}
