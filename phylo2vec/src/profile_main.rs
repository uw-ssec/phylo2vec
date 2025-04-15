use phylo2vec::{
    tree_vec::ops::{to_newick, to_vector},
    utils::sample_vector,
};
use std::env;

const DEFAULT_N_LEAVES: usize = 100000;

fn main() {
    // This is a placeholder for the main function.
    // You can add your code here to implement the functionality you need.
    let args: Vec<String> = env::args().collect();

    // Default to 5 if no argument is provided
    let n_leaves = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(DEFAULT_N_LEAVES)
    } else {
        DEFAULT_N_LEAVES
    };
    // Add your logic here
    // For example, you might want to load data, process it, and output results.
    // ...
    let v = sample_vector(n_leaves, true);
    let n = to_newick(&v);
    let re_v = to_vector(&n);
    println!("v len: {:?}", re_v.len());
}
