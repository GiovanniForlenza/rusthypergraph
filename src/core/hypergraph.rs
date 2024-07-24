use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};
use std::collections::{HashMap, HashSet};
use super::meta_handler::MetaHandler;
use super::label_encoder::LabelEncoder;


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
        let mut hypergraph = Hypergraph {
            attr: MetaHandler::new(),
            weighted,
            edges_by_order: HashMap::new(),
            adj: HashMap::new(),
            max_order: 0,
            edge_list: HashMap::new(),
        };

        if let Some(edges) = edge_list {
            let default_weights = vec![1.0; edges.len()];
            let weights = weights.unwrap_or(default_weights);

            for (i, edge) in edges.iter().enumerate() {
                let mut edge_metadata_map = HashMap::new();
                edge_metadata_map.insert("type".to_string(), "edge".to_string());
                edge_metadata_map.insert("name".to_string(), format!("{:?}", edge));

                if let Some(ref meta) = metadata {
                    if let Some(meta_value) = meta.get(&i.to_string()) {
                        edge_metadata_map.insert(i.to_string(), meta_value.clone());
                    }
                }
                
                hypergraph.add_edge(py, edge.clone(), Some(weights[i]), Some(edge_metadata_map))?;
            }
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
        sorted_edge.sort();
        let edge_str = format!("{:?}", sorted_edge);
    
        let edge_idx = self.attr.add_obj(edge_str.clone(), Some("edge".to_string()));
        if let Some(metadata) = &metadata {
            self.attr.set_attr(&edge_str, metadata.clone()).unwrap();
        }
    
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
    
        for &node in &sorted_edge {
            self.add_node(py, node)?;
            self.adj
                .entry(node)
                .or_insert_with(HashSet::new)
                .insert(edge_idx);
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
            let mut edge_metadata_map = HashMap::new();
            edge_metadata_map.insert("type".to_string(), "edge".to_string());
            edge_metadata_map.insert("name".to_string(), format!("{:?}", edge));

            if let Some(ref meta) = metadata {
                if let Some(meta_value) = meta.get(&i.to_string()) {
                    edge_metadata_map.insert(i.to_string(), meta_value.clone());
                }
            }

            self.add_edge(py, edge, weight, Some(edge_metadata_map))?;
        }
        Ok(())
    }

    pub fn add_node(&mut self, _py: Python, node: usize) -> PyResult<()> {
        if !self.adj.contains_key(&node) {
            self.adj.insert(node, HashSet::new());
            let mut attributes = HashMap::new();
            attributes.insert("type".to_string(), "node".to_string());
            attributes.insert("name".to_string(), node.to_string());
            self.attr.add_object(node.to_string(), Some(attributes));
        }
        Ok(())
    }    

    pub fn add_nodes(&mut self, py:Python, nodes: Vec<usize>) {
        for node in nodes {
            let _ =self.add_node(py, node);
        }
    }

    pub fn get_nodes(&self, py: Python, metadata: bool) -> PyResult<PyObject> {
        if !metadata {
            let nodes: Vec<usize> = self.adj.keys().cloned().collect();
            Ok(PyList::new(py, nodes).into())
        } else {
            let nodes_with_metadata: Vec<(usize, PyObject)> = self.adj.keys()
                .filter_map(|&node| {
                    if let Some(attributes) = self.attr.get_attributes(node) {
                        // Verifica se il tipo è "node"
                        if attributes.get("type") == Some(&"node".to_string()) {
                            let py_dict = PyDict::new(py);
                            for (key, value) in attributes {
                                py_dict.set_item(key, value).unwrap();
                            }
                            Some((node, py_dict.into()))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();
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
            println!("No attributes found for object ID {}", obj_id);
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
            if !ids {
                for edge in self.edge_list.keys() {
                    edges.push(PyTuple::new(py, edge).into());
                }
            } else {
                for edge in self.edge_list.keys() {
                    if let Some(edge_id) = self.attr.get_id_by_object(&PyTuple::new(py, edge).to_string()) {
                        edges.push(edge_id.into_py(py));
                    }
                }
            }
        } else {
            let mut edges_tmp: Vec<Vec<usize>> = Vec::new();
            if let Some(s) = size {
                let o = s - 1;
                if !up_to {
                    if let Some(order_edges) = self.edges_by_order.get(&o) {
                        edges_tmp.extend(order_edges.iter().cloned());
                    }
                } else {
                    for i in 0..=o {
                        if let Some(order_edges) = self.edges_by_order.get(&i) {
                            edges_tmp.extend(order_edges.iter().cloned());
                        }
                    }
                }
            } else if let Some(o) = order {
                if !up_to {
                    if let Some(order_edges) = self.edges_by_order.get(&o) {
                        edges_tmp.extend(order_edges.iter().cloned());
                    }
                } else {
                    for i in 0..=o {
                        if let Some(order_edges) = self.edges_by_order.get(&i) {
                            edges_tmp.extend(order_edges.iter().cloned());
                        }
                    }
                }
            }

            if ids {
                for edge in edges_tmp {
                    if let Some(edge_id) = self.attr.get_id_by_object(&PyTuple::new(py, edge).to_string()) {
                        edges.push(edge_id.into_py(py));
                    }
                }
            } else {
                for edge in edges_tmp {
                    edges.push(PyList::new(py, edge).into());
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

            if self.weighted {
                for edge in &edges {
                    if let Ok(edge_list) = edge.extract::<Vec<usize>>(py) {
                        if let Some(weight) = self.edge_list.get(&edge_list) {
                            subgraph.add_edge(py, edge_list.clone(), Some(*weight), None)?;
                        }
                    }
                }
            } else {
                for edge in &edges {
                    if let Ok(edge_list) = edge.extract::<Vec<usize>>(py) {
                        subgraph.add_edge(py, edge_list.clone(), None, None)?;
                    }
                }
            }

            for edge in subgraph.get_edges(py, false, None, None, false, false, false)?.extract::<Vec<Vec<usize>>>(py)? {
                if let Some(edge_id) = self.attr.get_id_by_object(&format!("{:?}", edge)) {
                    if let Some(meta) = self.attr.get_attributes(*edge_id) {
                        let meta_dict = PyDict::new(py);
                        for (key, value) in meta {
                            meta_dict.set_item(key, value)?;
                        }
                        let _ = subgraph.set_meta(py, *edge_id, meta.clone());
                    }
                }
            }

            if keep_isolated_nodes {
                for node in self.adj.keys() {
                    let _ = subgraph.add_node(py, *node);
                }
            }

            Ok(Py::new(py, subgraph)?.into_py(py))
        } else {
            Ok(PyList::new(py, edges).into())
        }
    }

    pub fn get_edges_metadata(&self) -> Vec<(Vec<usize>, HashMap<String, String>)> {
        self.edge_list.iter()
            .map(|(edge, _)| {
                let edge_str = format!("{:?}", edge);
                let edge_meta = self.attr.get_attr(&edge_str).unwrap_or(&HashMap::new()).clone();
                (edge.clone(), edge_meta)
            })
            .collect()
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
        sorted_edge.sort();
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

            let _ = self.attr.remove_object(&edge_str);

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

            let _ = self.attr.remove_object(&node_str);
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

    pub fn check_edge(&self, edge: Vec<usize>) -> bool {
        let mut sorted_edge = edge.clone();
        sorted_edge.sort();
        self.edge_list.contains_key(&sorted_edge)
    }

    pub fn check_node(&self, node: usize) -> bool {
        self.adj.contains_key(&node)
    }

    pub fn copy(&self, _py: Python) -> PyResult<Hypergraph> {
        let new_hypergraph = Hypergraph {
            attr: self.attr.clone(),
            weighted: self.weighted,
            edges_by_order: self.edges_by_order.clone(),
            adj: self.adj.clone(),
            max_order: self.max_order,
            edge_list: self.edge_list.clone(),
        };

        Ok(new_hypergraph)
    }
    
    pub fn set_meta(
        &mut self, 
        _py: Python, 
        obj_id: usize, 
        metadata: HashMap<String, String>
    ) -> PyResult<()> {
        if let Some(_obj) = self.attr.get_object_by_id(obj_id) {
            self.attr.set_attributes_by_id(obj_id, metadata);
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Object ID not found in hypergraph",
            ))
        }
    }    

    pub fn get_sizes(&self, py: Python) -> PyResult<PyObject> {
        let sizes: Vec<usize> = self.edge_list.keys().map(|edge| edge.len()).collect();
        Ok(PyList::new(py, sizes).into())
    }
    
    pub fn distribution_sizes(&self, py: Python) -> PyResult<PyObject> {
        
        let sizes: Vec<usize> = self.edge_list.keys().map(|edge| edge.len()).collect();

        let mut size_distribution: HashMap<usize, usize> = HashMap::new();
        for size in sizes {
            *size_distribution.entry(size).or_insert(0) += 1;
        }

        let py_dict = PyDict::new(py);
        for (size, count) in size_distribution {
            py_dict.set_item(size, count)?;
        }

        Ok(py_dict.into())
    }

    pub fn get_orders(&self, py: Python) -> PyResult<PyObject> {
        let orders: Vec<usize> = self.edge_list.keys().map(|edge| edge.len() - 1).collect();
        Ok(PyList::new(py, orders).into())
    }
    
    pub fn get_attr_meta(&self, py: Python, obj: usize, attr: String) -> PyResult<PyObject> {
        
        if let Some(attributes) = self.attr.get_attributes(obj) {
            if let Some(value) = attributes.get(&attr) {
                Ok(value.into_py(py))
            } else {
                Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("Attribute '{}' not found for object {}", attr, obj),
                ))
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Object ID {} not found in hypergraph", obj),
            ))
        }
    }

    fn get_incident_edges(&self, py: Python, node: usize, order: Option<usize>, size: Option<usize>) -> PyResult<Py<PyList>> {
        
        if order.is_some() && size.is_some() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Order and size cannot be both specified.",
            ));
        }

        let incident_edges: Vec<Vec<usize>> = if order.is_none() && size.is_none() {
            self.adj.get(&node)
                .map_or(Vec::new(), |edges| edges.iter().filter_map(|&edge_id| {
                    self.attr.get_object_by_id(edge_id).and_then(|edge_str| {
                        let edge: Vec<usize> = edge_str[1..edge_str.len()-1]
                            .split(", ")
                            .filter_map(|s| s.parse().ok())
                            .collect();
                        Some(edge)
                    })
                }).collect())
        } else {
           
            let order = order.or_else(|| size.map(|s| s - 1)).unwrap();

            self.adj.get(&node)
                .map_or(Vec::new(), |edges| edges.iter().filter_map(|&edge_id| {
                    self.attr.get_object_by_id(edge_id).and_then(|edge_str| {
                        let edge: Vec<usize> = edge_str[1..edge_str.len()-1]
                            .split(", ")
                            .filter_map(|s| s.parse().ok())
                            .collect();
                        if edge.len() == order + 1 {
                            Some(edge)
                        } else {
                            None
                        }
                    })
                }).collect())
        };

        Ok(PyList::new(py, incident_edges).into())
    }
    
    pub fn get_weight(&self, _py: Python, edge: Vec<usize>) -> PyResult<f64> {
        let mut sorted_edge = edge.clone();
        sorted_edge.sort();

        match self.edge_list.get(&sorted_edge) {
            Some(&weight) => Ok(weight),
            None => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Edge {:?} not in hypergraph.", edge),
            )),
        }
    }

    pub fn set_weight(&mut self, _py: Python, edge: Vec<usize>, weight: f64) -> PyResult<()> {
        let mut sorted_edge = edge.clone();
        sorted_edge.sort();

        if self.edge_list.contains_key(&sorted_edge) {
            self.edge_list.insert(sorted_edge, weight);
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Edge {:?} not in hypergraph.", edge),
            ))
        }
    }

    pub fn get_neighbors(&self, py: Python, node: usize, order: Option<usize>, size: Option<usize>) -> PyResult<PyObject> {
        if order.is_some() && size.is_some() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Order and size cannot be both specified.",
            ));
        }

        let mut neighbors = HashSet::new();

        let edges_py = self.get_incident_edges(py, node, order, size)?;
        let edges: &PyAny = edges_py.as_ref(py);

        for edge in edges.iter()? {
            let edge_vec: Vec<usize> = edge?.extract()?;
            for &neighbor in &edge_vec {
                if neighbor != node {
                    neighbors.insert(neighbor);
                }
            }
        }

        let py_neighbors = PyList::new(py, neighbors).into();

        Ok(py_neighbors)
    }
    
    #[pyo3(signature = (order = None, size = None, up_to = false))]
    pub fn get_weights(
        &self,
        _py: Python,
        order: Option<usize>,
        size: Option<usize>,
        up_to: bool,
    ) -> PyResult<Vec<f64>> {
        
        if order.is_some() && size.is_some() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Order and size cannot be both specified.",
            ));
        }

        let mut weights: Vec<f64> = Vec::new();

        
        if order.is_none() && size.is_none() {
            weights.extend(self.edge_list.values());
        } else {
            let target_order = size.map_or(order.unwrap() - 1, |s| s - 1);

            if up_to {
                for i in 1..=target_order {
                    if let Some(order_edges) = self.edges_by_order.get(&i) {
                        for edge in order_edges {
                            if let Some(weight) = self.edge_list.get(edge) {
                                weights.push(*weight);
                            }
                        }
                    }
                }
            } else {
                if let Some(order_edges) = self.edges_by_order.get(&target_order) {
                    for edge in order_edges {
                        if let Some(weight) = self.edge_list.get(edge) {
                            weights.push(*weight);
                        }
                    }
                }
            }
        }

        Ok(weights)
    }

    pub fn subhypergraph(&self, py: Python, nodes: Vec<usize>) -> PyResult<Self> {
        let mut subgraph = Hypergraph {
            attr: MetaHandler::new(),
            weighted: self.weighted,
            edges_by_order: HashMap::new(),
            adj: HashMap::new(),
            max_order: 0,
            edge_list: HashMap::new(),
        };

        subgraph.add_nodes(py, nodes.clone());
    
        for &node in &nodes {
            match self.get_meta(py, node) {
                Some(meta_py) => {
                    let meta: HashMap<String, String> = meta_py.extract(py)?;
                    println!("Metadata for node {}: {:?}", node, meta);
                    if let Err(e) = subgraph.set_meta(py, node, meta) {
                        println!("Error setting metadata for node {}: {:?}", node, e);
                    }
                }
                None => {
                    println!("No metadata found for node {}", node);
                }
            }
        }
    
        println!("Original edge list: {:?}", self.edge_list);
    
        for (edge, &weight) in &self.edge_list {
            println!("Current edge: {:?}", edge);
    
            if edge.iter().all(|&node| nodes.contains(&node)) {
                println!("Processing edge: {:?}", edge);
                let all_meta_available = edge.iter().all(|&node| self.get_meta(py, node).is_some());
                if !all_meta_available {
                    println!("One or more nodes in edge {:?} do not have metadata in the hypergraph", edge);
                    continue;
                }
    
                if let Some(metadata_py) = self.get_meta(py, edge[0]) {
                    let metadata: HashMap<String, String> = metadata_py.extract(py)?;
                    println!("Metadata for edge {:?}: {:?}", edge, metadata);
    
                    let result = if self.weighted {
                        subgraph.add_edge(py, edge.clone(), Some(weight), Some(metadata))
                    } else {
                        subgraph.add_edge(py, edge.clone(), None, Some(metadata))
                    };
    
                    if let Err(e) = result {
                        println!("Error adding edge {:?} to subgraph: {:?}", edge, e);
                    }
                }
            } else {
                let missing_nodes: Vec<_> = edge.iter().filter(|&&node| !nodes.contains(&node)).collect();
                println!("Skipping edge {:?} due to missing nodes: {:?}", edge, missing_nodes);
            }
        }
    
        Ok(subgraph)
    }
    
    #[pyo3(signature = (orders = None, sizes = None, keep_nodes = true))]
    pub fn subhypergraph_by_orders(
        &self,
        py: Python,
        orders: Option<Vec<usize>>,
        sizes: Option<Vec<usize>>,
        keep_nodes: bool,
    ) -> PyResult<Self> {
        if orders.is_none() && sizes.is_none() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Almeno uno tra orders e sizes deve essere specificato",
            ));
        }
        if orders.is_some() && sizes.is_some() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Order e size non possono essere entrambi specificati.",
            ));
        }

        let mut subgraph = Hypergraph {
            attr: MetaHandler::new(),
            weighted: self.weighted,
            edges_by_order: HashMap::new(),
            adj: HashMap::new(),
            max_order: 0,
            edge_list: HashMap::new(),
        };

        if keep_nodes {
            let nodes_py = self.get_nodes(py, true)?;
            let nodes_with_metadata: Vec<(usize, PyObject)> = nodes_py.extract(py)?;

            for (node, meta_py) in nodes_with_metadata {
                let meta: HashMap<String, String> = meta_py.extract(py)?;
                let _ = subgraph.add_node(py, node);
                let _ = subgraph.set_meta(py, node, meta);
                // match subgraph.set_meta(py, node, meta) {
                //     Ok(_) => println!("Successfully set metadata for node {}", node),
                //     Err(e) => println!("Failed to set metadata for node {}: {:?}", node, e),
                // }
            }
        }

        let sizes = if let Some(sizes) = sizes {
            sizes
        } else {
            orders.unwrap().into_iter().map(|order| order + 1).collect()
        };

        for size in sizes {
            let edges_py = self.get_edges(py, false, None, Some(size), false, false, false)?;
            let edges: &PyList = edges_py.as_ref(py).downcast::<PyList>()?;

            for edge_py in edges.iter() {
                let edge_list: Vec<usize> = edge_py.extract()?;

                if subgraph.weighted {
                    let weight = self.get_weight(py, edge_list.clone())?;
                    let meta_py = self.get_meta(py, edge_list[0]);
                    if let Some(meta_py) = meta_py {
                        let meta: HashMap<String, String> = meta_py.extract(py)?;
                        subgraph.add_edge(py, edge_list.clone(), Some(weight), Some(meta))?;
                    } else {
                        subgraph.add_edge(py, edge_list.clone(), Some(weight), None)?;
                    }
                } else {
                    let meta_py = self.get_meta(py, edge_list[0]);
                    if let Some(meta_py) = meta_py {
                        let meta: HashMap<String, String> = meta_py.extract(py)?;
                        subgraph.add_edge(py, edge_list.clone(), None, Some(meta))?;
                    } else {
                        subgraph.add_edge(py, edge_list.clone(), None, None)?;
                    }
                }
            }
        }

        Ok(subgraph)
    }

    
    fn get_mapping(&self, py: Python) -> PyResult<LabelEncoder> {
        let nodes_py = self.get_nodes(py, false)?;
        let nodes_list: &PyList = nodes_py.as_ref(py).downcast::<PyList>()?;

        let mut nodes: Vec<usize> = Vec::new();
        for node in nodes_list.iter() {
            nodes.push(node.extract::<usize>()?);
        }

        let mut encoder = LabelEncoder::new();
        encoder.fit(nodes);
        Ok(encoder)
    }
    
    fn __str__(&self, py: Python) -> PyResult<String> {
            
        let dist_sizes_py = self.distribution_sizes(py)?;
        let dist_sizes_str = dist_sizes_py.to_string();

        let edge = self.edge_list.len();
    
        let title = format!("Hypergraph with {} nodes and {} edges.\n", self.num_nodes(), edge);
        let details = format!("Distribution of hyperedge sizes: {}", dist_sizes_str);
    
        Ok(format!("{}{}", title, details))
    }

}

