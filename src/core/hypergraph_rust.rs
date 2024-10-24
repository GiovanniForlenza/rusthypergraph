use super::meta_handler::MetaHandler;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct HypergraphRust {
    attr: MetaHandler<String>,
    weighted: bool,
    edges_by_order: HashMap<usize, HashSet<Vec<usize>>>,
    adj: HashMap<usize, HashSet<usize>>,
    max_order: usize,
    edge_list: HashMap<Vec<usize>, f64>,
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

                hypergraph.add_edge(edge.clone(), Some(weights[i]), Some(edge_metadata_map)).unwrap();
            }
        }

        hypergraph
    }

    pub fn add_edge(
        &mut self,
        edge: Vec<usize>,
        weight: Option<f64>,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<(), String> {
        match (self.weighted, weight) {
            (true, None) => return Err("If the hypergraph is weighted, a weight must be provided.".to_string()),
            (false, Some(_)) => return Err("If the hypergraph is not weighted, no weight must be provided.".to_string()),
            _ => {}
        }

        let mut sorted_edge = edge;
        sorted_edge.sort_unstable();
        let edge_str = format!("{:?}", sorted_edge);

        let edge_idx = self.attr.add_obj(edge_str.clone(), Some("edge".to_string()));
        if let Some(metadata) = metadata {
            let _ = self.attr.set_attr(&edge_str, metadata);
        }

        let order = sorted_edge.len() - 1;
        self.max_order = self.max_order.max(order);

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
            self.add_node(node);
            self.adj
                .entry(node)
                .or_insert_with(HashSet::new)
                .insert(edge_idx);
        }

        Ok(())
    }

    pub fn add_edges(
        &mut self,
        edges: Vec<Vec<usize>>,
        weights: Option<Vec<f64>>,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<(), String> {
        // Controllo dei pesi per grafi ponderati e non ponderati
        match (self.weighted, &weights) {
            (true, None) => return Err("Weights must be provided for a weighted hypergraph.".to_string()),
            (false, Some(_)) => return Err("Weights should not be provided for an unweighted hypergraph.".to_string()),
            _ => {}
        }
    
        // Verifica che il numero di pesi corrisponda al numero di spigoli
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
    
            // Se l'arco esiste già, aggiorna il peso e i metadati
            if self.edge_exists(&edge) {
                self.update_edge(edge, weight, Some(edge_metadata_map))?;
            } else {
                // Altrimenti, aggiungi l'arco
                self.add_edge(edge, weight, Some(edge_metadata_map))?;
            }
        }
    
        Ok(())
    }

    fn edge_exists(&self, edge: &Vec<usize>) -> bool {
        let mut sorted_edge = edge.clone();
        sorted_edge.sort_unstable();
        // println!("Checking edge existence: {:?}", sorted_edge);
        self.edge_list.contains_key(&sorted_edge)
    }

    fn update_edge(
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

    pub fn add_node(&mut self, node: usize) {
        if !self.adj.contains_key(&node) {
            self.adj.insert(node, HashSet::new());
            let mut attributes = HashMap::new();
            attributes.insert("type".to_string(), "node".to_string());
            attributes.insert("name".to_string(), node.to_string());
            self.attr.add_object(node.to_string(), Some(attributes));
        }
    }

    pub fn add_nodes(&mut self, nodes: Vec<usize>) {
        for node in nodes {
            let _ = self.add_node(node);
        }
    }

    // pub fn get_nodes(&self, metadata: bool) -> Vec<(usize, Option<HashMap<String, String>>)> {
    //     if !metadata {
    //         self.adj.keys().map(|&node| (node, None)).collect()
    //     } else {
    //         self.adj.keys().filter_map(|&node| {
    //             if let Some(attributes) = self.attr.get_attributes(node) {
    //                 if attributes.get("type") == Some(&"node".to_string()) {
    //                     Some((node, Some(attributes.clone())))
    //                 } else {
    //                     None
    //                 }
    //             } else {
    //                 None
    //             }
    //         }).collect()
    //     }
    // }

    pub fn get_nodes_without_metadata(&self) -> Vec<usize> {
        self.adj.keys().copied().collect()
    }

    pub fn get_nodes_with_metadata(&self) -> Vec<(usize, HashMap<String, String>)> {
        self.adj.keys().filter_map(|&node| {
            if let Some(attributes) = self.attr.get_attributes(node) {
                if attributes.get("type") == Some(&"node".to_string()) {
                    Some((node, attributes.clone()))
                } else {
                    None
                }
            } else {
                None
            }
        }).collect()
    }

    pub fn get_meta(&self, obj_id: usize) -> Option<HashMap<String, String>> {
        self.attr.get_attributes(obj_id).cloned()
    }

    // pub fn get_edges(
    //     &self,
    //     ids: bool,
    //     order: Option<usize>,
    //     size: Option<usize>,
    //     up_to: bool,
    // ) -> Result<Vec<&Vec<usize>>, String> {
    //     if order.is_some() && size.is_some() {
    //         return Err("Order and size cannot both be specified.".to_string());
    //     }

    //     let mut edges = Vec::new();

    //     if order.is_none() && size.is_none() {
    //         if ids {
    //             edges = self.edge_list.keys().collect();
    //         } else {
    //             edges = self.edge_list.keys().collect();
    //         }
    //     } else {
    //         let target_order = size.map(|s| s - 1).or(order).unwrap();

    //         if !up_to {
    //             if let Some(order_edges) = self.edges_by_order.get(&target_order) {
    //                 edges.extend(order_edges.iter());
    //             }
    //         } else {
    //             for i in 0..=target_order {
    //                 if let Some(order_edges) = self.edges_by_order.get(&i) {
    //                     edges.extend(order_edges.iter());
    //                 }
    //             }
    //         }
    //     }

    //     Ok(edges)
    // }

    #[inline]
    fn get_all_edges(&self) -> Vec<&Vec<usize>> {
        // Prealloca il vettore con la capacità del numero di spigoli presenti
        let mut edges = Vec::with_capacity(self.edge_list.len());

        edges.extend(self.edge_list.keys());
        
        
        edges
    }

    pub fn get_edges(
        &self,
        ids: bool,
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

    pub fn is_weighted(&self) -> bool {
        return self.weighted;
    }

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

    pub fn remove_edges(&mut self, edges: Vec<Vec<usize>>) {
        for edge in edges {
            let _ = self.remove_edge(edge);
        }
    }

    pub fn remove_node(
        &mut self,
        node: usize,
        keep_edges: bool,
    ) -> Result<(), String> {
        // Prova a rimuovere il nodo da adj
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

    pub fn remove_nodes(
        &mut self,
        nodes: Vec<usize>,
        keep_edges: bool
    ) {
        for node in nodes {
            let _ = self.remove_node(node, keep_edges);
        }
    }

    pub fn is_uniform(&self) -> bool {
        return self.edges_by_order.len() == 1;
    }

    pub fn max_order(&self) -> usize {
        return self.max_order;
    }

    pub fn max_size(&self) -> usize {
        return self.max_order + 1;
    }

    pub fn num_nodes(&self) -> usize {
        return self.adj.len()
    }

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

    pub fn check_edge(&self, edge: Vec<usize>) -> bool {
        let mut sorted_edge = edge.clone();
        sorted_edge.sort_unstable();
        self.edge_list.contains_key(&sorted_edge)
    }

    pub fn check_node(&self, node: usize) -> bool {
        self.adj.contains_key(&node)
    }

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

    pub fn set_meta(&mut self, obj_id: usize, metadata: HashMap<String, String>) -> Result<(), String> {
        if let Some(_obj) = self.attr.get_object_by_id(obj_id) {
            self.attr.set_attributes_by_id(obj_id, metadata);
            Ok(())
        } else {
            Err(format!("Object ID {} not found in hypergraph", obj_id))
        }
    }

    pub fn get_sizes(&self) -> Vec<usize>{
        let mut sizes: Vec<usize> = self.edge_list.keys().map(|edge| edge.len()).collect();
        sizes.sort_unstable();
        sizes 
    }

    pub fn distribution_sizes(&self) -> HashMap<usize, usize> {
        let mut size_distribution = HashMap::new();
        for edge in self.edge_list.keys() {
            let size = edge.len();
            *size_distribution.entry(size).or_insert(0) += 1;
        }
        size_distribution
    }

    pub fn get_orders(&self) -> Vec<usize> {
        let mut orders: Vec<usize> = self.edge_list.keys().map(|edge| edge.len() - 1).collect();
        orders.sort_unstable();
        orders
    }

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

    pub fn get_incident_edges(
        &self,
        node: usize,
        order: Option<usize>,
        size: Option<usize>,
    ) -> Result<Vec<Vec<usize>>, String> {
        // Determina l'ordine (calcolato da size o direttamente da order)
        let target_order = size.map_or(order, |s| Some(s - 1));
    
        // Recupera gli spigoli incidenti sul nodo
        let incident_edges: Vec<Vec<usize>> = self.adj.get(&node).map_or(Vec::new(), |edges| {
            edges
                .iter()
                .filter_map(|&edge_id| {
                    // Recupera l'oggetto corrispondente all'ID dello spigolo
                    self.attr.get_object_by_id(edge_id).and_then(|edge_str| {
                        let edge: Vec<usize> = edge_str[1..edge_str.len() - 1]
                            .split(", ")
                            .filter_map(|s| s.parse().ok())
                            .collect();
    
                        // Filtra in base all'ordine, se specificato
                        if let Some(order) = target_order {
                            if edge.len() == order + 1 {
                                Some(edge)
                            } else {
                                None
                            }
                        } else {
                            Some(edge)
                        }
                    })
                })
                .collect()
        });
    
        Ok(incident_edges)
    }

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


    pub fn get_neighbors(
        &self,
        node: usize,
        order: Option<usize>,
        size: Option<usize>,
    ) -> Result<Vec<usize>, String> {
        // Recupera gli spigoli incidenti, gestendo l'errore se presente
        let edges = self.get_incident_edges(node, order, size)?;
    
        // Set per memorizzare i vicini, evitando duplicati
        let mut neighbors: HashSet<usize> = HashSet::new();
    
        // Itera sugli spigoli e inserisce i vicini nel set
        for edge_vec in edges {
            for &neighbor in &edge_vec {
                if neighbor != node {
                    neighbors.insert(neighbor);
                }
            }
        }
    
        // Converte il set in un vettore e lo restituisce
        Ok(neighbors.into_iter().collect::<Vec<_>>())
    }

    
    // da rivedere 
    pub fn get_weights(
        &self,
        order: Option<usize>,
        size: Option<usize>,
        up_to: bool,
    ) -> Result<Vec<f64>, String> {
        // Controlla se sia `order` che `size` sono specificati
        if order.is_some() && size.is_some() {
            return Err("Order and size cannot be both specified.".to_string());
        }
    
        let mut weights: Vec<f64> = Vec::new();
    
        // Caso 1: né `order` né `size` sono specificati -> restituisce tutti i pesi
        if order.is_none() && size.is_none() {
            weights.extend(self.edge_list.values().cloned());
        } else {
            // Calcola l'ordine target
            let target_order = size.or(order).map_or(0, |val| val - 1);
    
            if up_to {
                // Restituisce i pesi di tutti gli spigoli fino a `target_order`
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
                // Restituisce i pesi degli spigoli di esattamente `target_order`
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

    pub fn subhypergraph(&self, nodes: Vec<usize>) -> HypergraphRust {
        let mut subgraph = HypergraphRust {
            attr: MetaHandler::new(),
            weighted: self.weighted,
            edges_by_order: HashMap::new(),
            adj: HashMap::new(),
            max_order: 0,
            edge_list: HashMap::new(),
        };
    
        // Aggiungi i nodi al sottografo
        subgraph.add_nodes(nodes.clone());
    
        // Copia i metadati dei nodi nel sottografo
        for &node in &nodes {
            if let Some(meta) = self.get_meta(node) {
                if let Err(e) = subgraph.set_meta(node, meta) {
                    eprintln!("Error setting metadata for node {}: {:?}", node, e);
                }
            } else {
                eprintln!("No metadata found for node {}", node);
            }
        }
    
        // Processa gli spigoli
        for (edge, &weight) in &self.edge_list {
            if edge.iter().all(|&node| nodes.contains(&node)) {
                // Verifica che tutti i nodi dell'arco abbiano metadati
                if edge.iter().all(|&node| self.get_meta(node).is_some()) {
                    if let Some(metadata) = self.get_meta(edge[0]) {
                        let result = if self.weighted {
                            subgraph.add_edge(edge.clone(), Some(weight), Some(metadata))
                        } else {
                            subgraph.add_edge(edge.clone(), None, Some(metadata))
                        };
    
                        if let Err(e) = result {
                            eprintln!("Error adding edge {:?} to subgraph: {:?}", edge, e);
                        }
                    }
                } else {
                    eprintln!("One or more nodes in edge {:?} do not have metadata", edge);
                }
            } else {
                let missing_nodes: Vec<_> = edge
                    .iter()
                    .filter(|&&node| !nodes.contains(&node))
                    .collect();
                eprintln!("Skipping edge {:?} due to missing nodes: {:?}", edge, missing_nodes);
            }
        }
    
        subgraph
    }

    // pub fn subhypergraph_by_orders(
    //     &self,
    //     orders: Option<Vec<usize>>,
    //     sizes: Option<Vec<usize>>,
    //     keep_nodes: bool,
    // ) -> Result<Hypergraph, String> {
    //     if orders.is_none() && sizes.is_none() {
    //         return Err(String::from(
    //             "At least one of orders or sizes must be specified",
    //         ));
    //     }
    //     if orders.is_some() && sizes.is_some() {
    //         return Err(String::from(
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
    //         let nodes_with_metadata = self.get_nodes( true);

    //         nodes_with_metadata
    //             .into_iter()
    //             .map(|(node, meta_py)| {
    //                 let meta: HashMap<String, String> = meta_py.extract().unwrap();
    //                 (node, meta)
    //             })
    //             .collect()
    //     } else {
    //         Vec::new()
    //     };

    //     // Add nodes to the subgraph
    //     for (node, meta) in nodes {
    //         subgraph.add_node(node);
    //         subgraph.set_meta(node, meta);
    //     }

    //     let sizes = sizes.unwrap_or_else(|| orders.unwrap().iter().map(|&order| order + 1).collect());

    //     // Process edges
    //     for size in sizes {
    //         let edges = self.get_edges(false, None, Some(size), false, false, false)?;

    //         for edge in edges.iter() {
    //             let edge_list: Vec<usize> = edge.extract()?;
    //             let weight = if subgraph.weighted {
    //                 Some(self.get_weight(edge_list.clone())?)
    //             } else {
    //                 None
    //             };

    //             // Get metadata only once
    //             let meta = self.get_meta( edge_list[0]);

    //             // Add edge with weight and metadata
    //             subgraph.add_edge(edge_list.clone(), weight, meta)?;
    //         }
    //     }

    //     // Convert the subgraph back to Python types if needed
    //     Ok(subgraph)
    // }

    // fn get_mapping(&self) -> PyResult<LabelEncoder> {
    //     let nodes_py: PyObject = self.get_nodes(py, false)?;
    //     let nodes_list = nodes_py.downcast_bound::<PyList>(py).map_err(|e| {
    //         PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!("Errore nel downcast: {:?}", e))
    //     })?;
        
    //     let mut nodes: Vec<usize> = Vec::new();
    //     for node in nodes_list.iter() {
    //         nodes.push(node.extract::<usize>()?);
    //     }

    //     let mut encoder = LabelEncoder::new();
    //     encoder.fit(nodes);
    //     Ok(encoder)
    // }
}

impl std::fmt::Display for HypergraphRust {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dist_sizes = self.distribution_sizes();
        let edge_count = self.edge_list.len();

        write!(f, "Hypergraph with {} nodes and {} edges.\n", self.num_nodes(), edge_count)?;
        write!(f, "Distribution of hyperedge sizes: {:?}", dist_sizes)
    }
}

