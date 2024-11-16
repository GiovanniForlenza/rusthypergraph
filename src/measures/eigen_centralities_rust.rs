extern crate nalgebra as na;
use na::{DMatrix, DVector};
use std::collections::HashMap;
use crate::core::hypergraph_rust::HypergraphRust;
use rand::{distributions::{Distribution, Uniform}, Rng};
use std::collections::BTreeMap;

/// Performs power iteration method to find the dominant eigenvector of a matrix.
/// 
/// # Arguments
/// * `w_matrix` - The square matrix to find the dominant eigenvector for
/// * `tol` - Tolerance for convergence
/// * `max_iter` - Maximum number of iterations
/// 
/// # Returns
/// * `Ok(DVector<f64>)` - The dominant eigenvector if convergence is reached
/// * `Err(String)` - Error message if maximum iterations are reached without convergence
fn power_iteration(w_matrix: &DMatrix<f64>, tol: f64, max_iter: usize) -> Result<DVector<f64>, String> {
    let mut x = DVector::from_element(w_matrix.nrows(), 1.0);
    x = x.clone() / x.norm();
    let mut res = f64::INFINITY;
    let mut k = 0;

    while res > tol && k < max_iter {
        let y = w_matrix * &x;
        let y_norm = y.norm();
        res = (&x - &y / y_norm).norm();
        x = y / y_norm;
        k += 1;
    }

    if k >= max_iter {
        return Err("Metodo di potenza non convergente.".to_string());
    }

    Ok(x)
}

/// Calculates the Clique Eigenvector Centrality (CEC) for nodes in a uniform hypergraph.
/// 
/// # Arguments
/// * `hypergraph` - The input hypergraph
/// * `tol` - Tolerance for convergence
/// * `max_iter` - Maximum number of iterations
/// 
/// # Returns
/// * `Ok(HashMap<usize, f64>)` - Map of node indices to their centrality values
/// * `Err(String)` - Error if the hypergraph is not uniform or not connected
pub fn cec_centrality_sequential(
    hypergraph: &HypergraphRust, 
    tol: f64, 
    max_iter: usize
) -> Result<HashMap<usize, f64>, String> {
    if !hypergraph.is_uniform() {
        return Err("L'ipergrafo non è uniforme.".to_string());
    }
    if !hypergraph.is_connected_rust() {
        return Err("L'ipergrafo non è connesso.".to_string());
    }

    let num_nodes = hypergraph.num_nodes();
    let mut w_matrix = DMatrix::from_element(num_nodes, num_nodes, 0.0);
    
    for edge in hypergraph.get_edges(false, None, None, false)? {
        for i in 0..edge.len() {
            for j in (i + 1)..edge.len() {
                w_matrix[(edge[i], edge[j])] += 1.0;
                w_matrix[(edge[j], edge[i])] += 1.0;
            }
        }
    }

    let dominant_eig = power_iteration(&w_matrix, tol, max_iter)?;
    Ok((0..num_nodes).map(|node| (node, dominant_eig[node])).collect())
}

