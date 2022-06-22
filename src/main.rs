use crate::neat::Genome;
use crate::neat::Gene;

mod neat;

// TODO remove main
/// Main for testing purposes only
fn main(){
    // let mut g1 = Genome::new(2, 2);
    // let mut g2 = Genome::new(2, 2);
    let mut g1 = Genome::new(0, 5, 5);

    for i in 17..100 {
        g1.genes.push(Gene {
            enabled: true,
            from: 0,
            to: i,
            hm: 100,
            weight: 0.1,
        });

        g1.genes.push(Gene {
            enabled: true,
            from: i,
            to: 7,
            hm: 100,
            weight: 0.1,
        });
    }

    // g1.build_network();
    // g1.get_network();

    for _ in 0..100 {
        loop {
            let nodes = g1.get_linkable_nodes(None).unwrap();
            if nodes.0 > 6 {
                continue;
            }
            println!("{}, {}", nodes.0, nodes.1);
            break;
        }
    }

    // g2.genes.push(Gene {
    //     enabled: true,
    //     from: 0,
    //     to: 3,
    //     hm: 100,
    //     weight: 1.0,
    // });

    // g1.genes.push(Gene {
    //     enabled: true,
    //     from: 0,
    //     to: 3,
    //     hm: 99,
    //     weight: 1.0,
    // });

    // g1.genes.push(Gene {
    //     enabled: true,
    //     from: 0,
    //     to: 2,
    //     hm: 97,
    //     weight: 1.0,
    // });

    // println!("{}", Genome::compute_difference(&g1, &g2));
    // g1.get_linkable_nodes(None);
}
