use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
mod generation;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn rusthypergraph(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(generation::hoad_model, m)?)?;
    Ok(())
}
