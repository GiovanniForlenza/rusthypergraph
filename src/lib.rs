use pyo3::prelude::*;
//use pyo3::wrap_pyfunction;
mod generation_lib;
mod core;

#[pymodule]
fn rusthypergraph(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<core::hypergraph::Hypergraph>()?;
    m.add_class::<core::label_encoder::LabelEncoder>()?;
    // m.add_function(wrap_pyfunction!(generation_lib::activity_driven::hoad_model, m)?)?;
    // m.add_function(wrap_pyfunction!(generation_lib::random_module::generate_hypergraph, m)?)?;
    // m.add_function(wrap_pyfunction!(generation_lib::random_module::add_random_edge, m)?)?;
    // m.add_function(wrap_pyfunction!(generation_lib::random_module::add_random_edges, m)?)?;
    Ok(())
}
