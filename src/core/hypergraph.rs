use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};
use std::collections::{HashMap, HashSet};
use super::meta_handler::MetaHandler;

#[pyclass]
pub struct Hypergraph {
    attr: MetaHandler<String>,
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
    pub fn new(
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

        let mut hypergraph = Hypergraph {
            attr: MetaHandler::new(),
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
            return Err(pyo3::exceptions::PyValueError::new_err(
                "If the hypergraph is weighted, a weight must be provided.",
            ));
        }
        
        if !self.weighted && weight.is_some() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "If the hypergraph is not weighted, no weight must be provided.",
            ));
        }

        let mut sorted_edge = edge.clone();
        sorted_edge.sort_unstable();
        let edge_str = format!("{:?}", sorted_edge);

        let idx = self.attr.add_object(edge_str, metadata);

        let order = sorted_edge.len() - 1;
        if order > self.max_order {
            self.max_order = order;
        }

        self.edges_by_order
            .entry(order)
            .or_insert_with(HashSet::new)
            .insert(sorted_edge.clone());

        if self.weighted {
            self.edge_list.insert(sorted_edge.clone(), weight.unwrap_or(1.0));
        } else {
            *self.edge_list.entry(sorted_edge.clone()).or_insert(1.0) += 1.0;
        }

        for node in &sorted_edge {
            self.add_node(py, *node);
            self.adj.entry(*node).or_insert_with(HashSet::new).insert(idx);
        }

        Ok(())
    }

    pub fn add_edges(
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

    pub fn add_node(&mut self, py: Python, node: usize) {
        self.adj.entry(node).or_insert_with(HashSet::new);
        let mut metadata = HashMap::new();
        metadata.insert("Type".to_string(), "node".to_string());
        self.attr.add_object(node.to_string(), Some(metadata));
    }

    pub fn add_nodes(&mut self, py:Python, nodes: Vec<usize>) {
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

    pub fn get_meta(&self, py: Python, obj_id: usize) -> Option<PyObject> {
        if let Some(attributes) = self.attr.get_attributes(obj_id) {
            let py_dict = PyDict::new(py);
            for (key, value) in attributes {
                py_dict.set_item(key, value).unwrap();
            }
            Some(py_dict.into())
        } else {
            None
        }
    }

    // ids (bool): Se True, restituisce gli ID degli edge anziché gli oggetti edge stessi.
    // order (Option<usize>): Specifica l'ordine degli edge da restituire. L'ordine di un edge è determinato dal numero di nodi meno uno.
    // size (Option<usize>): Specifica la dimensione degli edge da restituire. La dimensione di un edge è il numero di nodi che contiene.
    // up_to (bool): Se True, restituisce tutti gli edge fino all'ordine (o dimensione) specificato, anziché solo quelli dell'ordine specificato.
    // subhypergraph (bool): Se True, restituisce un nuovo subhypergraph contenente gli edge selezionati, anziché una lista di edge.
    // keep_isolated_nodes (bool): Se True e subhypergraph è True, include anche i nodi isolati (nodi senza edge) nel subhypergraph risultante.

    #[pyo3(signature = (ids = false, order = None, size = None, up_to = false, subhypergraph = false, keep_isolated_nodes = false))]
    pub fn get_edges(
        &self,
        py: Python,
        ids: bool,
        order: Option<usize>,
        size: Option<usize>,
        up_to: bool,
        subhypergraph: bool,
        keep_isolated_nodes: bool,
    ) -> PyResult<PyObject> {
        if order.is_some() && size.is_some() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Order and size cannot be both specified.",
            ));
        }
        if ids && subhypergraph {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Cannot return subhypergraphs with ids.",
            ));
        }
        if !subhypergraph && keep_isolated_nodes {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Cannot keep nodes if not returning subhypergraphs.",
            ));
        }

        let mut edges: Vec<PyObject> = Vec::new();

        if order.is_none() && size.is_none() {
            for edge in self.edge_list.keys() {
                let py_edge = PyTuple::new(py, edge);
                if ids {
                    if let Some(edge_id) = self.attr.get_id_by_object(&py_edge.to_string()) {
                        edges.push(edge_id.into_py(py));
                    }
                } else {
                    edges.push(py_edge.into());
                }
            }
        } else {
            let max_order = if up_to {
                self.max_order
            } else {
                size.map_or(order, |s| Some(s - 1)).unwrap_or(self.max_order)
            };

            for i in 0..=max_order {
                if let Some(order_edges) = self.edges_by_order.get(&i) {
                    for edge in order_edges {
                        if ids {
                            let py_edge = PyTuple::new(py, edge);
                            if let Some(edge_id) = self.attr.get_id_by_object(&py_edge.to_string()) {
                                edges.push(edge_id.into_py(py));
                            }
                        } else {
                            edges.push(PyList::new(py, edge).into());
                        }
                    }
                }
            }
        }

        if subhypergraph {
            let mut subgraph = Hypergraph {
                attr: self.attr.clone(),
                weighted: self.weighted,
                edges_by_order: HashMap::new(),
                adj: HashMap::new(),
                max_order: 0,
                edge_list: HashMap::new(),
            };

            for edge in &edges {
                if let Ok(edge_list) = edge.extract::<Vec<usize>>(py) {
                    subgraph.add_edge(py, edge_list, None, None)?;
                }
            }

            if keep_isolated_nodes {
                for node in self.adj.keys() {
                    subgraph.add_node(py, *node);
                }
            }

            Ok(Py::new(py, subgraph)?.into_py(py))
        } else {
            Ok(PyList::new(py, edges).into())
        }
    }


    // pub fn check_edge() -> PyResult<PyObject>{}
    // pub fn check_node() -> PyResult<PyObject>{}
    // pub fn copy() -> PyResult<PyObject>{}
    // pub fn distribution_sizes() -> PyResult<PyObject>{}
    // pub fn get_attr_meta() -> PyResult<PyObject>{}
    // pub fn get_incident_edges() -> PyResult<PyObject>{}
    // pub fn get_mapping() -> PyResult<PyObject>{}
    // pub fn get_neighbors() -> PyResult<PyObject>{}
    // pub fn get_orders() -> PyResult<PyObject>{}
    // pub fn get_sizes() -> PyResult<PyObject>{}
    // pub fn get_weight() -> PyResult<PyObject>{}
    // pub fn get_weights() -> PyResult<PyObject>{}
    // pub fn is_uniform() -> PyResult<PyObject>{}
    // pub fn is_weighted() -> PyResult<PyObject>{}
    // pub fn max_order() -> PyResult<PyObject>{}
    // pub fn max_size() -> PyResult<PyObject>{}
    // pub fn num_edges() -> PyResult<PyObject>{}
    // pub fn num_nodes() -> PyResult<PyObject>{}
    // pub fn remove_edge() -> PyResult<PyObject>{}
    // pub fn remove_edges() -> PyResult<PyObject>{}
    // pub fn remove_node() -> PyResult<PyObject>{}
    // pub fn remove_nodes() -> PyResult<PyObject>{}
    // pub fn set_meta() -> PyResult<PyObject>{}
    // pub fn set_weight() -> PyResult<PyObject>{}
    // pub fn subhypergraph() -> PyResult<PyObject>{}
    // pub fn subhypergraph_by_orders() -> PyResult<PyObject>{}
    // pub fn subhypergraph_largest_component() -> PyResult<PyObject>{}
}

