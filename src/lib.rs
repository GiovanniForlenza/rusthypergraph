use pyo3::prelude::*;
mod core;
mod dynamics;
mod measures;

#[pymodule]
fn rusthypergraph(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_class::<core::hypergraph::Hypergraph>()?;
    m.add_class::<core::hypergraph_wrapp::Hypergraph>()?;
    m.add_wrapped(wrap_pyfunction!(measures::degree_wrapp::degree))?;
    Ok(())
}