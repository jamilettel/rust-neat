
pub struct LinkFrom {
    pub from: u32,
    pub weight: f64,
}

pub struct LinkTo {
    pub to: u32,
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::cell::UnsafeCell;
//     fn setup() -> (Node, Node) {
//         let n1: Node = Node::new(1, None);
//         let n2: Node = Node::new(1, None);
//         (n1, n2)
//     }

//     #[test]
//     fn can_be_built() {
//         let (n1, n2) = setup();
//         Link::new(UnsafeCell::new(n1).get(), UnsafeCell::new(n2).get(), 0.1);
//     }

//     #[test]
//     #[should_panic]
//     fn cannot_build_loop() {
//         let (n1, _) = setup();
//         let n1_cell = UnsafeCell::new(n1);
//         Link::new(n1_cell.get(), n1_cell.get(), 0.1);
//     }
// }
