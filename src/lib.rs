use pyo3::prelude::*;
mod core;
mod dynamics;
mod measures;

#[pymodule]
fn rusthypergraph(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_class::<core::hypergraph::Hypergraph>()?;
    m.add_class::<core::hypergraph_wrapp::Hypergraph>()?;
    m.add_wrapped(wrap_pyfunction!(measures::degree_wrapp::degree))?;
    m.add_wrapped(wrap_pyfunction!(measures::degree_wrapp::degree_sequence))?;
    m.add_wrapped(wrap_pyfunction!(measures::degree_wrapp::degree_correlation))?;
    m.add_wrapped(wrap_pyfunction!(measures::degree_wrapp::degree_distribution))?;
    Ok(())
}