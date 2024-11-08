use std::collections::{HashMap, HashSet};
use pyo3::prelude::*;
use crate::core::hypergraph_wrapp::Hypergraph;
use super::degree_rust::*;
use super::edge_similarity_rust::*;
use super::eigen_centralities_rust::*;

/// Computes the degree of a node in a hypergraph.
///
/// # Arguments
///
/// * `hypergraph` - A reference to the hypergraph.
/// * `node` - The node for which to compute the degree.
/// * `order` - An optional parameter specifying the order of the degree.
/// * `size` - An optional parameter specifying the size of the degree.
///
/// # Returns
///
/// * The degree of the node as an unsigned 64-bit integer.
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

/// Computes the degree sequence of a hypergraph.
///
/// # Arguments
///
/// * `hypergraph` - A reference to the hypergraph.
/// * `order` - An optional parameter specifying the order of the degree sequence.
/// * `size` - An optional parameter specifying the size of the degree sequence.
///
/// # Returns
///
/// * The degree sequence as a map from node indices to unsigned 64-bit integers.
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

/// Computes the degree correlation of a hypergraph.
///
/// # Arguments
///
/// * `hypergraph` - A reference to the hypergraph.
///
/// # Returns
///
/// * The degree correlation as a 2D vector of floating-point numbers.
#[pyfunction]
#[pyo3(signature = (hypergraph), name = "degree_correlation")]
pub fn degree_correlation(hypergraph: &Hypergraph) -> PyResult<Option<Vec<Vec<f64>>>> {
    let hypergraph_rust = &hypergraph.inner;
    Ok(Some(degree_correlation_rust(hypergraph_rust).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error computing degree correlation: {}", e))
    })?))
}

/// Computes the degree distribution of a hypergraph.
///
/// # Arguments
///
/// * `hypergraph` - A reference to the hypergraph.
/// * `order` - An optional parameter specifying the order of the degree distribution.
/// * `size` - An optional parameter specifying the size of the degree distribution.
///
/// # Returns
///
/// * The degree distribution as a map from unsigned 64-bit integers to signed 32-bit integers.
#[pyfunction]
#[pyo3(signature = (hypergraph, order=None, size=None), name = "degree_distribution")]
pub fn degree_distribution(hypergraph: &Hypergraph, order: Option<usize>, size: Option<usize>) -> PyResult<Option<HashMap<u64, i32>>> {
    let hypergraph_rust = &hypergraph.inner;
    degree_distribution_rust(hypergraph_rust, order, size).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error computing degree distribution: {}", e))
    })
}

/// Computes the intersection of two hyperedges.
///
/// # Arguments
///
/// * `hyperedge_a` - The first hyperedge.
/// * `hyperedge_b` - The second hyperedge.
///
/// # Returns
///
/// * The size of the intersection as an unsigned 32-bit integer.
#[pyfunction]
#[pyo3(name = "intersection")]
pub fn intersection(hyperedge_a: HashSet<usize>, hyperedge_b: HashSet<usize>) -> usize {
    intersection_rust(&hyperedge_a, &hyperedge_b)
}

/// Computes the Jaccard similarity between two hyperedges.
///
/// Jaccard similarity is defined as the size of the intersection divided by the size of the union of the sample sets.
///
/// # Arguments
///
/// * `hyperedge_a` - The first hyperedge.
/// * `hyperedge_b` - The second hyperedge.
///
/// # Returns
///
/// * The Jaccard similarity as a floating-point number.
#[pyfunction]
#[pyo3(name = "jaccard_similarity")]
pub fn jaccard_similarity(hyperedge_a: HashSet<usize>, hyperedge_b: HashSet<usize>) -> f64 {
    jaccard_similarity_rust(&hyperedge_a, &hyperedge_b)
}

/// Computes the Jaccard distance between two hyperedges.
///
/// Jaccard distance is defined as 1 minus the Jaccard similarity.
///
/// # Arguments
///
/// * `hyperedge_a` - The first hyperedge.
/// * `hyperedge_b` - The second hyperedge.
///
/// # Returns
///
/// * The Jaccard distance as a floating-point number.
#[pyfunction]
#[pyo3(name = "jaccard_distance")]
pub fn jaccard_distance(hyperedge_a: HashSet<usize>, hyperedge_b: HashSet<usize>) -> f64 {
    jaccard_distance_rust(&hyperedge_a, &hyperedge_b)
}

#[pyfunction]
#[pyo3(name = "cec_centrality")]
pub fn cec_centrality(hypergraph: &Hypergraph, tol: f64, max_iter: usize) -> PyResult<HashMap<usize, f64>> {
    let hypergraph_rust = &hypergraph.inner;
    cec_centrality_rust(hypergraph_rust, tol, max_iter).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error computing CEC: {}", e))
    })
}

#[pyfunction]
#[pyo3(name = "zec_centrality")]
pub fn zec_centrality(hypergraph: &Hypergraph, tol: f64, max_iter: usize) -> PyResult<HashMap<usize, f64>> {
    let hypergraph_rust = &hypergraph.inner;
    zec_centrality_rust(hypergraph_rust, tol, max_iter).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error computing ZEC: {}", e))
    })
}