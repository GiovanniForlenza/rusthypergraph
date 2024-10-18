use pyo3::prelude::*;
mod core;
mod dynamics;


#[pymodule]
fn rusthypergraph(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<core::hypergraph::Hypergraph>()?;
    Ok(())
}