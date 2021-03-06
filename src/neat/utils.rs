pub struct Settings {
    pub sigmoid_steepness: f64,
    pub small_genome_size: u32,
    pub similarity_c1: f64,
    pub similarity_c2: f64,
    pub similarity_c3: f64,
    pub max_difference: f64,
    /// chance of a weight being reassigned during a mutation
    pub w_mut_reassign: f64,
    /// Max value of reassigned weights
    pub w_mut_reassign_max: f64,
    /// Max uniform perturbation when mutating weights
    pub w_mut_change_max: f64,
}

pub static mut SETTINGS: Settings = Settings {
    sigmoid_steepness: 4.9,
    small_genome_size: 19,
    similarity_c1: 1.0,
    similarity_c2: 1.0,
    similarity_c3: 0.4,
    max_difference: 3.0,
    w_mut_reassign: 0.1,
    w_mut_reassign_max: 1.0,
    w_mut_change_max: 0.3,
};

pub fn sigmoid(x: f64) -> f64 {
    unsafe {
        1.0 / (1.0 + f64::exp(-SETTINGS.sigmoid_steepness * x))
    }
}
