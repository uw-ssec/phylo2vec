use rand::Rng;

#[derive(Default)]
pub enum SampleOrdering {
    #[default]
    Ordered,
    NotOrdered
}

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

