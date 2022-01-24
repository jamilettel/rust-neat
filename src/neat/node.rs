use super::sigmoid;
use super::LinkFrom;
use super::LinkTo;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum NodeType {
    BIAS,
    INPUT,
    HIDDEN,
    OUTPUT,
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Self::BIAS => "Bias",
                Self::INPUT => "Input",
                Self::HIDDEN => "Hidden",
                Self::OUTPUT => "Output",
            }
        )
    }
}

/**
# Defines a node in the Neural Network
*/
pub struct Node {
    pub node_type: NodeType,
    pub succ: Vec<LinkTo>,
    pub pred: Vec<LinkFrom>,
    pub value: f64,
    pub layer: i32,
    pub compute_iteration: u32,
}

impl fmt::Display for Node {
    // Allows us to print the Node with the default formatter
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Node of type {}]", self.node_type)
    }
}

impl Node {
    pub fn new(node_type: NodeType, layer: Option<i32>) -> Self {
        return Node {
            node_type,
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
        self.pred.push(LinkFrom {
            from: node_id,
            weight,
        });
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
            node_type: NodeType::BIAS,
            succ: Vec::new(),
            pred: Vec::new(),
            value: 0.1,
            layer: 1,
            compute_iteration: 0,
        };
        assert!(true);
    }
}
