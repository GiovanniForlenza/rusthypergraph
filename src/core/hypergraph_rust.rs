use super::{label_encoder::LabelEncoder, meta_handler::MetaHandler};
use std::collections::{HashMap, HashSet, BTreeMap};

/// A hypergraph data structure.
#[derive(Clone)]
pub struct HypergraphRust {
    /// Metadata handler for storing attributes associated with nodes and edges.
    attr: MetaHandler<String>,
    /// Indicates whether the hypergraph is weighted.
    weighted: bool,
    /// Stores edges organized by their order.
    edges_by_order: BTreeMap<usize, HashSet<Vec<usize>>>,
    /// Adjacency list representation of the hypergraph.
    adj: rustc_hash::FxHashMap<usize, HashSet<usize>>,
    /// Maximum order of the hypergraph.
    max_order: usize,
    /// List of edges with their associated weights.
    pub edge_list: rustc_hash::FxHashMap<Vec<usize>, f64>,
}

impl HypergraphRust {
    pub fn new(
        edge_list: Option<Vec<Vec<usize>>>,
        weighted: bool,
        weights: Option<Vec<f64>>,
        metadata: Option<HashMap<String, String>>,
    ) -> HypergraphRust {
        let mut hypergraph = HypergraphRust {
            attr: MetaHandler::new(),
            weighted,
            edges_by_order: BTreeMap::new(),
            adj: rustc_hash::FxHashMap::default(),
            max_order: 0,
            edge_list: rustc_hash::FxHashMap::default(),
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

                hypergraph.add_edge(edge.clone(), Some(weights[i]), Some(edge_metadata_map)).unwrap();
            }
        }

