use super::LinkFrom;
use super::LinkTo;
use super::sigmoid;
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
    succ: Vec<LinkTo>,
    pred: Vec<LinkFrom>,
    pub value: f64,
    pub layer: i32,
    compute_iteration: u32,
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
            pred: Vec::new(),
            value: 0.0,
            layer: layer.unwrap_or(0),
            compute_iteration: 0,
        };
    }

    pub fn add_link_to(&mut self, node_id: u32) {
        self.succ.push(LinkTo { to: node_id });
    }

    pub fn add_link_from(&mut self, node_id: u32, weight: f64) {
        self.pred.push(LinkFrom { from: node_id, weight });
    }

    /// Returns a reference to the vector of successors of the node
    pub fn get_succ(&self) -> &Vec<LinkTo> {
        &self.succ
    }

    /// Returns a reference to the vector of successors of the node
    pub fn get_pred(&self) -> &Vec<LinkFrom> {
        &self.pred
    }

    /// Get the node's id.
    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn set_layers(&mut self) {
        for next in &mut self.succ {
            unsafe {
                // if (*(*(*next)).dst).layer <= self.layer {
                //     (*(*(*next)).dst).layer = self.layer + 1;
                //     (*(*(*next)).dst).set_layers();
                // }
            }
        }
    }

    pub fn compute(&mut self, compute_iteration: Option<u32>) -> f64 {
        let compute_it = compute_iteration.unwrap_or(self.compute_iteration + 1);
        // Returns value because it has already been computed (or is an input when pred is empty)
        if compute_it <= self.compute_iteration || self.pred.is_empty() {
            return self.value;
        }
        for link in &self.pred {
            // unsafe {
                // self.value += (*(*(*link)).src).compute(Some(compute_it));
            // }
        }
        self.compute_iteration = compute_it;
        self.value = sigmoid(self.value);
        self.value
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
            pred: Vec::new(),
            value: 0.1,
            layer: 1,
            compute_iteration: 0,
        };
        assert!(true);
    }
}
