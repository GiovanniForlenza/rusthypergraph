use std::collections::{HashMap, HashSet, BTreeMap};
use pyo3::prelude::*;
use crate::core::hypergraph_wrapp::Hypergraph;
use super::degree_rust::*;
use super::edge_similarity_rust::*;
use super::eigen_centralities_rust::*;
use super::s_centralities_rust::*;

/// Python wrapper for computing the degree of a node in a hypergraph.
/// 
/// # Arguments
/// * `hypergraph` - Reference to the Python hypergraph object
/// * `node` - Node index to compute degree for
/// * `order` - Optional order constraint for incident edges
/// * `size` - Optional size constraint for incident edges
/// 
/// # Returns
/// * `PyResult<u64>` - The degree of the node
/// * Raises `PyValueError` if computation fails
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

/// Python wrapper for computing the degree sequence of a hypergraph.
/// 
/// # Arguments
/// * `hypergraph` - Reference to the Python hypergraph object
/// * `order` - Optional order constraint for incident edges
/// * `size` - Optional size constraint for incident edges
/// 
/// # Returns
/// * `PyResult<Option<HashMap<usize, u64>>>` - Map of node indices to their degrees
/// * Raises `PyValueError` if computation fails
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

/// Python wrapper for computing the degree correlation matrix of a hypergraph.
/// 
/// Computes correlations between degree sequences for different edge sizes.
/// 
/// # Arguments
/// * `hypergraph` - Reference to the Python hypergraph object
/// 
/// # Returns
/// * `PyResult<Option<Vec<Vec<f64>>>>` - Matrix of correlation coefficients
/// * Raises `PyValueError` if computation fails
#[pyfunction]
#[pyo3(signature = (hypergraph), name = "degree_correlation")]
pub fn degree_correlation(hypergraph: &Hypergraph) -> PyResult<Option<Vec<Vec<f64>>>> {
    let hypergraph_rust = &hypergraph.inner;
    Ok(Some(degree_correlation_rust(hypergraph_rust).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error computing degree correlation: {}", e))
    })?))
}

/// Python wrapper for computing the degree distribution of a hypergraph.
/// 
/// # Arguments
/// * `hypergraph` - Reference to the Python hypergraph object
/// * `order` - Optional order constraint for incident edges
/// * `size` - Optional size constraint for incident edges
/// 
/// # Returns
/// * `PyResult<Option<HashMap<u64, i32>>>` - Map of degrees to their frequencies
/// * Raises `PyValueError` if computation fails
#[pyfunction]
#[pyo3(signature = (hypergraph, order=None, size=None), name = "degree_distribution")]
pub fn degree_distribution(hypergraph: &Hypergraph, order: Option<usize>, size: Option<usize>) -> PyResult<Option<HashMap<u64, i32>>> {
    let hypergraph_rust = &hypergraph.inner;
    degree_distribution_rust(hypergraph_rust, order, size).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error computing degree distribution: {}", e))
    })
}

/// Python wrapper for computing the intersection size between two hyperedges.
/// 
/// # Arguments
/// * `hyperedge_a` - First hyperedge as a set of node indices
/// * `hyperedge_b` - Second hyperedge as a set of node indices
/// 
/// # Returns
/// * `usize` - Number of nodes that appear in both hyperedges
#[pyfunction]
#[pyo3(name = "intersection")]
pub fn intersection(hyperedge_a: HashSet<usize>, hyperedge_b: HashSet<usize>) -> usize {
    intersection_rust(&hyperedge_a, &hyperedge_b)
}

/// Python wrapper for computing the Jaccard similarity between two hyperedges.
/// 
/// The Jaccard similarity is defined as the size of the intersection divided by
/// the size of the union of the two sets.
/// 
/// # Arguments
/// * `hyperedge_a` - First hyperedge as a set of node indices
/// * `hyperedge_b` - Second hyperedge as a set of node indices
/// 
/// # Returns
/// * `f64` - A value between 0 (completely different) and 1 (identical)
#[pyfunction]
#[pyo3(name = "jaccard_similarity")]
pub fn jaccard_similarity(hyperedge_a: HashSet<usize>, hyperedge_b: HashSet<usize>) -> f64 {
    jaccard_similarity_rust(&hyperedge_a, &hyperedge_b)
}

