use pyo3::prelude::*;
use std::fmt;

/**
Main class, will allow you to train an AI
*/
#[pyclass]
pub struct NEAT {
    pub pop_size: i32,
}

impl fmt::Display for NEAT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[NEAT: pop_size: {}]", self.pop_size)
    }
}

#[pymethods]
impl NEAT {
    #[new]
    #[args(pop_size, test = 1)]
    pub fn new(pop_size: i32) -> Self {
        NEAT { pop_size }
    }

    fn __str__(&self) -> String {
        format!("{}", self)
    }
}
