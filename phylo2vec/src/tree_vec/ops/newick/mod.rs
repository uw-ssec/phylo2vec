use thiserror;

use crate::tree_vec::types::Ancestry;

mod newick_patterns;

pub use newick_patterns::NewickPatterns;

#[derive(Debug, thiserror::Error)]
pub enum NewickError {
    // For problematic int parsing in the Newick string
    #[error("ParseIntError: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    // For problematic float parsing in the Newick string
    #[error("ParseFloatError: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    // For problematic stack popping in get_cherries
    #[error("Stack underflow error encountered")]
    StackUnderflow,
}

fn node_substr(s: &str, start: usize) -> (&str, usize) {
    let substr: &str = &s[start..];
    let mut end: usize = start;

    // Find the next comma, closing parenthesis, or semicolon
    for (i, c) in substr.char_indices() {
        if c == ',' || c == ')' || c == ';' {
            end = start + i;
            break;
        }
    }

    let node = &s[start..end];

    (node, end)
}

pub fn get_cherries(newick: &str) -> Result<Ancestry, NewickError> {
    if newick.is_empty() {
        return Ok(Vec::new());
    }
    let mut ancestry: Ancestry = Vec::new();
    let mut stack: Vec<usize> = Vec::new();

    let newick_bytes = newick.as_bytes();

    let mut i: usize = 0;
    while i < newick.len() {
        let c: char = newick_bytes[i] as char;

        if c == ')' {
            i += 1;

            // Pop the children nodes from the stack
            let c2: usize = stack.pop().ok_or(NewickError::StackUnderflow)?;
            let c1: usize = stack.pop().ok_or(NewickError::StackUnderflow)?;

            // Get the parent node after ")"
            let (p, end) = node_substr(newick, i);
            i = end - 1;

            let p_int = p.parse::<usize>().map_err(NewickError::ParseIntError)?;

            // Add the triplet (c1, c2, p)
            ancestry.push([c1, c2, p_int]);

            // Push the parent node to the stack
            stack.push(p_int);
        } else if c.is_ascii_digit() {
            // Get the next node and push it to the stack
            let (node, end) = node_substr(newick, i);
            i = end - 1;

            stack.push(node.parse::<usize>().map_err(NewickError::ParseIntError)?);
        }

        i += 1;
    }

    Ok(ancestry)
}

pub fn get_cherries_no_parents(newick: &str) -> Result<Ancestry, NewickError> {
    if newick.is_empty() {
        return Ok(Vec::new());
    }
    let mut ancestry: Ancestry = Vec::new();
    let mut stack: Vec<usize> = Vec::new();

    let newick_bytes = newick.as_bytes();

    let mut i: usize = 0;
    while i < newick.len() {
        let c: char = newick_bytes[i] as char;

        if c == ')' {
            // Pop the children nodes from the stack
            let c2: usize = stack.pop().ok_or(NewickError::StackUnderflow)?;
            let c1: usize = stack.pop().ok_or(NewickError::StackUnderflow)?;

            let mut c_ordered = [c1, c2];
            c_ordered.sort();

            // No parent annotation --> store the max leaf
            ancestry.push([c1, c2, c_ordered[1]]);

            // Push the min leaf to the stack to represent this internal node going forward
            stack.push(c_ordered[0]);
        } else if c.is_ascii_digit() {
            // Get the next leaf and push it to the stack
            let (leaf, end) = node_substr(newick, i);
            i = end - 1;

            stack.push(leaf.parse::<usize>().map_err(NewickError::ParseIntError)?);
        }

        i += 1;
    }

    Ok(ancestry)
}

