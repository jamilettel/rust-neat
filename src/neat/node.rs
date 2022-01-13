use crate::neat::Link;
use std::fmt;

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
pub struct Node {
    id: u32,
    /// All nodes x such that the link self --> x exists
    succ: Vec<Link>,
    prec: Vec<Link>,

    pub value: f64,

    pub layer: i32,
}

impl fmt::Display for Node {
    // Allows us to print the Node with the default formatter
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Node: {}]", self.id)
    }
}

impl PartialEq for Node {
    // Defines equality operation between nodes
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Node {} // Do not remove

impl Node {
    pub fn new(id: u32, layer: Option<i32>) -> Self {
        return Node {
            id,
            succ: Vec::new(),
            prec: Vec::new(),
            value: 0.0,
            layer: layer.unwrap_or(0),
        };
    }

    // /// Adds a new successor
    // pub fn add_succ(&mut self, new_succ: *mut Node, weight: f64) {
    //     self.succ.push(Link::new(new_succ, weight));
    // }
    /// Links two nodes together
    pub fn link(from: &mut Node, to: &mut Node, weight: f64) {
        from.succ.push(Link::new(to, weight));
        to.prec.push(Link::new(from, weight))
    }

    /// Returns a reference to the vector of successors of the node
    pub fn get_succ(&self) -> &Vec<Link> {
        &self.succ
    }

    /// Get the node's id.
    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn set_layers(&mut self) {
        for next in &mut self.succ {
            unsafe {
                if (*next.node).layer <= self.layer {
                    (*next.node).layer = self.layer + 1;
                    (*next.node).set_layers();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() {
        // succ
        // let l: Link = Link {
        //     src: &n,
        //     dst: &n,
        //     weight: 69.0,
        // };
        // let g: Gene = Gene {
        //     id: 3,
        //     link: &l,
        //     enabled: true,
        // };
    }

    #[test]
    fn can_be_built() {
        let _n: Node = Node {
            id: 1,
            succ: Vec::new(),
            prec: Vec::new(),
            value: 0.1,
            layer: 1,
        };
        assert!(false);
    }
}
