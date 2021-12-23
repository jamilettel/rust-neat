use pyo3::prelude::*;

// Formats the sum of two numbers as string.
#[pyfunction]
fn test(a: usize, b: usize) -> PyResult<String> {
    println!("Hello");
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn rust_neat(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test, m)?)?;

    Ok(())
}