pub fn get_cherries_with_bls(newick: &str) -> Result<(Ancestry, Vec<[f32; 2]>), NewickError> {
    if newick.is_empty() {
        return Ok((Vec::new(), Vec::new())); // Return empty ancestry and branch length vectors
    }
    let mut ancestry: Ancestry = Vec::new();
    let mut bls: Vec<[f32; 2]> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();
    let mut bl_stack: Vec<f32> = Vec::new();

    let mut i: usize = 0;

    let newick_bytes = newick.as_bytes();

    while i < newick.len() {
        let c: char = newick_bytes[i] as char;

        if c == ')' {
            i += 1;

            // Pop the children nodes from the stack
            let c2: usize = stack.pop().ok_or(NewickError::StackUnderflow)?;
            let c1: usize = stack.pop().ok_or(NewickError::StackUnderflow)?;

            // Pop the BLs from the BL stack
            let bl2: f32 = bl_stack.pop().ok_or(NewickError::StackUnderflow)?;
            let bl1: f32 = bl_stack.pop().ok_or(NewickError::StackUnderflow)?;
            bls.push([bl1, bl2]);

            let (annotated_p, end) = node_substr(newick, i);
            i = end - 1;

            if end == newick.len() - 1 {
                let p = annotated_p.split(":").next().unwrap();
                let p_int: usize = p.parse::<usize>().map_err(NewickError::ParseIntError)?;
                ancestry.push([c1, c2, p_int]);
            } else {
                // Add the triplet (c1, c2, p)
                let (p, blp) = annotated_p.split_once(':').unwrap();

                let p_int = p.parse::<usize>().map_err(NewickError::ParseIntError)?;

                ancestry.push([c1, c2, p_int]);

                // Push the parent node to the stack
                stack.push(p_int);
                // Push the parent BL to the BL stack
                bl_stack.push(blp.parse::<f32>().map_err(NewickError::ParseFloatError)?);
            }
        } else if c.is_ascii_digit() {
            let (annotated_node, end) = node_substr(newick, i);
            i = end - 1;

            let (node, bln) = annotated_node.split_once(':').unwrap();

            stack.push(node.parse::<usize>().map_err(NewickError::ParseIntError)?);
            bl_stack.push(bln.parse::<f32>().map_err(NewickError::ParseFloatError)?);
        }

        i += 1;
    }

    Ok((ancestry, bls))
}

pub fn get_cherries_no_parents_with_bls(
    newick: &str,
) -> Result<(Ancestry, Vec<[f32; 2]>), NewickError> {
    if newick.is_empty() {
        return Ok((Vec::new(), Vec::new())); // Return empty ancestry and branch length vectors
    }
    let mut ancestry: Ancestry = Vec::new();
    let mut bls: Vec<[f32; 2]> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();
    let mut bl_stack: Vec<f32> = Vec::new();

    let mut i: usize = 0;

    let newick_bytes: &[u8] = newick.as_bytes();

    while i < newick.len() {
        let c: char = newick_bytes[i] as char;

        if c == ')' {
            i += 1;

            // Pop the children nodes from the stack
            let c2: usize = stack.pop().ok_or(NewickError::StackUnderflow)?;
            let c1: usize = stack.pop().ok_or(NewickError::StackUnderflow)?;

            // Pop the BLs from the BL stack
            let bl2: f32 = bl_stack.pop().ok_or(NewickError::StackUnderflow)?;
            let bl1: f32 = bl_stack.pop().ok_or(NewickError::StackUnderflow)?;

            let mut c_ordered = [c1, c2];
            c_ordered.sort();

            // No parent annotation --> store the max leaf
            ancestry.push([c1, c2, c_ordered[1]]);
            bls.push([bl1, bl2]);
            // Find the parental BL
            // Ex: ":0.2"
            let (annotated_node, end) = node_substr(newick, i);
            i = end - 1;

            if annotated_node.is_empty() && end == newick.len() - 1 {
                // if this is true, we reached the root without a BL
                break;
            }

            // Push the min leaf to the stack
            stack.push(c_ordered[0]);
            // Push the parent BL to the BL stack
            bl_stack.push(
                annotated_node[1..]
                    .parse::<f32>()
                    .map_err(NewickError::ParseFloatError)?,
            );
        } else if c.is_ascii_digit() {
            let (annotated_node, end) = node_substr(newick, i);
            i = end - 1;

            let (node, bln) = annotated_node.split_once(':').unwrap();

            stack.push(node.parse::<usize>().map_err(NewickError::ParseIntError)?);
            bl_stack.push(bln.parse::<f32>().map_err(NewickError::ParseFloatError)?);
        }

        i += 1;
    }

    Ok((ancestry, bls))
}

// The recursive function that builds the Newick string
fn _build_newick_recursive_inner(p: usize, ancestry: &Ancestry) -> String {
    let leaf_max = ancestry.len();

    // Extract the children (c1, c2) and ignore the parent from the ancestry tuple
    let [c1, c2, _] = ancestry[p - leaf_max - 1];

    // Recursive calls for left and right children, checking if they are leaves or internal nodes
    let left = if c1 > leaf_max {
        _build_newick_recursive_inner(c1, ancestry)
    } else {
        c1.to_string() // It's a leaf node, just convert to string
    };

    let right = if c2 > leaf_max {
        _build_newick_recursive_inner(c2, ancestry)
    } else {
        c2.to_string() // It's a leaf node, just convert to string
    };

    // Create the Newick string in the form (left, right)p
    format!("({},{}){}", left, right, p)
}

/// Remove parent labels from the Newick string
///
/// # Example
///
/// ```
/// use phylo2vec::tree_vec::ops::newick::remove_parent_labels;
///
/// let newick = "(((0,(3,5)6)8,2)9,(1,4)7)10;";
/// let result = remove_parent_labels(newick);
/// assert_eq!(result, "(((0,(3,5)),2),(1,4));");
/// ```
pub fn remove_parent_labels(newick: &str) -> String {
    let newick_patterns = NewickPatterns::new();
    return newick_patterns.parents.replace_all(newick, ")").to_string();
}

