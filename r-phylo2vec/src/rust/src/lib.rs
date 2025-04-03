use extendr_api::prelude::*;
use nalgebra::DMatrix;

use phylo2vec::tree_vec::ops;
use phylo2vec::utils;

/// Sample a random tree via Phylo2Vec
/// @export
#[extendr]
fn sample(n_leaves: usize, ordered: bool) -> Vec<i32> {
    let v = utils::sample_vector(n_leaves, ordered);
    v.iter().map(|&x| x as i32).collect()
}

/// Recover a rooted tree (in Newick format) from a Phylo2Vec vector
/// @export
#[extendr]
fn to_newick_from_vector(input_integers: Vec<i32>) -> String {
    let input_vector = input_integers.iter().map(|&x| x as usize).collect();
    let newick = ops::to_newick_from_vector(&input_vector);
    newick
}

/// Recover a rooted tree (in Newick format) from a Phylo2Vec matrix
/// @export
#[extendr]
fn to_newick_from_matrix(input_integers: DMatrix<i32>) -> String {
    let input_matrix = input_integers
        .iter()
        .map(|&x| x as f32)
        .collect::<Vec<f32>>()
        .chunks(input_integers.ncols())
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<Vec<f32>>>();
    let newick = ops::to_newick_from_matrix(&input_matrix);
    newick
}

/// Convert a newick string to a Phylo2Vec vector
/// @export
#[extendr]
fn to_vector(newick: &str) -> Vec<i32> {
    let v = ops::to_vector(&newick);
    v.iter().map(|&x| x as i32).collect()
}

/// Validate a Phylo2Vec vector
/// @export
#[extendr]
fn check_v(input_integers: Vec<i32>) {
    let input_vector = input_integers.iter().map(|&x| x as usize).collect();
    utils::check_v(&input_vector);
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod phylo2vec;
    fn sample;
    fn to_newick_from_vector;
    fn to_newick_from_matrix;
    fn to_vector;
    fn check_v;
}
