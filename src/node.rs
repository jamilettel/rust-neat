

// enum NodeType {}

pub struct Node {
    id: u32,
    label: str,
    pred: Vec<Node>, // List of predecessors of the Node, 
    // All nodes x such that the link x --> self exists
    succ: Vec<Node>,// List of successors of the Node, 
    // All nodes x such that the link self --> x exists
    
    // node_type : If useful we can crete an Enum for Input, Output,  
    
    input : u32, // input value of the node 
    output : u32, // output value of the node
    
}


impl fmt::Display for Node { 
    // Allows us to print the Node with the default formatter 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {} -- {} --> {} ", self.src, self.weight, self.dst)
    }
}

impl PartialEq for Node {
    // Defines equality operation between nodes
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.label == other.label
    }
}

impl Eq for Node {} // Do not remove


impl Node {
    pub fn add_pred(&self, new_pred: &Self) -> bool {
        if ! (self.pred.contains(&new_pred) || self.succ.contains(&new_pred)) {
            self.pred.push(new_pred);
            return True;
        }
        False
    }
    
    // TODO : add_succ, delete_pred, delete_succ, clone, update_input, update_output
    
}
