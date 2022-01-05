use pyo3::prelude::*;

pub mod neat;

// Formats the sum of two numbers as string.
#[pyfunction]
fn test(a: usize, b: usize) -> PyResult<String> {
    println!("Hello");
    Ok((a + b).to_string())
}

#[pymodule]
fn rust_neat(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test, m)?)?;
    m.add_class::<neat::NEAT>()?;

    Ok(())
}
