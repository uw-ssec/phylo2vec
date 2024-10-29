use rand::Rng;

pub enum SampleOrdering {
    Ordered,
    NotOrdered
}

impl Default for SampleOrdering {
    fn default() -> Self {
        SampleOrdering::NotOrdered
    }
}

/// Sample a vector with `n_leaves - 1` elements.
/// 
/// If ordering is True, sample an ordered tree, by default ordering is False
/// ordering=True: v_i in {0, 1, ..., i} for i in (0, n_leaves-1)
/// ordering=False: v_i in {0, 1, ..., 2*i} for i in (0, n_leaves-1)
/// 
/// # Examples
/// 
/// ```
/// use phylo2vec::utils::sample;
/// use phylo2vec::utils::SampleOrdering;
/// let v = sample(10, SampleOrdering::NotOrdered);
/// let v2 = sample(5, SampleOrdering::Ordered);
/// ```
pub fn sample(n_leaves: usize, ordering: SampleOrdering) -> Vec<usize> {
    let mut v: Vec<usize> = Vec::with_capacity(n_leaves - 1);
    let mut rng = rand::thread_rng();

    match ordering {
        SampleOrdering::Ordered => {
            for i in 0..(n_leaves - 1) {
                v.push(rng.gen_range(0..(i + 1)));
            }
        }
        SampleOrdering::NotOrdered => {
            for i in 0..(n_leaves - 1) {
                v.push(rng.gen_range(0..(2 * i + 1)));
            }
        }
    }

    v
}

/// Input validation of a Phylo2Vec vector
/// 
/// The input is checked to satisfy the Phylo2Vec constraints
/// 
/// # Panics
/// 
/// Panics if any element of the input vector is out of bounds
/// 
/// # Examples
/// 
/// ```
/// use phylo2vec::utils::check_v;
/// check_v(&vec![0, 0, 1]);
/// ```
pub fn check_v(v: &Vec<usize>) {
    let k = v.len();
    let v_max: Vec<usize> = (0..k).map(|i| i * 2).collect();

    for i in 0..k {
        assert!(v[i] <= v_max[i], "Validation failed: v[{}] = {} is out of bounds", i, v[i]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let v = sample(20, SampleOrdering::Ordered);
        assert_eq!(v.len(), 19);
        for i in 0..19 {
            assert!(v[i] <= i);
        };
        check_v(&v);

        let v = sample(20, SampleOrdering::NotOrdered);
        assert_eq!(v.len(), 19);
        check_v(&v);
        for i in 0..19 {
            assert!(v[i] <= 2 * i);
        }
    }

    #[test]
    fn test_check_v() {
        check_v(&vec![0, 0, 2, 1, 0]);
    }

    #[test]
    #[should_panic]
    fn test_check_v_should_panic() {
        check_v(&vec![0, 0, 9, 1]);
    }
}
