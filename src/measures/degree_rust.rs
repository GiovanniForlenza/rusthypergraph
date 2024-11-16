use std::collections::HashMap;
use crate::core::hypergraph_rust::HypergraphRust;

/// Calculates the degree of a node in a hypergraph.
/// 
/// # Arguments
/// * `hypergraph` - Reference to the hypergraph
/// * `node` - The node index to calculate degree for
/// * `order` - Optional order constraint for incident edges
/// * `size` - Optional size constraint for incident edges
/// 
/// # Returns
/// * `Ok(u64)` - The degree of the node
/// * `Err(String)` - Error if both order and size are specified
pub fn degree_rust(hypergraph: &HypergraphRust, node: usize, order: Option<usize>, size: Option<usize>) -> Result<u64, String> {
    
    let edges = match (order, size) {
        (Some(_), Some(_)) => return Err("Order and size cannot be both specified.".to_string()),
        (Some(order), None) => hypergraph.get_incident_edges(node, Some(order), None)?,
        (None, Some(size)) => hypergraph.get_incident_edges(node, None, Some(size))?,
        (None, None) => hypergraph.get_incident_edges(node, None, None)?,
    };
    
    Ok(edges.len() as u64)
}

/// Calculates the degree sequence for all nodes in a hypergraph.
/// 
/// # Arguments
/// * `hypergraph` - Reference to the hypergraph
/// * `order` - Optional order constraint for incident edges
/// * `size` - Optional size constraint for incident edges
/// 
/// # Returns
/// * `Ok(Some(HashMap<usize, u64>))` - Map of node indices to their degrees
/// * `Err(String)` - Error if both order and size are specified
pub fn degree_sequence_rust(hypergraph: &HypergraphRust, order: Option<usize>, size: Option<usize>) -> Result<Option<HashMap<usize, u64>>, String> {
    
    if order.is_some() && size.is_some() {
        return Err("Order and size cannot be both specified.".to_string());
    }

    let order = match size {
        Some(size) if size > 0 => Some(size - 1),
        _ => order,
    };
    
    let mut degree_seq = HashMap::new();
    for node in hypergraph.get_nodes_without_metadata() {
        let degree = match order {
            Some(o) => degree_rust(hypergraph, node, Some(o),None)?,
            None => degree_rust(hypergraph, node, None, None)?,
        };

        degree_seq.insert(node,degree);

    }
    Ok(Some(degree_seq))
}

/// Calculates the Pearson correlation coefficient between two vectors.
/// 
/// # Arguments
/// * `x` - First vector of values
/// * `y` - Second vector of values
/// 
/// # Returns
/// * `Ok(f64)` - The correlation coefficient between -1.0 and 1.0
/// * `Err(String)` - Error if vectors have different lengths or fewer than 2 elements
pub fn pearson_correlation(x: &[u64], y: &[u64]) -> Result<f64, String> {
    if x.len() != y.len() || x.len() < 2 {
        return Err("Vectors must have the same length and contain at least two elements.".to_string());
    }

    let mean_x = x.iter().map(|&xi| xi as f64).sum::<f64>() / x.len() as f64;
    let mean_y = y.iter().map(|&yi| yi as f64).sum::<f64>() / y.len() as f64;

    let num = x.iter().zip(y.iter())
        .map(|(&xi, &yi)| (xi as f64 - mean_x) * (yi as f64 - mean_y))
        .sum::<f64>();

    let den_x = (x.iter().map(|&xi| (xi as f64 - mean_x).powi(2)).sum::<f64>()).sqrt();
    let den_y = (y.iter().map(|&yi| (yi as f64 - mean_y).powi(2)).sum::<f64>()).sqrt();

    if den_x == 0.0 || den_y == 0.0 {
        return Ok(0.0);
    }

    Ok(num / (den_x * den_y))
}

/// Calculates the degree correlation matrix for a hypergraph.
/// 
/// Computes correlations between degree sequences for different edge sizes.
/// 
/// # Arguments
/// * `hypergraph` - Reference to the hypergraph
/// 
/// # Returns
/// * `Ok(Vec<Vec<f64>>)` - Matrix of correlation coefficients
/// * `Err(String)` - Error if degree sequences cannot be computed
pub fn degree_correlation_rust(hypergraph: &HypergraphRust) -> Result<Vec<Vec<f64>>, String> {
    let max_size = hypergraph.max_size();
    let mut seqs = Vec::new();

    for size in 2..=max_size {
        match degree_sequence_rust(hypergraph, None, Some(size))? {
            Some(seq) => seqs.push(seq),
            None => return Err(format!("Failed to get degree sequence for size {}", size)),
        }
    }

    let len = seqs.len();
    let mut matrix_degree_corr = Vec::with_capacity(len);

    for _ in 0..len {
        matrix_degree_corr.push(vec![0.0; len]);
    }

    for i in 0..len {
        for j in 0..len {
            if seqs[i].len() < 2 || seqs[j].len() < 2 {
                matrix_degree_corr[i][j] = f64::NAN;
            } else {
                let seq_i: Vec<u64> = seqs[i].iter().map(|(_, &d)| d).collect();
                let seq_j: Vec<u64> = seqs[j].iter().map(|(_, &d)| d).collect();

                match pearson_correlation(&seq_i, &seq_j) {
                    Ok(corr) => matrix_degree_corr[i][j] = (corr * 100000000.0).round() / 100000000.0,
                    Err(_) => matrix_degree_corr[i][j] = f64::NAN,
                }
            }
        }
    }

    Ok(matrix_degree_corr)
}

/// Calculates the degree distribution of a hypergraph.
/// 
/// # Arguments
/// * `hypergraph` - Reference to the hypergraph
/// * `order` - Optional order constraint for incident edges
/// * `size` - Optional size constraint for incident edges
/// 
/// # Returns
/// * `Ok(Some(HashMap<u64, i32>))` - Map of degrees to their frequencies
/// * `Err(String)` - Error if both order and size are specified
pub fn degree_distribution_rust(
    hypergraph: &HypergraphRust,
    order: Option<usize>,
    size: Option<usize>,
) -> Result<Option<HashMap<u64, i32>>, String> {
    if order.is_some() && size.is_some() {
        return Err("Order and size cannot be both specified.".to_string());
    }

    let effective_order = if let Some(s) = size {
        Some(s - 1)
    } else {
        order
    };

    let degree_seq = degree_sequence_rust(hypergraph, effective_order, None)?;

    let mut degree_dist = HashMap::new();

    if let Some(degree_seq) = degree_seq {
        for (_, degree) in degree_seq {
            *degree_dist.entry(degree).or_insert(0) += 1;
        }
    }

    Ok(Some(degree_dist))
}
