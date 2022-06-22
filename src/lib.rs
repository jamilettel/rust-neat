mod neat;
use pyo3::prelude::*;

#[pymodule]
fn rust_neat(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<neat::NEAT>()?;
    m.add_class::<neat::Genome>()?;

    Ok(())
}
