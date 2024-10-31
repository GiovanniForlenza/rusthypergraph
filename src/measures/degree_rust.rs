use crate::core::hypergraph_rust::HypergraphRust;

pub fn degree_rust(hypergraph: &HypergraphRust, node: usize, order: Option<usize>, size: Option<usize>) -> Result<u64, String> {
    
    let edges = match (order, size) {
        (Some(_), Some(_)) => return Err("Order and size cannot be both specified.".to_string()),
        (Some(order), None) => hypergraph.get_incident_edges(node, Some(order), None)?,
        (None, Some(size)) => hypergraph.get_incident_edges(node, None, Some(size))?,
        (None, None) => hypergraph.get_incident_edges(node, None, None)?,
    };
    
    Ok(edges.len() as u64)
}
