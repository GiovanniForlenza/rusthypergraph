use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rand::prelude::*;
use std::collections::{HashMap, HashSet};

#[pyfunction]
fn hoad_model(_py: Python, n: usize, activities_per_order: HashMap<usize, Vec<f64>>, time: usize) -> PyResult<PyObject> {
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


#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}


#[pymodule]
fn rusthypergraph(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hoad_model,m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
