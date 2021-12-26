use std::fmt;


// enum NodeType {}

pub struct Node {
    id: u32,
    label: String,
    // All nodes x such that the link x --> self exists
    pred: Vec<&'static Node>, // List of references to the predecessors of the Node, 
    
    // All nodes x such that the link self --> x exists
    succ: Vec<&'static Node>,// List of references to the successors of the Node, 
    
    input : f64, // input value of the node 
    output : f64, // output value of the node
    
    // node_type : If useful we can crete an Enum for Input, Output,  
    
}


impl fmt::Display for Node { 
    // Allows us to print the Node with the default formatter 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node {} : {} ", self.id, self.label)
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

    pub fn new(id: u32, label: String, pred: Vec<&'static Node>, succ: Vec<&'static Node>) -> Node {
        return Node { 
            id, 
            label, 
            pred,
            succ, 
            input: 0,
            output: 0   
        }
    }

    /// Adds a new predecessor, returns true if successful false otherwise
    pub fn add_pred(&mut self, new_pred: &'static Self) -> bool {
        if ! (self.pred.contains(&new_pred) || self.succ.contains(&new_pred)) {
            self.pred.push(new_pred);
            return true;
        }
        false
    }
    
    /// Adds a new successor, returns true if successful false otherwise
    pub fn add_succ(&mut self, new_succ: &'static Self) -> bool {
        if ! (self.pred.contains(&new_succ) || self.succ.contains(&new_succ)) {
            self.succ.push(new_succ);
            return true;
        }
        false
    }

    /// Returns a reference to the vector of predecessors of the node 
    pub fn get_pred(&self) -> &Vec<&Node> { &self.pred }

    /// Returns a reference to the vector of successors of the node
    pub fn get_succ(&self) -> &Vec<&Node> { &self.succ }

    pub fn get_input(&self) -> f64 { &self.input }

    pub fn get_output(&self) -> f64 { &self.output }
    
    /// ...... and returns the input
    pub fn update_input(&mut self) -> f64 {
        // TODO
    }

    /// ...... and returns the output    
    pub fn update_output(&mut self) -> f64 {
        // TODO
    }


    pub fn delete_pred(&mut self, pred: &Self) -> bool {
        for (index, &p) in self.pred.iter().enumerate() {
            if p == pred {
                self.pred.swap_remove(index)
                return true
            }
        }
        false
    }


    // TODO :  delete_succ, clone, update_input, update_output, drop
    
}
