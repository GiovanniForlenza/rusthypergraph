use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass]
pub struct LabelEncoder {
    mapping: HashMap<usize, usize>,
}

#[pymethods]
impl LabelEncoder {
    #[new]
    pub fn new() -> Self {
        LabelEncoder {
            mapping: HashMap::new(),
        }
    }

    pub fn fit(&mut self, nodes: Vec<usize>) {
        let mut node_sort = nodes.clone();
        node_sort.sort();
        for (i, node) in node_sort.iter().enumerate() {
            self.mapping.insert(*node, i);
        }
    }

    pub fn transform(&self, node: usize) -> Option<usize> {
        self.mapping.get(&node).cloned()
    }

    pub fn inverse_transform(&self, index: usize) -> Option<usize> {
        self.mapping.iter().find_map(|(&node, &i)| {
            if i == index {
                Some(node)
            } else {
                None
            }
        })
    }

    pub fn get_mapping(&self) -> HashMap<usize, usize> {
        self.mapping.clone()
    }
}
