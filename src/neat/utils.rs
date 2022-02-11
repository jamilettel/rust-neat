pub struct Settings {
    pub sigmoid_steepness: f64,
    pub small_genome_size: u32,
    pub similarity_c1: f64,
    pub similarity_c2: f64,
    pub similarity_c3: f64,
    pub max_difference: f64,
}

pub static mut SETTINGS: Settings = Settings {
    sigmoid_steepness: 4.9,
    small_genome_size: 19,
    similarity_c1: 1.0,
    similarity_c2: 1.0,
    similarity_c3: 0.4,
    max_difference: 3.0,
};

pub fn sigmoid(x: f64) -> f64 {
    unsafe {
        1.0 / (1.0 + f64::exp(-SETTINGS.sigmoid_steepness * x))
    }
}