/// Python wrapper for computing the Jaccard distance between two hyperedges.
/// 
/// The Jaccard distance is defined as 1 minus the Jaccard similarity.
/// 
/// # Arguments
/// * `hyperedge_a` - First hyperedge as a set of node indices
/// * `hyperedge_b` - Second hyperedge as a set of node indices
/// 
/// # Returns
/// * `f64` - A value between 0 (identical) and 1 (completely different)
#[pyfunction]
#[pyo3(name = "jaccard_distance")]
pub fn jaccard_distance(hyperedge_a: HashSet<usize>, hyperedge_b: HashSet<usize>) -> f64 {
    jaccard_distance_rust(&hyperedge_a, &hyperedge_b)
}

/// Python wrapper for computing the Clique Eigenvector Centrality (CEC) of nodes in a hypergraph.
/// 
/// # Arguments
/// * `hypergraph` - Reference to the Python hypergraph object
/// * `tol` - Tolerance for convergence
/// * `max_iter` - Maximum number of iterations
/// 
/// # Returns
/// * `PyResult<HashMap<usize, f64>>` - Map of node indices to their centrality values
/// * Raises `PyValueError` if computation fails
#[pyfunction]
#[pyo3(name = "cec_centrality")]
pub fn cec_centrality(hypergraph: &Hypergraph, tol: f64, max_iter: usize) -> PyResult<HashMap<usize, f64>> {
    let hypergraph_rust = &hypergraph.inner;
    cec_centrality_sequential(hypergraph_rust, tol, max_iter).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error computing CEC: {}", e))
    })
}

/// Python wrapper for computing the Z-Eigenvector Centrality (ZEC) of nodes in a hypergraph.
/// 
/// # Arguments
/// * `hypergraph` - Reference to the Python hypergraph object
/// * `tol` - Tolerance for convergence
/// * `max_iter` - Maximum number of iterations
/// 
/// # Returns
/// * `PyResult<HashMap<usize, f64>>` - Map of node indices to their centrality values
/// * Raises `PyValueError` if computation fails
#[pyfunction]
#[pyo3(name = "zec_centrality")]
pub fn zec_centrality(hypergraph: &Hypergraph, tol: f64, max_iter: usize) -> PyResult<HashMap<usize, f64>> {
    let hypergraph_rust = &hypergraph.inner;
    match zec_centrality_rust(hypergraph_rust, max_iter, tol) {
        Ok(result) => Ok(result.into_iter().collect()),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(e))
    }
}

/// Python wrapper for computing the H-Eigenvector Centrality (HEC) of nodes in a hypergraph.
/// 
/// # Arguments
/// * `hypergraph` - Reference to the Python hypergraph object
/// * `tol` - Tolerance for convergence
/// * `max_iter` - Maximum number of iterations
/// 
/// # Returns
/// * `PyResult<BTreeMap<usize, f64>>` - Ordered map of node indices to their centrality values
/// * Raises `PyValueError` if computation fails
#[pyfunction]
#[pyo3(name = "hec_centrality")]
pub fn hec_centrality(hypergraph: &Hypergraph, tol: f64, max_iter: usize) -> PyResult<BTreeMap<usize, f64>> {
    let hypergraph_rust = &hypergraph.inner;
    hec_centrality_rust(hypergraph_rust, tol, max_iter).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error computing HEC: {}", e))
    })
}

/// Python wrapper for computing the S-Betweenness centrality of edges in a hypergraph.
/// 
/// # Arguments
/// * `hypergraph` - Reference to the Python hypergraph object
/// * `s` - Threshold value for edge connectivity
/// 
/// # Returns
/// * `PyResult<HashMap<String, f64>>` - Map of edge identifiers to their betweenness values
#[pyfunction]
#[pyo3(name = "s_betweenness")]
pub fn s_betweenness(hypergraph: &Hypergraph, s: f64) -> PyResult<HashMap<String, f64>> {
    let hypergraph_rust = &hypergraph.inner;
    Ok(s_betweenness_rust(hypergraph_rust, s).into_iter()
        .map(|(k, v)| (k.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","), v as f64))
        .collect())
}

/// Python wrapper for computing the S-Closeness centrality of edges in a hypergraph.
/// 
/// # Arguments
/// * `hypergraph` - Reference to the Python hypergraph object
/// * `s` - Threshold value for edge connectivity
/// 
/// # Returns
/// * `PyResult<HashMap<String, f64>>` - Map of edge identifiers to their closeness values
#[pyfunction]
#[pyo3(name = "s_closeness")]
pub fn s_closeness(hypergraph: &Hypergraph, s: f64) -> PyResult<HashMap<String, f64>> {
    let hypergraph_rust = &hypergraph.inner;
    Ok(s_closeness_rust(hypergraph_rust, s).into_iter()
        .map(|(k, v)| (k.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","), v))
        .collect())
}
