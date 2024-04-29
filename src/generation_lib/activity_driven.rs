use pyo3::prelude::*;
use rand::prelude::*;
use std::collections::{HashMap, HashSet};

// Questa funzione genera un modello di ipergrafo usando il modello HOAD (Activity-Driven Hypergraph) 
// con un numero specificato di nodi (N) e attività per ogni ordine (activities_per_order) 
// per un determinato periodo di tempo (time).

#[pyfunction]
pub fn hoad_model(_py: Python, n: usize, activities_per_order: HashMap<usize, Vec<f64>>, time: usize) -> PyResult<PyObject> {
    let mut rng = rand::thread_rng();
    let mut hyperedges = Vec::new();

    for (order, act_vect) in activities_per_order.iter() {
        for t in 0..time {
            for node_i in 0..n {
                if act_vect[node_i] > rng.gen::<f64>() {
                    let mut neigh_list: Vec<usize> = (0..n).collect();
                    neigh_list.shuffle(&mut rng);
                    neigh_list.truncate(*order);
                    neigh_list.push(node_i);
                    if neigh_list.len() == neigh_list.iter().cloned().collect::<HashSet<_>>().len() {
                        hyperedges.push((t, neigh_list.clone()));
                    }
                }
            }
        }
    }

    Python::with_gil(|py| {
        let module = py.import("hypergraphx.core.temporal_hypergraph").unwrap();
        let temporal_hypergraph_class = module.getattr("TemporalHypergraph").unwrap();
        let temporal_hypergraph_instance = temporal_hypergraph_class.call1((hyperedges,))?;

        Ok(temporal_hypergraph_instance.into_py(py))
    })
}