// use numpy::ndarray::{Array2, Axis};
// use pyo3::prelude::*;
// use pyo3::types::PyList;
// use sprs::CsMat;
// use crate::core::hypergraph::Hypergraph;
// use rand::Rng;

// fn transition_matrix(py: Python, hg: &Hypergraph) -> PyResult<CsMat<f64>> {
//     let n = hg.num_nodes();
//     let mut t = Array2::<f64>::zeros((n, n));
    
//     let edges_py = hg.get_edges(py, false, None, None, false, false, false)?;
//     let edges_list: &PyList = edges_py.extract(py)?;

//     for edge in edges_list.iter() {
//         let edge_list: &PyList = edge.extract()?;  
//         let edge_len = edge_list.len();
//         let edge_vec: Vec<usize> = edge_list.extract()?;  
    
//         for i in 0..edge_len {
//             for j in (i + 1)..edge_len {
//                 let u = edge_vec[i];
//                 let v = edge_vec[j];
//                 t[(u, v)] += (edge_len - 1) as f64;
//                 t[(v, u)] += (edge_len - 1) as f64;
//             }
//         }
//     }
   
//     let row_sums = t.sum_axis(Axis(1));
//     for (i, row_sum) in row_sums.iter().enumerate() {
//         if *row_sum > 0.0 {
//             t.row_mut(i).mapv_inplace(|x| x / row_sum);
//         }
//     }
    
//     let mut row_ptrs = Vec::with_capacity(n + 1);
//     let mut col_indices = Vec::new();
//     let mut values = Vec::new();
    
//     row_ptrs.push(0);
//     let mut current_index = 0;

//     for i in 0..n {
//         for j in 0..n {
//             if t[(i, j)] != 0.0 {
//                 col_indices.push(j);
//                 values.push(t[(i, j)]);
//                 current_index += 1;
//             }
//         }
//         row_ptrs.push(current_index);
//     }

//     let csr_t = CsMat::new(
//         (n, n),
//         row_ptrs,
//         col_indices,
//         values,
//     );

//     Ok(csr_t)
// }

// #[pyfunction]
// pub fn random_walk(py: Python, hg: &Hypergraph, s: usize, time: usize) -> PyResult<Vec<usize>> {
//     let k = transition_matrix(py, hg)?;

//     let mut nodes = vec![s];
//     let mut rng = rand::thread_rng();

//     for _ in 0..time {
//         let current_node = *nodes.last().unwrap();
//         let row = k.outer_view(current_node).unwrap();
//         let probs: Vec<f64> = row.iter().map(|(_, &val)| val).collect();

//         if probs.is_empty() {
//             break;
//         }

//         let cumulative_probs: Vec<f64> = probs.iter().scan(0.0, |acc, &p| {
//             *acc += p;
//             Some(*acc)
//         }).collect();

//         let rand_num = rng.gen::<f64>();
//         let next_node = row.indices()
//                            .iter()
//                            .zip(cumulative_probs.iter())
//                            .find(|&(_, &cumulative_prob)| rand_num < cumulative_prob)
//                            .map(|(&index, _)| index)
//                            .unwrap_or(current_node);

//         nodes.push(next_node);
//     }

//     Ok(nodes)
// }