/// Calculates the Z-eigenvector Centrality (ZEC) for nodes in a uniform hypergraph.
/// 
/// This implementation uses the Z-eigenvalue method to compute centrality values
/// for nodes in the hypergraph.
/// 
/// # Arguments
/// * `hypergraph` - The input hypergraph
/// * `max_iter` - Maximum number of iterations
/// * `tol` - Tolerance for convergence
/// 
/// # Returns
/// * `Ok(BTreeMap<usize, f64>)` - Ordered map of node indices to their centrality values
/// * `Err(String)` - Error if the hypergraph is not uniform or not connected
pub fn zec_centrality_rust(
    hypergraph: &HypergraphRust,
    max_iter: usize,
    tol: f64,
) -> Result<BTreeMap<usize, f64>, String> {
    
    if !hypergraph.is_uniform() {
        return Err("The hypergraph is not uniform.".to_string());
    }
    if !hypergraph.is_connected_rust() {
        return Err("The hypergraph is not connected.".to_string());
    }

    let g = |v: &Vec<f64>, edge: &Vec<usize>| -> f64 {
        edge.iter().map(|&node| v[node]).product()
    };

    let num_nodes = hypergraph.num_nodes();
    let mut rng = rand::thread_rng();
    let mut x: Vec<f64> = (0..num_nodes).map(|_| rng.gen()).collect();

    let norm = x.iter().sum::<f64>();
    for xi in x.iter_mut() {
        *xi /= norm;
    }

    for _ in 0..max_iter {
        let mut new_x = vec![0.0; num_nodes];
        for edge in hypergraph.get_edges(false, None, None, false).map_err(|e| e.to_string())? {
            let edge_value = g(&x, edge);
            for node in edge.iter() {
                new_x[*node] += edge_value;
            }
        }

        let sign = new_x[0].signum();
        let norm = new_x.iter().map(|&xi| xi.abs()).sum::<f64>();
        for xi in new_x.iter_mut() {
            *xi = sign * (*xi / norm);
        }

        let diff = x.iter()
            .zip(new_x.iter())
            .map(|(xi, new_xi)| (xi - new_xi).abs())
            .sum::<f64>();

        if diff <= tol {
            break;
        }
        x = new_x;
    }

    let mut zec = BTreeMap::new();
    for (node, &value) in x.iter().enumerate() {
        zec.insert(node, value);
    }

    Ok(zec)
}

/// Calculates the H-eigenvector Centrality (HEC) for nodes in a uniform hypergraph.
/// 
/// This implementation computes centrality values using the H-eigenvalue method,
/// which is particularly suited for uniform hypergraphs.
/// 
/// # Arguments
/// * `hypergraph` - The input hypergraph
/// * `tol` - Tolerance for convergence
/// * `max_iter` - Maximum number of iterations
/// 
/// # Returns
/// * `Ok(BTreeMap<usize, f64>)` - Ordered map of node indices to their centrality values,
///                                sorted in descending order by centrality
/// * `Err(String)` - Error if the hypergraph is not uniform, not connected, or if
///                   maximum iterations are reached without convergence
pub fn hec_centrality_rust(
    hypergraph: &HypergraphRust,
    tol: f64,
    max_iter: usize
) -> Result<BTreeMap<usize, f64>, String> {
    if !hypergraph.is_uniform() {
        return Err("The hypergraph is not uniform.".to_string());
    }
    if !hypergraph.is_connected_rust() {
        return Err("The hypergraph is not connected.".to_string());
    }

    let num_nodes = hypergraph.num_nodes();
    let edges = hypergraph.get_edges(false, None, None, false)?;
    let m = edges[0].len(); // m is the size of each edge (uniformity)

    let mut x = {
        let mut rng = rand::thread_rng();
        let dist = Uniform::new(0.0, 1.0);
        let mut initial = DVector::from_iterator(num_nodes, 
            (0..num_nodes).map(|_| dist.sample(&mut rng)));

        let norm = initial.norm();
        initial.scale_mut(1.0 / norm);
        initial
    };

    let mut new_x = DVector::zeros(num_nodes);

    for _ in 0..max_iter {
        new_x.fill(0.0);
        for edge in edges.iter() {
            for &i in edge.iter() {
                // Calculate product of all nodes in edge except i
                let prod: f64 = edge.iter()
                    .filter(|&&j| j != i)
                    .map(|&j| x[j])
                    .product();
                new_x[i] += prod;
            }
        }

        for val in new_x.iter_mut() {
            *val = val.powf(1.0 / (m as f64 - 1.0));
        }

        let norm = new_x.norm();
        new_x.scale_mut(1.0 / norm);

        let diff = (&x - &new_x).norm();
        if diff <= tol {
            let mut centrality_vec: Vec<(usize, f64)> = new_x.iter()
                .enumerate()
                .map(|(i, &v)| (i, v))
                .collect();
            
            centrality_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            
            return Ok(centrality_vec.into_iter().collect());
        }

        std::mem::swap(&mut x, &mut new_x);
    }

    Err("Maximum iterations reached without convergence".to_string())
}
