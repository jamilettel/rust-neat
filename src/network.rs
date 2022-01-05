use crate::Node;

pub struct Network {
    pub inputs: Vec<Node>,
    hidden: Vec<Node>,
    pub outputs: Vec<Node>,
    links: Vec<Link>,
    genome: Vec<Gene>,
}

impl Network {
    fn new() -> Self {
        Network {
            inputs: [],
            hidden: [],
            outputs: [],
            links: [],
            genome: [],
        }
    }

    fn compute() {
        todo!();
    }

}
