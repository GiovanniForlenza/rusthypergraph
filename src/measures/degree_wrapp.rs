use pyo3::prelude::*;
use crate::core::hypergraph_wrapp::Hypergraph;
use super::degree_rust::degree_rust;


#[pyfunction]
#[pyo3(signature = (hypergraph, node, order=None, size=None), name = "degree")]
pub fn degree(
    hypergraph: &Hypergraph,
    node: usize,
    order: Option<usize>,
    size: Option<usize>,
) -> PyResult<u64> {
    let hypergraph_rust = &hypergraph.inner;
    degree_rust(hypergraph_rust, node, order, size).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error computing degree: {}", e))
    })
}

