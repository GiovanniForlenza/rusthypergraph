use std::collections::HashMap;
use pyo3::prelude::*;
use crate::core::hypergraph_wrapp::Hypergraph;
use super::degree_rust::*;


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

#[pyfunction]
#[pyo3(signature = (hypergraph, order=None, size=None), name = "degree_sequence")]
pub fn degree_sequence(
    hypergraph: &Hypergraph,
    order: Option<usize>,
    size: Option<usize>,
) -> PyResult<Option<HashMap<usize, u64>>> {
    let hypergraph_rust = &hypergraph.inner;
    degree_sequence_rust(hypergraph_rust, order, size).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error computing degree sequence: {}", e))
    })
}

#[pyfunction]
#[pyo3(signature = (hypergraph), name = "degree_correlation")]
pub fn degree_correlation(hypergraph: &Hypergraph) -> PyResult<Option<Vec<Vec<f64>>>> {
    let hypergraph_rust = &hypergraph.inner;
    Ok(Some(degree_correlation_rust(hypergraph_rust).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error computing degree correlation: {}", e))
    })?))
}

#[pyfunction]
#[pyo3(signature = (hypergraph, order=None, size=None), name = "degree_distribution")]
pub fn degree_distribution(hypergraph: &Hypergraph, order: Option<usize>, size: Option<usize>) -> PyResult<Option<HashMap<u64, i32>>> {
    let hypergraph_rust = &hypergraph.inner;
    degree_distribution_rust(hypergraph_rust, order, size).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error computing degree distribution: {}", e))
    })
}