use crate::neat::Genome;
use crate::neat::Gene;

mod neat;

// TODO remove main
/// Main for testing purposes only
fn main(){
    for _ in 0..10 {
        let mut g1 = Genome::new(0, 5, 5);

        for i in 117..200 {
            g1.genes.push(Gene {
                enabled: true,
                from: 0,
                to: i,
                hm: i,
                weight: 0.1,
            });

            g1.genes.push(Gene {
                enabled: true,
                from: i,
                to: 7,
                hm: i * 2,
                weight: 0.1,
            });
        }

        let nodes = g1.get_linkable_nodes(None);
        println!("Is some: {}", nodes.is_some());
        let nodes = nodes.unwrap();
        println!("Node 0: {}, Node 1: {}", nodes.0, nodes.1);
        let layer1 = g1.network.as_ref().unwrap().nodes[&nodes.0].layer;
        let layer2 = g1.network.as_ref().unwrap().nodes[&nodes.1].layer;
        assert!(layer1 <= layer2);
    }
}
