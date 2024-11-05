use pyo3::exceptions::PyValueError;
use pyo3::{exceptions, prelude::*};
use pyo3::types::{PyDict, PyList, PyString};
use std::collections::HashMap;
use super::hypergraph_rust::HypergraphRust;

#[pyclass]
#[derive(Clone)]
pub struct Hypergraph {
    pub inner: HypergraphRust,
}

#[pymethods]
impl Hypergraph {
    #[new]
    #[pyo3(signature = (edge_list=None, weighted=false, weights=None, metadata=None))]
    pub fn new(
        edge_list: Option<Vec<Vec<usize>>>,
        weighted: bool,
        weights: Option<Vec<f64>>,
        metadata: Option<HashMap<String, String>>,
    ) -> PyResult<Self> {
        let hypergraph = HypergraphRust::new(
            edge_list,
            weighted,
            weights,
            metadata    
        );
        
        Ok(Hypergraph { inner: hypergraph })
    }

    #[pyo3(signature = (edge, weight = None, metadata = None))]
    pub fn add_edge(
        &mut self,
        edge: Vec<usize>,
        weight: Option<f64>,
        metadata: Option<HashMap<String, String>>,
    ) -> PyResult<()> {
        self.inner.add_edge(edge, weight, metadata).
            map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))
    }

    #[pyo3(signature = (edges, weights=None, metadata=None))]
    pub fn add_edges(
        &mut self,
        edges: Vec<Vec<usize>>,
        weights: Option<Vec<f64>>,
        metadata: Option<HashMap<String, String>>,
    ) -> PyResult<()> {
        self.inner.add_edges(edges, weights, metadata).
            map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))
    }

    pub fn add_node(&mut self, node: usize) {
        self.inner.add_node(node)
    }

    pub fn add_nodes(&mut self, nodes: Vec<usize>) {
        self.inner.add_nodes(nodes)
    }

    pub fn get_nodes(&self, py: Python, metadata: bool) -> PyResult<PyObject> {
        
        if metadata {
            let nodes = self.inner.get_nodes_with_metadata();
            Ok(nodes.into_py(py))
        }  else {
            let nodes = self.inner.get_nodes_without_metadata();
            Ok(nodes.into_py(py))
        }
        
    }

    pub fn get_meta(&self, py: Python, obj_id: usize) -> PyResult<Option<PyObject>> {
        match self.inner.get_meta(obj_id) {
            Some(meta) => {
                let dict = PyDict::new_bound(py);
                for (k, v) in meta {
                    dict.set_item(k, v)?;
                }
                Ok(Some(dict.into()))
            }
            None => Ok(None),
        }
    }

    #[pyo3(signature = (ids = false, order = None, size = None, up_to = false))]
    pub fn get_edges(
        &self,
        py: Python,
        ids: bool,
        order: Option<usize>,
        size: Option<usize>,
        up_to: bool
    ) -> PyResult<Py<PyList>> {
        let edges = self.inner.get_edges(ids, order, size, up_to);
        let py_edges = PyList::new_bound(py, edges);
        Ok(py_edges.into())
    }

    pub fn get_edges_metadata(&self) -> Vec<(Vec<usize>, HashMap<String, String>)> {
        self.inner.get_edges_metadata()
    }

    pub fn is_weighted(&self) -> bool {
        self.inner.is_weighted()
    }

    pub fn remove_edge(&mut self, _py: Python, edge: Vec<usize>) -> PyResult<()> {
        self.inner.remove_edge(edge)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))
    }

    pub fn remove_edges(&mut self, _py: Python, edges: Vec<Vec<usize>>) {
        self.inner.remove_edges(edges);
    }

    #[pyo3(signature = (node, keep_edges = None))]
    pub fn remove_node(
        &mut self,
        _py: Python,
        node: usize,
        keep_edges: Option<bool>,
    ) -> PyResult<()> {
        let keep_edges = keep_edges.unwrap_or(false);
        self.inner.remove_node(node, keep_edges)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _> (e))
    }

    #[pyo3(signature = (nodes, keep_edges = None))]
    pub fn remove_nodes(
        &mut self,
        _py: Python,
        nodes: Vec<usize>,
        keep_edges: Option<bool>,
    ) {
        let keep_edges = keep_edges.unwrap_or(false);

        self.inner.remove_nodes(nodes, keep_edges);
    }

    pub fn is_uniform(&self) -> bool {
        return self.inner.is_uniform();
    }

    pub fn max_order(&self) -> usize {
        return self.inner.max_order();
    }

    pub fn max_size(&self) -> usize {
        return self.inner.max_size();
    }

    pub fn num_nodes(&self) -> usize {
        return self.inner.num_nodes();
    }

    #[pyo3(signature = (order = None, size = None, up_to = false))]
    pub fn num_edges(
        &self,
        _py: Python,
        order: Option<usize>,
        size: Option<usize>,
        up_to: bool,
    ) -> PyResult<usize> {
        // Controllo se sia `order` che `size` sono specificati
        if order.is_some() && size.is_some() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Order and size cannot both be specified.",
            ));
        }

        // Chiama la funzione Rust `num_edges` interna con i parametri corretti
        match self.inner.num_edges(order, size, up_to) {
            Ok(num) => Ok(num),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(e)),
        }
    }

    pub fn check_edge(&self, edge: Vec<usize>) -> bool {
        self.inner.check_edge(edge)
    }

    pub fn check_node(&self, node: usize) -> bool {
        self.inner.check_node(node)
    }

    pub fn copy(&self, _py: Python) -> PyResult<Hypergraph> {
        let new_hypergraph = self.inner.copy();
        Ok( Hypergraph { inner: new_hypergraph } )
    }

    pub fn set_meta(
        &mut self,
        _py: Python,
        obj_id: usize,
        metadata: HashMap<String, String>,
    ) -> PyResult<()> {
        let _ = self.inner.set_meta(obj_id, metadata);
        Ok(())
    }

    pub fn get_sizes(&self, py: Python) -> PyResult<PyObject> {
        let sizes = self.inner.get_sizes();
        Ok(PyList::new_bound(py, sizes).into()) 
    }

    fn distribution_sizes(&self) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            let dist = self.inner.distribution_sizes();
            let dict = PyDict::new_bound(py);
            for (k, v) in dist {
                dict.set_item(k, v)?;
            }
            Ok(dict.to_object(py))
        })
    }

    pub fn get_orders(&self, py: Python) -> PyResult<PyObject> {
        let orders = self.inner.get_orders();
        Ok(PyList::new_bound(py, orders).into())
    }

    pub fn get_attr_meta(&self, py: Python, obj: usize, attr: String) -> PyResult<PyObject> {
        match self.inner.get_attr_meta(obj, attr) {
            Ok(value) => {
                Ok(PyString::new_bound(py, value).into_py(py))
            }
            Err(err_msg) => {
                Err(PyValueError::new_err(err_msg))
            }
        }
    }

    #[pyo3(signature = (node, order=None, size=None))]
    fn get_incident_edges(
        &self,
        py: Python,
        node: usize,
        order: Option<usize>,
        size: Option<usize>,
    ) -> PyResult<Py<PyList>> {
        // Gestisci l'errore nel caso in cui sia `order` che `size` siano specificati
        if order.is_some() && size.is_some() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Order and size cannot both be specified.",
            ));
        }

        match self.inner.get_incident_edges(node, order, size) {
            Ok(edges) => Ok(PyList::new_bound(py, edges).into()),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(e)),
        }
    }

    pub fn get_weight(&self, py: Python, edge: Vec<usize>) -> PyResult<PyObject> {
        match self.inner.get_weight(edge) {
            Ok(weight) => {
                Ok(weight.into_py(py))
            }
            Err(err_msg) => {
                Err(PyValueError::new_err(err_msg))
            }
        }
    }

    pub fn set_weight(&mut self, _py: Python, edge: Vec<usize>, weight: f64) -> PyResult<()> {
        
        match self.inner.set_weight(edge, weight) {
            Ok(_) => Ok(()),  
            Err(e) => Err(PyErr::new::<exceptions::PyValueError, _>(e)),  
        }
    }

    #[pyo3(signature = (node, order = None, size = None))]
    pub fn get_neighbors(
        &self,
        py: Python,
        node: usize,
        order: Option<usize>,
        size: Option<usize>,
    ) -> PyResult<PyObject> {
        if order.is_some() && size.is_some() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Order and size cannot both be specified.",
            ));
        }

        match self.inner.get_neighbors(node, order, size) {
            Ok(neighbors) => Ok(PyList::new_bound(py, neighbors).into()),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(e)),
        }
    }
    
    #[pyo3(signature = (order = None, size = None, up_to = false))]
    pub fn get_weights(
        &self,
        _py: Python,
        order: Option<usize>,
        size: Option<usize>,
        up_to: bool,
    ) -> PyResult<Vec<f64>> {
        
        match self.inner.get_weights(order, size, up_to) {
            Ok(weights) => {
                Ok(weights)
            }
            Err(err_msg) => {
                Err(PyErr::new::<PyValueError, _>(err_msg))
            }
        }
    }

    #[pyo3(name = "get_mapping")]
    pub fn get_mapping(&self, py: Python) -> PyResult<PyObject> {
        self.inner
            .get_mapping()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))
            .map(|m| m.into_py(py))
    }

    // pub fn subhypergraph(&self, nodes: Vec<usize>) -> PyResult<Hypergraph> {
    //     let subgraph = self.inner.subhypergraph(nodes);
    //     Ok(Hypergraph { inner: subgraph })  
    //     // match self.inner.subhypergraph(nodes) {
    //     //     Ok(subgraph) => Ok(Hypergraph { inner: subgraph }),
    //     //     Err(err_msg) => Err(PyValueError::new_err(err_msg)),
    //     // }
    // }

    // #[pyo3(signature = (orders = None, sizes = None, keep_nodes = true))]
    // pub fn subhypergraph_by_orders(
    //     &self,
    //     py: Python,
    //     orders: Option<Vec<usize>>,
    //     sizes: Option<Vec<usize>>,
    //     keep_nodes: bool,
    // ) -> PyResult<Self> {
    //     if orders.is_none() && sizes.is_none() {
    //         return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
    //             "At least one of orders or sizes must be specified",
    //         ));
    //     }
    //     if orders.is_some() && sizes.is_some() {
    //         return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
    //             "Orders and sizes cannot both be specified.",
    //         ));
    //     }

    //     let mut subgraph = Hypergraph {
    //         attr: MetaHandler::new(),
    //         weighted: self.weighted,
    //         edges_by_order: HashMap::new(),
    //         adj: HashMap::new(),
    //         max_order: 0,
    //         edge_list: HashMap::new(),
    //     };

    //     // Store nodes as Rust types directly
    //     let nodes: Vec<(usize, HashMap<String, String>)> = if keep_nodes {
    //         let nodes_py = self.get_nodes(py, true)?;
    //         let nodes_with_metadata: Vec<(usize, PyObject)> = nodes_py.extract(py)?;

    //         nodes_with_metadata
    //             .into_iter()
    //             .map(|(node, meta_py)| {
    //                 let meta: HashMap<String, String> = meta_py.extract(py).unwrap();
    //                 (node, meta)
    //             })
    //             .collect()
    //     } else {
    //         Vec::new()
    //     };

    //     // Add nodes to the subgraph
    //     for (node, meta) in nodes {
    //         subgraph.add_node(py, node)?;
    //         subgraph.set_meta(py, node, meta)?;
    //     }

    //     let sizes = sizes.unwrap_or_else(|| orders.unwrap().iter().map(|&order| order + 1).collect());

    //     // Process edges
    //     for size in sizes {
    //         let edges_py: PyObject = self.get_edges(py, false, None, Some(size), false, false, false)?;

    //         // Effettua il downcast a PyList
    //         let edges = edges_py.downcast_bound::<PyList>(py).map_err(|e| {
    //             PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!("Errore nel downcast: {:?}", e))
    //         })?;


    //         for edge_py in edges.iter() {
    //             let edge_list: Vec<usize> = edge_py.extract()?;
    //             let weight = if subgraph.weighted {
    //                 Some(self.get_weight(py, edge_list.clone())?)
    //             } else {
    //                 None
    //             };

    //             // Get metadata only once
    //             let meta_py = self.get_meta(py, edge_list[0]);
    //             let meta = meta_py.map(|m| m.extract::<HashMap<String, String>>(py)).transpose()?;

    //             // Add edge with weight and metadata
    //             subgraph.add_edge(py, edge_list.clone(), weight, meta)?;
    //         }
    //     }

    //     // Convert the subgraph back to Python types if needed
    //     Ok(subgraph)
    // }


    fn __str__(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }
}

