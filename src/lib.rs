use pyo3::prelude::*;

pub mod neat;

// Formats the sum of two numbers as string.
#[pyfunction]
fn test(a: usize, b: usize) -> PyResult<String> {
    println!("Hello");
    Ok((a + b).to_string())
}

#[pymodule]
fn rust_neat(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test, m)?)?;
    m.add_class::<neat::NEAT>()?;

    Ok(())
}

mod link;
mod node;
mod gene;
use crate::node::Node;
use crate::link::Link;
use crate::gene::Gene;

fn main(){

    let n : Node = Node::new(1, String::from(""), Vec::new(), Vec::new());
    // let ref_n: &'static Node = &n;
    let l : Link = Link {
        src: &n,
        dst: &n,
        weight: 69.0
    };
    let g : Gene = Gene {
        id: 3,
        link: &l,
        enabled: true,
    };

    println!("{}", g);
}
