use phylo2vec::tree_vec::ops;
use phylo2vec::utils;
use std::env;

const DEFAULT_N_LEAVES: usize = 100000;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n_leaves = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(DEFAULT_N_LEAVES)
    } else {
        DEFAULT_N_LEAVES
    };
    let v = utils::sample_vector(n_leaves, true);
    let n = ops::to_newick(&v);
    let re_v = ops::to_vector(&n);
    println!("vector length: {:?}", re_v.len());
}
