use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};
use std::collections::{HashMap, HashSet};
use super::meta_handler::MetaHandler;

#[pyclass]
#[derive(Clone)]
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

    pub fn add_node(&mut self, _py: Python, node: usize) {
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

    pub fn is_weighted(&self) -> bool{
        return  self.weighted;
    }
    
    pub fn remove_edge(
        &mut self,
        _py: Python,
        edge: Vec<usize>
    ) -> PyResult<()> {
        let mut sorted_edge = edge.clone();
        sorted_edge.sort_unstable();
        let edge_str = format!("{:?}", sorted_edge);

        if let Some(edge_id) = self.attr.get_id_by_object(&edge_str) {
            
            self.edge_list.remove(&sorted_edge);
            let order = sorted_edge.len() - 1;
            
            if let Some(order_edges) = self.edges_by_order.get_mut(&order) {
                order_edges.remove(&sorted_edge);
                if order_edges.is_empty() {
                    self.edges_by_order.remove(&order);
                }
            }

            for node in &sorted_edge {
                if let Some(adj_edges) = self.adj.get_mut(node) {
                    adj_edges.remove(&edge_id);
                    if adj_edges.is_empty() {
                        self.adj.remove(node);
                    }
                }
            }

            self.attr.remove_object(&edge_str);

            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Edge not in hypergraph",
            ))
        }
    }
    
    pub fn remove_edges(&mut self, _py: Python, edges: Vec<Vec<usize>>) -> PyResult<()>{
        for edge in edges{
            let _ = self.remove_edge(_py, edge);
        }
        Ok(())
    }

// da testare --- 
    #[pyo3(signature = (node, keep_edges = None))]
    pub fn remove_node(
        &mut self,
        _py: Python,
        node: usize,
        keep_edges: Option<bool>
    ) -> PyResult<()> {
        let keep_edges = keep_edges.unwrap_or(false);

        if self.adj.contains_key(&node) {
            let node_str = node.to_string();

            if !keep_edges {
                if let Some(edges) = self.adj.remove(&node) {
                    for edge_id in edges {
                        if let Some(edge_str) = self.attr.get_object_by_id(edge_id) {
                            let edge: Vec<usize> = edge_str[1..edge_str.len() - 1]
                                .split(", ")
                                .filter_map(|s| s.parse().ok())
                                .collect();
                            self.remove_edge(_py, edge)?;
                        }
                    }
                }
            } else {
                self.adj.remove(&node);
            }

            self.attr.remove_object(&node_str);
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Node not in hypergraph",
            ))
        }
    }

    #[pyo3(signature = (nodes, keep_edges = None))]
    pub fn remove_nodes(
        &mut self,
        _py: Python,
        nodes: Vec<usize>,
        keep_edges: Option<bool>
    ) -> PyResult<()> {
        let keep_edges = keep_edges.unwrap_or(false);

        for node in nodes {
            self.remove_node(_py, node, Some(keep_edges))?;
        }
        Ok(())
    }
    
    pub fn is_uniform(&self) -> bool {
        return self.edges_by_order.len() == 1;
    }


    pub fn max_order(&self) -> usize{
        return self.max_order;
    }

    pub fn max_size(&self) -> usize{
        return self.max_order + 1;
    }

    // pub fn num_nodes(&self, py: Python) -> PyResult<usize> {
    //     let nodes = self.get_nodes(py, false)?;
    //     Ok(nodes.len())
    // }
    //restituisce il nimero di chiavi presenti in 'adj' che rappresenta il numero do nodi nell'ipergrafo
    pub fn num_nodes(&self) -> usize {
        self.adj.len()
    }
    
    #[pyo3(signature = (order = None, size = None, up_to = false))]
    pub fn num_edges(
        &self, 
        _py: Python, 
        order: Option<usize>, 
        size: Option<usize>, 
        up_to: bool
    ) -> PyResult<usize> {
        if order.is_some() && size.is_some() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Order and size cannot be both specified.",
            ));
        }

        if order.is_none() && size.is_none() {
            return Ok(self.edge_list.len());
        } else {
            let order = if let Some(size) = size {
                size - 1
            } else {
                order.unwrap()
            };

            if !up_to {
                match self.edges_by_order.get(&order) {
                    Some(edges) => Ok(edges.len()),
                    None => Ok(0),
                }
            } else {
                let mut count = 0;
                for i in 0..=order {
                    if let Some(edges) = self.edges_by_order.get(&i) {
                        count += edges.len();
                    }
                }
                Ok(count)
            }
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
    // pub fn set_meta() -> PyResult<PyObject>{}
    // pub fn set_weight() -> PyResult<PyObject>{}
    // pub fn subhypergraph() -> PyResult<PyObject>{}
    // pub fn subhypergraph_by_orders() -> PyResult<PyObject>{}
    // pub fn subhypergraph_largest_component() -> PyResult<PyObject>{}
}