/// Check if the Newick string has parent labels
///
/// # Example
///
/// ```
/// use phylo2vec::tree_vec::ops::newick::has_parents;
///
/// let newick = "(((0,(3,5)6)8,2)9,(1,4)7)10;";
/// let result = has_parents(newick);
/// assert_eq!(result, true);
///
/// let newick_no_parents = "(((0,(3,5)),2),(1,4));";
/// let result_no_parents = has_parents(newick_no_parents);
/// assert_eq!(result_no_parents, false);
/// ```
pub fn has_parents(newick: &str) -> bool {
    let newick_patterns = NewickPatterns::new();
    return newick_patterns.parents.is_match(newick);
}

/// Find the number of leaves in the Newick string
///
/// # Example
///
/// ```
/// use phylo2vec::tree_vec::ops::newick::find_num_leaves;
///
/// let newick = "(((0,(3,5)6)8,2)9,(1,4)7)10;";
/// let result = find_num_leaves(newick);
/// assert_eq!(result, 6);
/// ```
pub fn find_num_leaves(newick: &str) -> usize {
    let newick_patterns = NewickPatterns::new();
    let result: Vec<usize> = newick_patterns
        .pairs
        .captures_iter(newick)
        .map(|caps| {
            let (_, [_, node]) = caps.extract();
            node.parse::<usize>().unwrap()
        })
        .collect();

    return result.len();
}

/// Build newick string from the ancestry matrix
pub fn build_newick(ancestry: &Ancestry) -> String {
    // Get the root node, which is the parent value of the last ancestry element
    let root = ancestry.last().unwrap()[2];

    // Build the Newick string starting from the root, and append a semicolon
    format!("{};", _build_newick_recursive_inner(root, ancestry))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree_vec::ops::to_newick;
    use crate::utils::sample_vector;
    use rstest::*;

    #[rstest]
    #[case("(((0,(3,5)6)8,2)9,(1,4)7)10;", "(((0,(3,5)),2),(1,4));")]
    #[case("(0,(1,(2,(3,(4,5)6)7)8)9)10;", "(0,(1,(2,(3,(4,5)))));")]
    #[case("((0,2)5,(1,3)4)6;", "((0,2),(1,3));")]
    fn test_remove_parent_labels(#[case] newick: &str, #[case] expected: &str) {
        let result = remove_parent_labels(&newick);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(10)]
    #[case(100)]
    #[case(1000)]
    fn test_has_parents(#[case] n_leaves: usize) {
        let v = sample_vector(n_leaves, false);
        let newick = to_newick(&v);
        // Check if the newick string has parents
        let result = has_parents(&newick);
        assert_eq!(result, true);

        // Check if the newick string does not have parents
        let result_no_parents = has_parents(&remove_parent_labels(&newick));
        assert_eq!(result_no_parents, false);
    }

    #[rstest]
    #[case(10)]
    #[case(100)]
    #[case(1000)]
    fn test_find_num_leaves(#[case] n_leaves: usize) {
        let v = sample_vector(n_leaves, false);
        let newick = to_newick(&v);
        // Check if the newick string has parents
        let result = find_num_leaves(&newick);
        assert_eq!(result, n_leaves);
    }

    #[rstest]
    #[case("((1:0.5,2:0.7)1:0.9,3:0.8)2:0.8;", vec![[1, 2, 1], [1, 3, 2]], vec![[0.5, 0.7], [0.9, 0.8]])]
    #[case("(1:0.5,2:0.7);", vec![[1, 2, 2]], vec![[0.5, 0.7]] )]
    fn test_get_cherries_with_bls(
        #[case] newick: &str,
        #[case] expected_ancestry: Vec<[usize; 3]>,
        #[case] expected_bls: Vec<[f32; 2]>,
    ) {
        let ancestry: Ancestry;
        let bls: Vec<[f32; 2]>;
        if has_parents(newick) {
            (ancestry, bls) =
                get_cherries_with_bls(newick).expect("failed to get cherries with branch lengths");
        } else {
            (ancestry, bls) = get_cherries_no_parents_with_bls(newick)
                .expect("failed to get cherries with branch lengths (no parents)");
        }

        // Verify the ancestry
        assert_eq!(ancestry, expected_ancestry); // Ensure ancestry matches the expected

        // Verify the branch lengths
        assert_eq!(bls.len(), expected_bls.len()); // Ensure the number of branch lengths is correct
        assert_eq!(bls, expected_bls); // Ensure branch lengths match the expected
    }
}
