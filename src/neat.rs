#![allow(dead_code)]

mod neat;
pub use neat::NEAT;

mod gene;
pub use gene::Gene;

mod link;
pub use link::Link;

mod network;
pub use network::Network;

mod node;
pub use node::Node;

mod utils;
pub use utils::Settings;
pub use utils::sigmoid;
pub use utils::SETTINGS;
