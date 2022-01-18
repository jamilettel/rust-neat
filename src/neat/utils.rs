pub struct Settings {
    pub sigmoid_steepness: f64,
}

pub static mut SETTINGS: Settings = Settings {
    sigmoid_steepness: 4.9,
};

pub fn sigmoid(x: f64) -> f64 {
    unsafe {
        1.0 / (1.0 + f64::exp(-SETTINGS.sigmoid_steepness * x))
    }
}
