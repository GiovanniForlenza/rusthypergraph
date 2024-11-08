extern crate nalgebra as na;
use na::{DMatrix, DVector};
use std::collections::HashMap;
use crate::core::hypergraph_rust::HypergraphRust;

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

pub fn cec_centrality_rust(hypergraph: &HypergraphRust, tol: f64, max_iter: usize) -> Result<HashMap<usize, f64>, String>{

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

    let centrality = (0..num_nodes)
        .map(|node| (node, dominant_eig[node]))
        .collect();

    Ok(centrality)
}

pub fn zec_centrality_rust(hypergraph: &HypergraphRust, tol: f64, max_iter: usize) -> Result<HashMap<usize, f64>, String> {
    if !hypergraph.is_uniform() {
        return Err("L'ipergrafo non è uniforme.".to_string());
    }
    if !hypergraph.is_connected_rust() {
        return Err("L'ipergrafo non è connesso.".to_string());
    }

    let g = |v: &DVector<f64>, edge: &Vec<usize>| {
        edge.iter().map(|&node| v[node]).product::<f64>()
    };

    let mut x = DVector::from_iterator(hypergraph.num_nodes(), (0..hypergraph.num_nodes()).map(|_| rand::random::<f64>()));
    x /= x.sum();

    for _ in 0..max_iter {
        let mut new_x = DVector::zeros(hypergraph.num_nodes());

        for edge in hypergraph.get_edges(false, None, None, false)? {
            let edge_prod = g(&x, edge);
            for &node in edge {
                new_x[node] += edge_prod;
            }
        }

        new_x /= new_x.sum();
        if (x.clone() - &new_x).norm() <= tol {
            break;
        }
        x = new_x;
    }

    let centrality: HashMap<usize, f64> = (0..hypergraph.num_nodes())
        .map(|node| (node, x[node]))
        .collect();

    Ok(centrality)
}
