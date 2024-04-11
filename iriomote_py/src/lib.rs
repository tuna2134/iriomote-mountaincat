use pyo3::prelude::*;
mod ai;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn iriomote_py(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ai::IriomotePy>()?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

    m.add(
        "InitializationError",
        py.get_type::<ai::InitializationError>(),
    )?;
    m.add("ExecutionError", py.get_type::<ai::ExecutionError>())?;
    Ok(())
}
