use pyo3::prelude::*;
mod core;
mod dynamics;
mod measures;

#[pymodule]
fn rusthypergraph(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_class::<core::hypergraph::Hypergraph>()?;
    m.add_class::<core::hypergraph_wrapp::Hypergraph>()?;
    m.add_wrapped(wrap_pyfunction!(measures::measures_wrapp::degree))?;
    m.add_wrapped(wrap_pyfunction!(measures::measures_wrapp::degree_sequence))?;
    m.add_wrapped(wrap_pyfunction!(measures::measures_wrapp::degree_correlation))?;
    m.add_wrapped(wrap_pyfunction!(measures::measures_wrapp::degree_distribution))?;
    m.add_wrapped(wrap_pyfunction!(measures::measures_wrapp::intersection))?;
    m.add_wrapped(wrap_pyfunction!(measures::measures_wrapp::jaccard_similarity))?;
    m.add_wrapped(wrap_pyfunction!(measures::measures_wrapp::jaccard_distance))?;
    m.add_wrapped(wrap_pyfunction!(measures::measures_wrapp::cec_centrality))?;
    m.add_wrapped(wrap_pyfunction!(measures::measures_wrapp::zec_centrality))?;
    m.add_wrapped(wrap_pyfunction!(measures::measures_wrapp::hec_centrality))?;
    m.add_wrapped(wrap_pyfunction!(measures::measures_wrapp::s_betweenness))?;
    m.add_wrapped(wrap_pyfunction!(measures::measures_wrapp::s_closeness))?;
    Ok(())
}