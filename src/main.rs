mod neat;
use neat::Gene;
/// Main for testing purposes only
fn main(){

    let g: Gene = Gene {
        historical_marking: 0,
        enabled: true,
        from: 1,
        to: 2,
        weight: 0.0,
    };

    println!("{}", g);
}
