use std::collections::HashSet;

/// Computes the intersection size of two hash sets.
/// 
/// # Arguments
///
/// * `a` - A reference to the first hash set.
/// * `b` - A reference to the second hash set.
///
/// # Returns
///
/// * The number of elements that are common to both hash sets.
pub fn intersection_rust(a: &HashSet<usize>, b: &HashSet<usize>) -> usize {
    a.intersection(b).count()
}

/// Computes the Jaccard similarity between two hash sets.
///
/// Jaccard similarity is defined as the size of the intersection divided by the size of the union of the sample sets.
///
/// # Arguments
///
/// * `a` - A reference to the first hash set.
/// * `b` - A reference to the second hash set.
///
/// # Returns
///
/// * The Jaccard similarity as a floating-point number.
pub fn jaccard_similarity_rust(a: &HashSet<usize>, b: &HashSet<usize>) -> f64 {
    intersection_rust(a, b) as f64 / (a.len() + b.len() - intersection_rust(a, b)) as f64
}

/// Computes the Jaccard distance between two hash sets.
///
/// Jaccard distance is defined as 1 minus the Jaccard similarity.
///
/// # Arguments
///
/// * `a` - A reference to the first hash set.
/// * `b` - A reference to the second hash set.
///
/// # Returns
///
/// * The Jaccard distance as a floating-point number.
pub fn jaccard_distance_rust(a: &HashSet<usize>, b: &HashSet<usize>) -> f64 {
    1.0 - jaccard_similarity_rust(a, b)
}

