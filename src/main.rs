
mod link;
mod node;
mod gene;
use crate::node::Node;
use crate::link::Link;
use crate::gene::Gene;

/// Main for testing purposes only
fn main(){

    let n: Node = Node::new(1, Vec::new(), Vec::new(), None);
    let l: Link = Link {
        src: &n,
        dst: &n,
        weight: 69.0
    };
    let g: Gene = Gene {
        id: 3,
        link: &l,
        enabled: true,
    };

    println!("{}", g);
}
