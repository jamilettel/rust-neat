use std::fmt;

/**
Gene is a structural mutation that happened to a given 
*/
#[derive(Clone)]
pub struct Gene {
    pub historical_marking: u32,
    pub enabled: bool,
    pub from: u32,
    pub to: u32,
    pub weight: f64,
}

impl fmt::Display for Gene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[Gene {} ({}): from: {} -- w: {} --> to: {}]",
            self.historical_marking,
            if self.enabled { "enabled" } else { "disabled" },
            self.from,
            self.weight,
            self.to,
        )
    }
}
