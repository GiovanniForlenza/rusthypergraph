use pyo3::prelude::*;
use pyo3::types::PyList;
use std::collections::HashMap;
use rand::seq::SliceRandom;

// Questa funzione genera un ipergrafo casuale con un numero specificato di nodi e una distribuzione 
// specificata del numero di archi per ogni dimensione.
#[pyfunction]
pub fn generate_hypergraph(py: Python<'_>, num_nodes: usize, num_edges_by_size: HashMap<usize, usize>) -> PyResult<PyObject> {

    let hypergraph_module = py.import("hypergraphx.core.hypergraph")?;
    let hypergraph_class = hypergraph_module.getattr("Hypergraph")?;

    let hg = hypergraph_class.call0()?.to_object(py);

    let nodes: Vec<usize> = (0..num_nodes).collect();
    hg.call_method1(py, "add_nodes", (nodes,))?;

    for (size, count) in num_edges_by_size.iter() {
        let edges: Vec<Vec<usize>> = (0..*count)
            .map(|_| {
                let mut rng = rand::thread_rng();
                let mut nodes_sample: Vec<usize> = (0..num_nodes).collect();
                nodes_sample.shuffle(&mut rng);
                nodes_sample.truncate(*size);
                nodes_sample.sort();
                nodes_sample
            })
            .collect();

        let edges_py = PyList::new(py, edges);
        hg.call_method1(py, "add_edges", (edges_py,))?;
    }

    Ok(hg)
}

//  Questa funzione aggiunge un nuovo arco casuale all'ipergrafo specificato.
#[pyfunction]
pub fn add_random_edge(hg: &PyAny, order: Option<usize>, size: Option<usize>, inplace: Option<bool>) -> PyResult<Option<PyObject>> {
    let order = order;
    let size = size;
    let inplace = inplace.unwrap_or(true);

    if order.is_some() && size.is_some() {
        return Err(pyo3::exceptions::PyValueError::new_err("Order and size cannot be both specified."));
    }
    if order.is_none() && size.is_none() {
        return Err(pyo3::exceptions::PyValueError::new_err("Order or size must be specified."));
    }

    let size = size.unwrap_or_else(|| order.unwrap() + 1);

    let nodes: Vec<usize> = hg.call_method0("get_nodes")?.extract()?;

    let mut rng = rand::thread_rng();
    let mut edge: Vec<usize> = nodes.choose_multiple(&mut rng, size).cloned().collect();
    edge.sort(); 
    
    if inplace {
        hg.call_method1("add_edge", (edge,))?;
        Ok(None)
    } else {
        let h = hg.call_method0("copy")?.call_method1("add_edge", (edge,))?;
        Ok(Some(h.into()))
    }
}

//  Questa funzione aggiunge un numero specificato di nuovi archi casuali all'ipergrafo specificato.
#[pyfunction]
pub fn add_random_edges(hg: &PyAny, num_edges: usize, order: Option<usize>, size: Option<usize>, inplace: Option<bool>) -> PyResult<Option<PyObject>> {
    let order = order;
    let size = size;
    let inplace = inplace.unwrap_or(true);

    if order.is_some() && size.is_some() {
        return Err(pyo3::exceptions::PyValueError::new_err("Order and size cannot be both specified."));
    }
    if order.is_none() && size.is_none() {
        return Err(pyo3::exceptions::PyValueError::new_err("Order or size must be specified."));
    }

    let size = size.unwrap_or_else(|| order.unwrap() + 1);

    let nodes: Vec<usize> = hg.call_method0("get_nodes")?.extract()?;

    let mut edges = std::collections::HashSet::new();
    let mut rng = rand::thread_rng();
    while edges.len() < num_edges {
        let edge: Vec<usize> = nodes.choose_multiple(&mut rng, size).cloned().collect();
        let mut sorted_edge = edge.clone(); 
        sorted_edge.sort(); 
        edges.insert(sorted_edge);
    }

    let edges_py: Vec<PyObject> = edges.iter().map(|edge| edge.clone().into_py(hg.py())).collect();

    if inplace {
        hg.call_method1("add_edges", (edges_py,))?;
        Ok(None)
    } else {
        let h = hg.call_method0("copy")?.call_method1("add_edges", (edges_py,))?;
        Ok(Some(h.into()))
    }
}
