/// A type alias for the Pair type, which is a tuple representing (child1, child2)
pub type Pair = (usize, usize);

/// A type alias for the Pairs type, which is a vector of pairs representing [child1, child2]
pub type Pairs = Vec<Pair>;

/// A type alias for the Ancestry type, which is a vector of vectors representing [child1, child2, parent]
pub type Ancestry = Vec<[usize; 3]>;
