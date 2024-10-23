use rand::Rng;

pub fn sample_ordered(n_leaves: usize) -> Vec<usize> {
    let mut v: Vec<usize> = Vec::with_capacity(n_leaves);
    let mut rng = rand::thread_rng();

    for i in 0..n_leaves {
        v.push(rng.gen_range(0..=i));
    }
    v
}

pub fn sample_unordered(n_leaves: usize) -> Vec<usize> {
    let mut v: Vec<usize> = Vec::with_capacity(n_leaves);
    let mut rng = rand::thread_rng();

    for i in 0..n_leaves {
        v.push(rng.gen_range(0..=2 * i));
    }
    v
}
