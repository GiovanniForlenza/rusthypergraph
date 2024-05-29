use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};
use std::collections::{HashMap, HashSet};

// Funzione per ottenere l'istanza di MetaHandler da Python
fn get_meta_handler(py: Python) -> PyResult<PyObject> {
    let module = py.import("hypergraphx.core.meta_handler")?;
    let metahandler = module.getattr("MetaHandler")?.call0()?;
    Ok(metahandler.into_py(py))
}

#[pyclass]
pub struct Hypergraph {
    attr: PyObject,
    weighted: bool,
    edges_by_order: HashMap<usize, HashSet<Vec<usize>>>,
    adj: HashMap<usize, HashSet<usize>>,
    max_order: usize,
    edge_list: HashMap<Vec<usize>, f64>,
}

#[pymethods]
impl Hypergraph {
    #[new]
    #[pyo3(signature = (edge_list=None, weighted=false, weights=None, metadata=None))]
    fn new(
        py: Python,
        edge_list: Option<Vec<Vec<usize>>>,
        weighted: bool,
        weights: Option<Vec<f64>>,
        metadata: Option<HashMap<String, String>>,
    ) -> PyResult<Self> {
        // Verifica che i pesi siano forniti e corrispondano al numero di archi
        if weighted {
            if weights.is_none() {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "If the hypergraph is weighted, weights must be provided.",
                ));
            }
            let weights = weights.as_ref().unwrap();
            if edge_list.as_ref().map_or(false, |edges| edges.len() != weights.len()) {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "If the hypergraph is weighted, weights must match the number of edges.",
                ));
            }
        }

        let attr = get_meta_handler(py)?;

        let mut hypergraph = Hypergraph {
            attr,
            weighted,
            edges_by_order: HashMap::new(),
            adj: HashMap::new(),
            max_order: 0,
            edge_list: HashMap::new(),
        };

        if let Some(edges) = edge_list {
            hypergraph.add_edges(py, edges, weights, metadata)?;
        }

        Ok(hypergraph)
    }

    pub fn add_edge(
        &mut self,
        py: Python,
        edge: Vec<usize>,
        weight: Option<f64>,
        metadata: Option<HashMap<String, String>>,
    ) -> PyResult<()> {
        if self.weighted && weight.is_none() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "If the hypergraph is weighted, a weight must be provided.",
            ));
        }
        if !self.weighted && weight.is_some() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "If the hypergraph is not weighted, no weight must be provided.",
            ));
        }

        let mut sorted_edge = edge.clone();
        sorted_edge.sort_unstable();

        let py_sorted_edge = PyTuple::new(py, &sorted_edge);

        let idx: usize = self
            .attr
            .call_method(py, "add_obj", (py_sorted_edge,), None)?
            .extract(py)?;

        let order = sorted_edge.len() - 1;

        if let Some(meta) = metadata {
            let metadata_dict = PyDict::new(py);
            for (k, v) in meta {
                metadata_dict.set_item(k, v)?;
            }
            self.attr.call_method1(py, "set_attr", (py_sorted_edge, metadata_dict))?;
        }

        if order > self.max_order {
            self.max_order = order;
        }

        self.edges_by_order
            .entry(order)
            .or_insert_with(HashSet::new)
            .insert(sorted_edge.clone());

        let edge_key = sorted_edge.clone();
        if self.weighted {
            self.edge_list.insert(edge_key, weight.unwrap_or(1.0));
        } else {
            *self.edge_list.entry(edge_key).or_insert(1.0) += 1.0;
        }

        for node in &sorted_edge {
            self.add_node(py, *node);
            self.adj.entry(*node).or_insert_with(HashSet::new).insert(idx);
        }

        Ok(())
    }

    fn add_edges(
        &mut self,
        py: Python,
        edges: Vec<Vec<usize>>,
        weights: Option<Vec<f64>>,
        metadata: Option<HashMap<String, String>>,
    ) -> PyResult<()> {
        for (i, edge) in edges.into_iter().enumerate() {
            let weight = weights.as_ref().map(|w| w[i]);
            self.add_edge(py, edge, weight, metadata.clone())?;
        }
        Ok(())
    }

    fn add_node(&mut self, py: Python, node: usize) {
        self.adj.entry(node).or_insert_with(HashSet::new);
        let _ = self.attr.call_method(py, "add_obj", (node, "node"), None);
    }

    fn add_nodes(&mut self, py:Python ,nodes: Vec<usize>) {
        for node in nodes {
            self.add_node(py, node);
        }
    }

    pub fn get_nodes(&self, py: Python, metadata: bool) -> PyResult<PyObject> {
        if !metadata {
            let nodes: Vec<usize> = self.adj.keys().cloned().collect();
            Ok(PyList::new(py, nodes).into())
        } else {
            let nodes_with_metadata: Vec<(usize, PyObject)> = self.adj.keys().map(|&node| {
                let meta = self.get_meta(py, node).unwrap_or_else(|| PyDict::new(py).into());
                (node, meta)
            }).collect();
            Ok(PyList::new(py, nodes_with_metadata).into())
        }
    }

    fn get_meta(&self, py: Python, node: usize) -> Option<PyObject> {
        let py_node = PyTuple::new(py, &[node]);
        self.attr.call_method(py, "get_meta", (py_node,), None).ok()
    }

//     // pub fn check_edge() -> PyResult<PyObject>{}
//     // pub fn check_node() -> PyResult<PyObject>{}
//     // pub fn copy() -> PyResult<PyObject>{}
//     // pub fn distribution_sizes() -> PyResult<PyObject>{}
//     // pub fn get_attr_meta() -> PyResult<PyObject>{}
//     // pub fn get_edges() -> PyResult<PyObject>{}
//     // pub fn get_incident_edges() -> PyResult<PyObject>{}
//     // pub fn get_mapping() -> PyResult<PyObject>{}
//     // pub fn get_meta() -> PyResult<PyObject>{}
//     // pub fn get_neighbors() -> PyResult<PyObject>{}
//     // pub fn get_nodes() -> PyResult<PyObject>{}
//     // pub fn get_orders() -> PyResult<PyObject>{}
//     // pub fn get_sizes() -> PyResult<PyObject>{}
//     // pub fn get_weight() -> PyResult<PyObject>{}
//     // pub fn get_weights() -> PyResult<PyObject>{}
//     // pub fn is_uniform() -> PyResult<PyObject>{}
//     // pub fn is_weighted() -> PyResult<PyObject>{}
//     // pub fn max_order() -> PyResult<PyObject>{}
//     // pub fn max_size() -> PyResult<PyObject>{}
//     // pub fn num_edges() -> PyResult<PyObject>{}
//     // pub fn num_nodes() -> PyResult<PyObject>{}
//     // pub fn remove_edge() -> PyResult<PyObject>{}
//     // pub fn remove_edges() -> PyResult<PyObject>{}
//     // pub fn remove_node() -> PyResult<PyObject>{}
//     // pub fn remove_nodes() -> PyResult<PyObject>{}
//     // pub fn set_meta() -> PyResult<PyObject>{}
//     // pub fn set_weight() -> PyResult<PyObject>{}
//     // pub fn subhypergraph() -> PyResult<PyObject>{}
//     // pub fn subhypergraph_by_orders() -> PyResult<PyObject>{}
//     // pub fn subhypergraph_largest_component() -> PyResult<PyObject>{}
}

