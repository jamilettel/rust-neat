mod neat;
use pyo3::prelude::*;

#[pymodule]
fn rust_neat(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test, m)?)?;
    m.add_class::<neat::NEAT>()?;

    Ok(())
}