        hypergraph
    }

    /// Adds a new edge to the hypergraph.
    ///
    /// # Arguments
    ///
    /// * `edge`: The edge to be added.
    /// * `weight`: The weight of the edge. If the hypergraph is not weighted, this argument is ignored.
    /// * `metadata`: Additional metadata associated with the edge. The metadata can be used to store extra information about the edge.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the edge was added successfully.
    /// * `Err(String)` if the edge could not be added. This can happen if the hypergraph is weighted and no weight is provided, or if the hypergraph is not weighted and a weight is provided.
    pub fn add_edge(
        &mut self,
        edge: Vec<usize>,
        weight: Option<f64>,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<(), String> {
        // Pre-allocare il vettore con la dimensione corretta
        let mut sorted_edge = Vec::with_capacity(edge.len());
        sorted_edge.extend_from_slice(&edge);
        sorted_edge.sort_unstable(); // sort_unstable è più veloce di sort

        // Usa entry API per ridurre i lookup
        self.edges_by_order
            .entry(sorted_edge.len() - 1)
            .or_insert_with(HashSet::new)
            .insert(sorted_edge.clone());

        // Usa insert invece di entry per i pesi quando possibile
        if self.weighted {
            self.edge_list.insert(sorted_edge.clone(), weight.unwrap_or(1.0));
        } else {
            *self.edge_list.entry(sorted_edge.clone()).or_insert(0.0) += 1.0;
        }

        let edge_str = format!("{:?}", sorted_edge);
    
        let edge_idx = self.attr.add_obj(edge_str.clone(), Some("edge".to_string()), metadata);
    
        let order = sorted_edge.len() - 1;
        self.max_order = self.max_order.max(order);
    
        for &node in &sorted_edge {
            // Verifica se il nodo ha già un ID nei metadati
            if self.attr.get_id_by_object(&node.to_string()).is_none() {
                let mut node_metadata = HashMap::new();
                node_metadata.insert("type".to_string(), "node".to_string());
                node_metadata.insert("name".to_string(), node.to_string());
                self.attr.add_obj(node.to_string(), Some("node".to_string()), Some(node_metadata));
            }
            
            self.adj
                .entry(node)
                .or_insert_with(HashSet::new)
                .insert(edge_idx);
        }
    
        Ok(())
    }

    /// Adds multiple edges to the hypergraph.
    ///
    /// # Arguments
    ///
    /// * `edges`: The edges to be added.
    /// * `weights`: The weights of the edges. If the hypergraph is not weighted, this argument is ignored.
    /// * `metadata`: Additional metadata associated with the edges. The metadata can be used to store extra information about the edges.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the edges were added successfully.
    /// * `Err(String)` if the edges could not be added. This can happen if the hypergraph is weighted and no weights are provided, or if the hypergraph is not weighted and weights are provided.
    pub fn add_edges(
        &mut self,
        edges: Vec<Vec<usize>>,
        weights: Option<Vec<f64>>,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<(), String> {
        match (self.weighted, &weights) {
            (true, None) => return Err("Weights must be provided for a weighted hypergraph.".to_string()),
            (false, Some(_)) => return Err("Weights should not be provided for an unweighted hypergraph.".to_string()),
            _ => {}
        }
    
        if let Some(ref w) = weights {
            if w.len() != edges.len() {
                return Err("The number of edges and weights must be the same.".to_string());
            }
        }
    
        for (i, edge) in edges.into_iter().enumerate() {
            let mut edge_metadata_map = HashMap::new();
            edge_metadata_map.insert("type".to_string(), "edge".to_string());
            edge_metadata_map.insert("name".to_string(), format!("{:?}", edge));
    
            if let Some(ref meta) = metadata {
                if let Some(meta_value) = meta.get(&i.to_string()) {
                    edge_metadata_map.insert(i.to_string(), meta_value.clone());
                }
            }
    
            let weight = weights.as_ref().map(|w| w[i]);
    
            // Gestisci ogni nodo dell'edge
            for &node in &edge {
                if self.attr.get_id_by_object(&node.to_string()).is_none() {
                    let mut node_metadata = HashMap::new();
                    node_metadata.insert("type".to_string(), "node".to_string());
                    node_metadata.insert("name".to_string(), node.to_string());
                    self.attr.add_obj(node.to_string(), Some("node".to_string()), Some(node_metadata));
                }
                self.add_node(node);
            }
    
            if self.edge_exists(&edge) {
                self.update_edge(edge, weight, Some(edge_metadata_map))?;
            } else {
                self.add_edge(edge, weight, Some(edge_metadata_map))?;
            }
        }
    
        Ok(())
    }

    /// Verifica se un arco esiste nel grafo.
    ///
    /// # Argumenti
    ///
    /// * `edge`: L'arco da verificare.
    ///
    /// # Returns
    ///
    /// * `true` se l'arco esiste.
    /// * `false` altrimenti.
    fn edge_exists(&self, edge: &Vec<usize>) -> bool {
        let mut sorted_edge = edge.clone();
        sorted_edge.sort_unstable();
        
        self.edge_list.contains_key(&sorted_edge)
    }

    /// Updates an existing edge in the hypergraph.
    ///
    /// # Arguments
    ///
    /// * `edge`: The edge to be updated.
    /// * `weight`: An optional new weight for the edge.
    /// * `metadata`: An optional new metadata map for the edge.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the edge was updated successfully.
    /// * `Err(String)` if the edge does not exist in the hypergraph.
    pub fn update_edge(
        &mut self,
        edge: Vec<usize>,
        weight: Option<f64>,
        metadata: Option<HashMap<String, String>>
    ) -> Result<(), String> {
        let mut sorted_edge = edge.clone();
        sorted_edge.sort_unstable(); // Assicurati di ordinare lo spigolo
    
        // Controlla se l'arco esiste
        if self.edge_exists(&sorted_edge) {
            // Aggiorna il peso se è fornito
            if let Some(w) = weight {
                self.edge_list.insert(sorted_edge.clone(), w);
            }
    
            // Aggiorna i metadati usando set_attr
            if let Some(meta) = metadata {
                // Usa set_attr per aggiornare i metadati
                let edge_str = format!("{:?}", sorted_edge);
                self.attr.set_attr(&edge_str, meta)?;
            }
    
            Ok(())
        } else {
            Err("Edge does not exist.".to_string())
        }
    }

    /// Adds a node to the hypergraph.
    ///
    /// # Arguments
    ///
    /// * `node`: The node to be added.
    ///
    /// # Notes
    ///
    /// This function is idempotent, i.e., adding the same node more than once
    /// will not result in duplicate nodes in the hypergraph.
    pub fn add_node(&mut self, node: usize) {
        self.adj.entry(node).or_insert_with(HashSet::new);
        
        if self.attr.get_id_by_object(&node.to_string()).is_none() {
            let mut attributes = HashMap::with_capacity(2);
            attributes.insert("type".to_string(), "node".to_string());
            attributes.insert("name".to_string(), node.to_string());
            self.attr.add_obj(node.to_string(), Some("node".to_string()), Some(attributes));
        }
    }

    /// Adds multiple nodes to the hypergraph.
    ///
    /// # Arguments
    ///
    /// * `nodes`: The nodes to be added.
    ///
    /// # Notes
    ///
    /// This function is idempotent, i.e., adding the same node more than once
    /// will not result in duplicate nodes in the hypergraph.
    pub fn add_nodes(&mut self, nodes: Vec<usize>) {
        for node in nodes {
            let _ = self.add_node(node);
        }
    }

    /// Returns all nodes in the hypergraph.
    ///
    /// # Returns
    ///
    /// A vector of all node IDs in the hypergraph.
    pub fn get_nodes_without_metadata(&self) -> Vec<usize> {
        self.adj.keys().copied().collect()
    }

    /// Returns all nodes in the hypergraph, including their metadata.
    ///
    /// # Returns
    ///
    /// A vector of tuples, where each tuple contains a node ID and its
    /// associated metadata.
    pub fn get_nodes_with_metadata(&self) -> Vec<(usize, HashMap<String, String>)> {
        self.adj.keys().filter_map(|&node| {
            if let Some(attributes) = self.attr.get_attributes(node) {
                if attributes.get("type") == Some(&"node".to_string()) {
                    Some((node, attributes.clone()))
                } else {
                    None
                }
            } else {
                println!("Nodo: {} senza attributi", node);
                None
            }
        }).collect()
    }

    /// Returns the metadata associated with the given object ID.
    ///
    /// # Arguments
    ///
    /// * `obj_id`: The ID of the object whose metadata should be retrieved.
    ///
    /// # Returns
    ///
    /// A `HashMap` containing the object's metadata, or `None` if the object
    /// is not present in the hypergraph.
    pub fn get_meta(&self, obj_id: usize) -> Option<&HashMap<String, String>> {
        let attr = self.attr.get_attributes(obj_id);
        attr
    }

    /// Returns all edges in the hypergraph, without filtering.
    ///
    /// # Returns
    ///
    /// A vector of references to all edges in the hypergraph.
    #[inline]
    pub fn get_all_edges(&self) -> Vec<&Vec<usize>> {
        // let mut edges = Vec::with_capacity(self.edge_list.len());
        // edges.extend(self.edge_list.keys());        
        // edges
        self.edge_list.keys().collect()
    }

    /// Returns all edges in the hypergraph, with optional filtering.
    ///
    /// # Arguments
    ///
    /// * `ids`: If `true`, returns the IDs of the edges instead of references to
    ///   the edges themselves.
    /// * `order`: If specified, returns only edges with this order.
    /// * `size`: If specified, returns only edges with this size (i.e., number of
    ///   nodes).
    /// * `up_to`: If `true`, returns all edges with orders up to and including
    ///   `order`, or up to and including `size - 1` if `size` is specified.
    ///
    /// # Returns
    ///
    /// A vector of references to the edges in the hypergraph that match the
    /// specified criteria, or an error message if `order` and `size` are both
    /// specified.
    pub fn get_edges(
        &self,
        _ids: bool,
        order: Option<usize>,
        size: Option<usize>,
        up_to: bool,
    ) -> Result<Vec<&Vec<usize>>, String> {
        // Verifica immediata per condizioni errate
        if order.is_some() && size.is_some() {
            return Err("Order and size cannot both be specified.".to_string());
        }

        // Caso semplice: né `order` né `size` sono specificati
        if order.is_none() && size.is_none() {
            return Ok(self.get_all_edges());
        }

        // Preallocazione per i casi con `order` o `size`
        let mut edges = Vec::with_capacity(self.edge_list.len());

        // Determina l'ordine target
        let target_order = size.map(|s| s - 1).or(order).unwrap_or(0);

        // Se `up_to` è false, prendi solo l'ordine specificato
        if !up_to {
            if let Some(order_edges) = self.edges_by_order.get(&target_order) {
                edges.extend(order_edges.iter());
            }
        } else {
            // Se `up_to` è true, prendi tutti gli ordini fino a `target_order`
            for i in 0..=target_order {
                if let Some(order_edges) = self.edges_by_order.get(&i) {
                    edges.extend(order_edges.iter());
                }
            }
        }

        Ok(edges)
    }

    /// Returns all edges in the hypergraph, along with their associated metadata.
    ///
    /// # Returns
    ///
    /// A vector of tuples, where each tuple contains a reference to an edge and a
    /// `HashMap` containing its associated metadata.
    pub fn get_edges_metadata(&self) -> Vec<(Vec<usize>, HashMap<String, String>)> {
        self.edge_list
            .iter()
            .map(|(edge, _)| {
                let edge_str = format!("{:?}", edge);
                let edge_meta = self
                    .attr
                    .get_attr(&edge_str)
                    .unwrap_or(&HashMap::new())
                    .clone();
                (edge.clone(), edge_meta)
            })
            .collect()
    }

    /// Returns `true` if the hypergraph is weighted, `false` otherwise.
    ///
    /// # Returns
    ///
    /// `true` if the hypergraph is weighted, `false` otherwise.
    pub fn is_weighted(&self) -> bool {
        return self.weighted;
    }

    /// Removes an edge from the hypergraph.
    ///
    /// # Arguments
    ///
    /// * `edge`: The edge to be removed.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the edge was removed successfully.
    /// * `Err(String)` if the edge does not exist in the hypergraph.
    pub fn remove_edge(&mut self, edge: Vec<usize>) -> Result<(), String> {
        let mut sorted_edge = edge.clone();
        sorted_edge.sort_unstable();
        let edge_str = format!("{:?}", sorted_edge);
    
        if let Some(edge_id) = self.attr.get_id_by_object(&edge_str) {
            // Rimuovi lo spigolo dalla lista degli spigoli
            self.edge_list.remove(&sorted_edge);
    
            let order = sorted_edge.len() - 1;
            if let Some(order_edges) = self.edges_by_order.get_mut(&order) {
                order_edges.remove(&sorted_edge);
                if order_edges.is_empty() {
                    self.edges_by_order.remove(&order);
                }
            }
    
            // Rimuovi lo spigolo dalle adiacenze dei nodi
            for node in &sorted_edge {
                if let Some(adj_edges) = self.adj.get_mut(node) {
                    adj_edges.remove(&edge_id);
                    if adj_edges.is_empty() {
                        self.adj.remove(node);
                    }
                }
            }
    
            // Rimuovi l'oggetto corrispondente dai metadati
            let _ = self.attr.remove_object(&edge_str);
    
            Ok(())
        } else {
            Err("Edge not found in hypergraph".to_string())
        }
    }

    /// Removes multiple edges from the hypergraph.
    ///
    /// # Arguments
    ///
    /// * `edges`: The edges to be removed.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the edges were removed successfully.
    /// * `Err(String)` if any of the edges do not exist in the hypergraph.
    pub fn remove_edges(&mut self, edges: Vec<Vec<usize>>) {
        for edge in edges {
            let _ = self.remove_edge(edge);
        }
    }

    /// Removes a node from the hypergraph.
    ///
    /// # Arguments
    ///
    /// * `node`: The node to be removed.
    /// * `keep_edges`: A boolean indicating whether to keep the edges associated with the node.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the node was removed successfully.
    /// * `Err(String)` if the node does not exist in the hypergraph.
    pub fn remove_node(
        &mut self,
        node: usize,
        keep_edges: bool,
    ) -> Result<(), String> {
        if let Some(edges) = self.adj.remove(&node) {
            if !keep_edges {
                for edge_id in edges {
                    if let Some(edge_str) = self.attr.get_object_by_id(edge_id) {
                        let edge: Vec<usize> = edge_str[1..edge_str.len() - 1]
                            .split(", ")
                            .filter_map(|s| s.parse().ok())
                            .collect();
                        // Rimuovi lo spigolo associato
                        let _ = self.remove_edge(edge);
                    }
                }
            }
            // Rimuovi il nodo dal gestore degli attributi
            self.attr.remove_object(&node.to_string()).map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err(format!("Node {} not found in hypergraph.", node))
        }
    }

    /// Removes multiple nodes from the hypergraph.
    ///
    /// # Arguments
    ///
    /// * `nodes`: The nodes to be removed.
    /// * `keep_edges`: A boolean indicating whether to keep the edges associated with the nodes.
    ///
    /// # Notes
    ///
    /// The function is idempotent, i.e., attempting to remove a node that is not in the hypergraph
    /// does not result in an error.
    pub fn remove_nodes(
        &mut self,
        nodes: Vec<usize>,
        keep_edges: bool
    ) {
        for node in nodes {
            let _ = self.remove_node(node, keep_edges);
        }
    }

    /// Checks if the hypergraph is uniform.
    ///
    /// # Returns
    ///
    /// `true` if the hypergraph has uniform edge sizes, `false` otherwise.
    pub fn is_uniform(&self) -> bool {
        self.edges_by_order.len() == 1
    }

    /// Returns the maximum order of the hypergraph.
    ///
    /// # Returns
    ///
    /// The maximum order of the hypergraph.
    pub fn max_order(&self) -> usize {
        return self.max_order;
    }

    /// Returns the maximum size of the hypergraph.
    ///
    /// # Returns
    ///
    /// The maximum size of the hypergraph, which is the maximum order plus one.
    pub fn max_size(&self) -> usize {
        self.max_order + 1
    }

    /// Returns the number of nodes in the hypergraph.
    ///
    /// # Returns
    ///
    /// The number of nodes in the hypergraph.
    pub fn num_nodes(&self) -> usize {
        return self.adj.len()
    }

    /// Returns the number of edges in the hypergraph.
    ///
    /// # Arguments
    ///
    /// * `order`: The order of the edges to be counted. If `None`, counts all edges.
    /// * `size`: The size of the edges to be counted, which is the order plus one. If `None`, counts all edges.
    /// * `up_to`: A boolean indicating whether to count all edges up to the specified order or size. If `false`, counts only edges of the specified order or size.
    ///
    /// # Returns
    ///
    /// The number of edges in the hypergraph that match the specified criteria.
    pub fn num_edges(
        &self,
        order: Option<usize>,
        size: Option<usize>,
        up_to: bool,
    ) -> Result<usize, String> {
        // Controlla se `order` e `size` sono entrambi `None`, restituisci il numero totale di spigoli
        if order.is_none() && size.is_none() {
            return Ok(self.edge_list.len());
        }
    
        // Se `size` è specificato, lo converte in `order`; altrimenti usa `order`
        let order = size.map_or(order.unwrap(), |s| s - 1);
    
        // Se `up_to` è false, conta solo gli spigoli di un ordine specifico
        if !up_to {
            match self.edges_by_order.get(&order) {
                Some(edges) => Ok(edges.len()),
                None => Ok(0),
            }
        } else {
            // Se `up_to` è true, conta gli spigoli fino all'ordine specificato
            let mut count = 0;
            for i in 0..=order {
                if let Some(edges) = self.edges_by_order.get(&i) {
                    count += edges.len();
                }
            }
            Ok(count)
        }
    }

    /// Checks if an edge exists in the hypergraph.
    ///
    /// # Arguments
    ///
    /// * `edge`: A vector representing the edge to be checked.
    ///
    /// # Returns
    ///
    /// * `true` if the edge exists in the hypergraph.
    /// * `false` otherwise.
    pub fn check_edge(&self, edge: Vec<usize>) -> bool {
        let mut sorted_edge = edge;
        sorted_edge.sort_unstable();
        self.edge_list.contains_key(&sorted_edge)
    }

    /// Checks if a node exists in the hypergraph.
    ///
    /// # Arguments
    ///
    /// * `node`: The node to be checked.
    ///
    /// # Returns
    ///
    /// * `true` if the node exists in the hypergraph.
    /// * `false` otherwise.
    pub fn check_node(&self, node: usize) -> bool {
        self.adj.contains_key(&node)
    }

    /// Creates a deep copy of the hypergraph.
    ///
    /// # Returns
    ///
    /// A new `HypergraphRust` instance with the same nodes, edges, and attributes
    /// as the current hypergraph.
    pub fn copy(&self) -> HypergraphRust {
        let new_hypergraph = HypergraphRust {
            attr: self.attr.clone(),
            weighted: self.weighted,
            edges_by_order: self.edges_by_order.clone(),
            adj: self.adj.clone(),
            max_order: self.max_order,
            edge_list: self.edge_list.clone(),
        };

        new_hypergraph
    }

    /// Sets metadata for the specified object ID in the hypergraph.
    ///
    /// # Arguments
    ///
    /// * `obj_id`: The ID of the object whose metadata is to be set.
    /// * `metadata`: A `HashMap` containing key-value pairs representing the metadata to be set.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the metadata was successfully set.
    /// * `Err(String)` if the object ID is not found in the hypergraph.
    pub fn set_meta(&mut self, obj_id: usize, metadata: HashMap<String, String>) -> Result<(), String> {
        if let Some(_obj) = self.attr.get_object_by_id(obj_id) {
            self.attr.set_attributes_by_id(obj_id, metadata);
            Ok(())
        } else {
            Err(format!("Object ID {} not found in hypergraph", obj_id))
        }
    }

    /// Returns a sorted vector of all edge sizes in the hypergraph.
    ///
    /// # Returns
    ///
    /// A vector of `usize` values, sorted in ascending order, representing the sizes of all edges in the hypergraph.
    pub fn get_sizes(&self) -> Vec<usize>{
        let sizes: Vec<usize> = self.edge_list.keys().map(|edge| edge.len()).collect();
        sizes 
    }

    /// Returns a distribution of edge sizes in the hypergraph.
    ///
    /// # Returns
    ///
    /// A `HashMap` where each key is an edge size and the corresponding value is the count of edges of that size.
    pub fn distribution_sizes(&self) -> HashMap<usize, usize> {
        let mut size_distribution = HashMap::new();
        for edge in self.edge_list.keys() {
            let size = edge.len();
            *size_distribution.entry(size).or_insert(0) += 1;
        }
        size_distribution
    }

    /// Returns a sorted vector of all edge orders in the hypergraph.
    ///
    /// # Returns
    ///
    /// A vector of `usize` values, sorted in ascending order, representing the orders of all edges in the hypergraph.
    pub fn get_orders(&self) -> Vec<usize> {
        let orders: Vec<usize> = self.edge_list.keys().map(|edge| edge.len() - 1).collect();
        orders
    }

    /// Returns the value of a specific attribute for a given object.
    ///
    /// # Arguments
    ///
    /// * `obj`: The ID of the object whose attribute value should be retrieved.
    /// * `attr`: The name of the attribute whose value should be retrieved.
    ///
    /// # Returns
    ///
    /// A `Result` containing a reference to a `String` if the object and attribute exist, or an error message otherwise.
    pub fn get_attr_meta(&self, obj: usize, attr: String) -> Result<&String, String> {
        if let Some(attributes) = self.attr.get_attributes(obj) {
            if let Some(value) = attributes.get(&attr) {
                Ok(value)
            } else {
                Err(format!(
                    "Attribute '{}' not found for object {}",
                    attr, obj
                ))
            }
        } else {
            Err(format!(
                "Object ID {} not found in hypergraph",
                obj
            ))
        }
    }

    /// Returns the incident edges of a given node in the hypergraph.
    ///
    /// # Arguments
    ///
    /// * `node`: The ID of the node whose incident edges should be retrieved.
    /// * `order`: An optional parameter specifying the order of the edges to be returned.
    /// * `size`: An optional parameter specifying the size of the edges to be returned.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of vectors of `usize` values representing the incident edges of the node
    pub fn get_incident_edges(
        &self,
        node: usize,
        order: Option<usize>,
        size: Option<usize>,
    ) -> Result<Vec<Vec<usize>>, String> {
        let target_order = size.map_or(order, |s| Some(s - 1));
        
        // Preallocare la capacità basata sulla dimensione dell'adiacenza
        let mut incident_edges = Vec::with_capacity(
            self.adj.get(&node).map_or(0, |edges| edges.len())
        );
        
        if let Some(edges) = self.adj.get(&node) {
            for &edge_id in edges {
                if let Some(edge_str) = self.attr.get_object_by_id(edge_id) {
                    let edge: Vec<usize> = edge_str[1..edge_str.len() - 1]
                        .split(", ")
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    
                    if target_order.map_or(true, |order| edge.len() == order + 1) {
                        incident_edges.push(edge);
                    }
                }
            }
        }
        
        incident_edges.sort_unstable();
        Ok(incident_edges)
    }

    /// Returns the weight of a specific edge in the hypergraph.
    ///
    /// # Arguments
    ///
    /// * `edge`: The edge whose weight should be retrieved.
    ///
    /// # Returns
    ///
    /// A `Result` containing the weight of the edge, or an error message if the edge is not in the hypergraph.
    pub fn get_weight(&self, edge: Vec<usize>) -> Result<f64, String> {
        let mut sorted_edge = edge.clone();
        sorted_edge.sort_unstable();
    
        match self.edge_list.get(&sorted_edge) {
            Some(&weight) => Ok(weight),
            None => Err(format!(
                "Edge {:?} not in hypergraph.",
                edge
            )),
        }
    }

    /// Sets the weight of a specific edge in the hypergraph.
    ///
    /// # Arguments
    ///
    /// * `edge`: The edge whose weight should be set.
    /// * `weight`: The new weight for the edge.
    ///
    /// # Returns
    ///
    /// A `Result` containing `()`, or an error message if the edge is not in the hypergraph.
    pub fn set_weight(&mut self, edge: Vec<usize>, weight: f64) -> Result<(), String> {
        let mut sorted_edge = edge.clone();
        sorted_edge.sort_unstable();

        if self.edge_list.contains_key(&sorted_edge) {
            self.edge_list.insert(sorted_edge, weight);
            Ok(())
        } else {
            Err(format!("Edge {:?} not in hypergraph.", edge))
        }
    }

    /// Returns the neighbors of a given node in the hypergraph.
    ///
    /// # Arguments
    ///
    /// * `node`: The ID of the node whose neighbors should be retrieved.
    /// * `order`: An optional parameter specifying the order of the edges to be returned.
    /// * `size`: An optional parameter specifying the size of the edges to be returned.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `usize` values representing the neighbors of the node
    pub fn get_neighbors(
        &self,
        node: usize,
        order: Option<usize>,
        size: Option<usize>,
    ) -> Result<Vec<usize>, String> {
        // Usa una FxHashSet per performance migliori
        let mut neighbors = rustc_hash::FxHashSet::default();
        
        if let Some(edges) = self.adj.get(&node) {
            for &edge_id in edges {
                if let Some(edge_str) = self.attr.get_object_by_id(edge_id) {
                    let edge: Vec<usize> = edge_str[1..edge_str.len() - 1]
                        .split(", ")
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    
                    if size.map_or(true, |s| edge.len() == s) 
                        && order.map_or(true, |o| edge.len() == o + 1) {
                        neighbors.extend(edge.iter().filter(|&&n| n != node));
                    }
                }
            }
        }
        
        Ok(neighbors.into_iter().collect())
    }

    /// Returns the weights of all edges in the hypergraph.
    ///
    /// # Arguments
    ///
    /// * `order`: An optional parameter specifying the order of the edges to be returned.
    /// * `size`: An optional parameter specifying the size of the edges to be returned.
    /// * `up_to`: A boolean value indicating whether to return weights up to the specified order or size.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `f64` values representing the weights of the edges, or an error message if the edge is not in the hypergraph.
    pub fn get_weights(
        &self,
        order: Option<usize>,
        size: Option<usize>,
        up_to: bool,
    ) -> Result<Vec<f64>, String> {
        // Controllo se entrambi `order` e `size` sono specificati
        if order.is_some() && size.is_some() {
            return Err("Order and size cannot be both specified.".to_string());
        }

        let mut weights: Vec<f64> = Vec::new();

        // Caso 1: né `order` né `size` sono specificati -> restituisce tutti i pesi
        if order.is_none() && size.is_none() {
            weights.extend(self.edge_list.values().cloned());
        } else {
            // Determina l'ordine target: usa `size - 1` se `size` è specificato, altrimenti usa `order`
            let target_order = size.map(|s| s - 1).or(order);

            // `up_to` == true, quindi restituisce tutti i pesi degli spigoli fino a `target_order`
            if up_to {
                // Itera sugli ordini da 1 fino a `target_order`
                for current_order in 0..=target_order.unwrap_or(0) {
                    if let Some(order_edges) = self.edges_by_order.get(&current_order) {
                        for edge in order_edges {
                            if let Some(&weight) = self.edge_list.get(edge) {
                                weights.push(weight);
                            }
                        }
                    }
                }
            } else {
                // `up_to` == false, restituisce solo i pesi degli spigoli di esattamente `target_order`
                if let Some(current_order) = target_order {
                    if let Some(order_edges) = self.edges_by_order.get(&current_order) {
                        for edge in order_edges {
                            if let Some(&weight) = self.edge_list.get(edge) {
                                weights.push(weight);
                            }
                        }
                    }
                }
            }
        }

        Ok(weights)
    }

    pub fn is_connected_rust(&self) -> bool {
        if self.num_nodes() == 0 {
            return true;
        }

        let mut visited = rustc_hash::FxHashSet::default();
        let mut to_visit = Vec::with_capacity(self.num_nodes());
        
        // Inizia dal primo nodo disponibile
        if let Some(&start_node) = self.adj.keys().next() {
            visited.insert(start_node);
            to_visit.push(start_node);
        } else {
            return true;
        }
        
        while let Some(node) = to_visit.pop() {
            if let Some(edges) = self.adj.get(&node) {
                for &edge_id in edges {
                    if let Some(edge_str) = self.attr.get_object_by_id(edge_id) {
                        let edge: Vec<usize> = edge_str[1..edge_str.len() - 1]
                            .split(", ")
                            .filter_map(|s| s.parse().ok())
                            .collect();
                        
                        for &neighbor in &edge {
                            if visited.insert(neighbor) {
                                to_visit.push(neighbor);
                            }
                        }
                    }
                }
            }
        }
        
        visited.len() == self.num_nodes()
    }

    /// Returns a subgraph of the hypergraph with the specified nodes.
    ///
    /// # Arguments
    ///
    /// * `nodes`: The nodes to be included in the subgraph.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `HypergraphRust` object representing the subgraph, or an error message if the nodes are not in the hypergraph.
    pub fn subhypergraph(&self, nodes: Vec<usize>) -> HypergraphRust {
        // Creare un HashSet per lookup O(1)
        let node_set: rustc_hash::FxHashSet<_> = nodes.iter().copied().collect();
        
        // Stima della capacità per le strutture dati
        let estimated_edges = (self.edge_list.len() / 2).max(16);
        let estimated_nodes = nodes.len();
        
        // Inizializzare il nuovo hypergraph con capacità pre-allocate
        let mut subgraph = HypergraphRust {
            attr: MetaHandler::new(),
            weighted: self.weighted,
            edges_by_order: BTreeMap::new(),
            adj: rustc_hash::FxHashMap::with_capacity_and_hasher(estimated_nodes, Default::default()),
            max_order: 0,
            edge_list: rustc_hash::FxHashMap::with_capacity_and_hasher(estimated_edges, Default::default()),
        };

        // Copiare i nodi e i loro metadati
        for &node in &nodes {
            if let Some(_node_attrs) = self.attr.get_attributes(node) {
                subgraph.add_node(node);
                if let Some(node_meta) = self.get_meta(node) {
                    let _ = subgraph.set_meta(node, node_meta.clone());
                }
            }
        }

        // Copiare gli archi rilevanti
        for (edge, weight) in &self.edge_list {
            // Verifica se tutti i nodi dell'arco sono nel sottoinsieme
            if edge.iter().all(|node| node_set.contains(node)) {
                let edge_str = format!("{:?}", edge);
                if let Ok(edge_meta) = self.attr.get_attr(&edge_str) {
                    subgraph.add_edge(
                        edge.clone(),
                        Some(*weight),
                        Some(edge_meta.clone())
                    ).unwrap_or_default();
                } else {
                    subgraph.add_edge(
                        edge.clone(),
                        Some(*weight),
                        None
                    ).unwrap_or_default();
                }
            }
        }

        // Aggiornare max_order
        if let Some(&max) = subgraph.edges_by_order.keys().max() {
            subgraph.max_order = max;
        }

        subgraph
    }

    pub fn get_mapping(&self) -> Result<LabelEncoder, String> {
        let nodes = self.get_nodes_without_metadata();
        
        if nodes.is_empty() {
            return Err("Errore: nessun nodo trovato.".to_string());
        }

        let mut encoder = LabelEncoder::new();
        encoder.fit(nodes);
    
        Ok(encoder)
    }
}

impl std::fmt::Display for HypergraphRust {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dist_sizes = self.distribution_sizes();
        let edge_count = self.edge_list.len();

        write!(f, "Hypergraph with {} nodes and {} edges.\n", self.num_nodes(), edge_count)?;
        write!(f, "Distribution of hyperedge sizes: {:?}", dist_sizes)
    }
}

