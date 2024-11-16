use std::collections::HashMap;
use rustworkx_core::centrality::{betweenness_centrality, closeness_centrality};
use rustworkx_core::petgraph::graph::Graph;
use rustworkx_core::petgraph::Undirected;
use crate::core::hypergraph_rust::HypergraphRust;
use std::collections::HashSet;

/// Calculates the S-Betweenness centrality for edges in a hypergraph.
/// 
/// # Arguments
/// * `hypergraph` - The input hypergraph
/// * `s` - Threshold value for edge connectivity
/// 
/// # Returns
/// A HashMap mapping edge indices to their betweenness centrality values
pub fn s_betweenness_rust(hypergraph: &HypergraphRust, s: f64) -> HashMap<Vec<usize>, f64> {
    let (graph, id_to_edge) = line_graph(hypergraph, "intersection", s, false);
    let betweenness = betweenness_centrality(&graph, false, true, 50);
    
    betweenness.into_iter()
        .enumerate()
        .filter_map(|(k, v)| v.map(|value| (id_to_edge[&k].clone(), value)))
        .collect()
}

/// Calculates the S-Closeness centrality for edges in a hypergraph.
/// 
/// # Arguments
/// * `hypergraph` - The input hypergraph
/// * `s` - Threshold value for edge connectivity
/// 
/// # Returns
/// A HashMap mapping edge indices to their closeness centrality values
pub fn s_closeness_rust(hypergraph: &HypergraphRust, s: f64) -> HashMap<Vec<usize>, f64> {
    let (graph, id_to_edge) = line_graph(hypergraph, "intersection", s, false);
    let closeness = closeness_centrality(&graph, true);
    
    closeness.into_iter()
        .enumerate()
        .filter_map(|(k, v)| v.map(|value| (id_to_edge[&k].clone(), value)))
        .collect()
}

/// Constructs a line graph from a hypergraph based on edge intersections.
/// 
/// Creates a graph where nodes represent hyperedges and edges represent 
/// relationships between hyperedges based on their intersection size.
/// 
/// # Arguments
/// * `hypergraph` - The input hypergraph
/// * `distance_type` - Type of distance measure to use ("intersection" or "jaccard")
/// * `s` - Threshold value for edge connectivity
/// * `weighted` - Whether to use weighted edges in the line graph
/// 
/// # Returns
/// A tuple containing:
/// * The line graph as a Graph<(), f64, Undirected>
/// * A HashMap mapping node indices to their corresponding hyperedge indices
pub fn line_graph(
    hypergraph: &HypergraphRust, 
    distance_type: &str,
    s: f64, 
    weighted: bool
) -> (Graph<(), f64, Undirected>, HashMap<usize, Vec<usize>>) {
    let edge_list: Vec<_> = hypergraph.edge_list.keys().collect();
    let num_edges = edge_list.len();
    let mut id_to_edge: HashMap<usize, Vec<usize>> = HashMap::new();
    
    let mut graph = Graph::<(), f64, Undirected>::default();
    let mut node_indices = Vec::with_capacity(num_edges);

    for (i, edge) in edge_list.iter().enumerate() {
        node_indices.push(graph.add_node(()));
        id_to_edge.insert(i, (*edge).clone());
    }

    let calculate_distance = |edge1: &Vec<usize>, edge2: &Vec<usize>| -> f64 {
        let set1: HashSet<_> = edge1.iter().collect();
        let set2: HashSet<_> = edge2.iter().collect();
        
        match distance_type {
            "intersection" => set1.intersection(&set2).count() as f64,
            "jaccard" => {
                let intersection = set1.intersection(&set2).count() as f64;
                let union = set1.union(&set2).count() as f64;
                intersection / union
            },
            _ => set1.intersection(&set2).count() as f64, // default to intersection
        }
    };

    for i in 0..num_edges {
        for j in (i + 1)..num_edges {
            let edge1 = &edge_list[i];
            let edge2 = &edge_list[j];

            let distance = calculate_distance(edge1, edge2);

            if distance >= s {
                let weight = if weighted { distance } else { 1.0 };
                graph.add_edge(node_indices[i], node_indices[j], weight);
            }
        }
    }

    (graph, id_to_edge)
}