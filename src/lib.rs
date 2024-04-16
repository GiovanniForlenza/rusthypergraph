use pyo3::prelude::*;
/*
use std::collections::{HashMap, HashSet};

pub struct Hypergraph{
    //attr: MetaHandler<T>,
    weighted: bool,
    edges_by_order: HashMap<usize, Vec<usize>>,
    adj: HashMap<usize, HashSet<usize>>,
    max_order: usize,
    edge_list: HashMap<Vec<usize>, f64>,
    neighbors: HashMap<usize, HashSet<usize>>,
}
impl Hypergraph{
    pub fn new(edge_list: Option<Vec<Vec<usize>>>, weighted: bool, weights: Option<Vec<f64>>, /*metadata:*/ ) -> Self{
        let mut hypergraph = Hypergraph {
            //attr: metadata.unwrap_or_else(MetaHandler::new),
            weighted,
            edges_by_order: HashMap::new(),
            adj: HashMap::new(),
            max_order: 0,
            edge_list: HashMap::new(),
            neighbors: HashMap::new(),
        };
        hypergraph.add_edges(edge_list, weights);
        hypergraph
    }

    pub fn add_edge(&mut self, edge: Vec<usize>, weight: Option<f64>, metadata: Option<HashMap<String, String>>) {
        // Algoritmo per aggiungere un edge

        // Aggiorniamo i vicini dei nodi coinvolti nell'edge
        for &node in &edge {
            for &neighbor in &edge {
                if node != neighbor {
                    self.neighbors.entry(node).or_insert(HashSet::new()).insert(neighbor);
                }
            }
        }
    }


    pub fn get_neighbors(&self, node: usize, order: Option<usize>, size: Option<usize>) -> HashSet<usize> {
        match (order, size) {
            (None, None) => {
                if let Some(neighbors) = self.neighbors.get(&node) {
                    neighbors.clone() // Cloniamo per evitare di modificare la struttura dati originale
                } else {
                    HashSet::new()
                }
            }
            _ => unimplemented!("Order and size specific case"),
        }
    }
}
*/

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}


#[pymodule]
fn rusthypergraph(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
