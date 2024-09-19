use pyo3::prelude::*;
//use pyo3::wrap_pyfunction;
mod core;
mod dynamics;

#[pymodule]
fn rusthypergraph(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<core::label_encoder::LabelEncoder>()?;
    m.add_class::<core::hypergraph::Hypergraph>()?;
    m.add_function(wrap_pyfunction!(dynamics::randwalk::random_walk, m)?)?;
    Ok(())
}
