use pyo3::prelude::*;
use pyo3::types::{PyList, PyDict};
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
        for (i, node) in nodes.iter().enumerate() {
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
