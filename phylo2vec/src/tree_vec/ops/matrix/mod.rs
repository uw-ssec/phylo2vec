use std::collections::HashMap;
use crate::tree_vec::types::Ancestry;
use crate::tree_vec::ops::newick::{get_cherries_with_bls, get_cherries_no_parents_with_bls};
use crate::tree_vec::ops::vector::{build_vector, order_cherries, order_cherries_no_parents};



// fn _reduce_with_bls(newick: &str) -> (Vec<[usize; 3]>, Vec<[f32; 2]>) {
//     let mut ancestry = Vec::new();
//     let mut bls = Vec::new();

//     // Helper function to perform the recursive reduction
//     fn do_reduce(newick: &str, ancestry: &mut Vec<[usize; 3]>, bls: &mut Vec<[f32; 2]>) -> String {
//         let mut new_newick = String::from(newick);
//         let mut open_idx = 0;
//         for (i, char) in newick.chars().enumerate() {
//             if char == '(' {
//                 open_idx = i + 1;
//             } else if char == ')' {
//                 let (child1, child2) = new_newick[open_idx..i].split_once(',').unwrap();
//                 let parent = new_newick[i + 1..].split_once(',').unwrap().0.split_once(')').unwrap().0;

//                 let (child1_val, bl1) = child1.split_once(':').unwrap();
//                 let (child2_val, bl2) = child2.split_once(':').unwrap();
//                 let parent_val = parent.split(':').next().unwrap();

//                 ancestry.push([
//                     child1_val.parse().unwrap(),
//                     child2_val.parse().unwrap(),
//                     parent_val.parse().unwrap(),
//                 ]);

//                 bls.push([bl1.parse().unwrap(), bl2.parse().unwrap()]);

//                 new_newick = new_newick[0..open_idx - 1].to_string() + &new_newick[i + 1..];

//                 return do_reduce(&new_newick, ancestry, bls);
//             }
//         }
//         new_newick  // Return the modified newick string
//     }

//     // Initialize recursion and process the Newick string
//     do_reduce(&newick.trim_end_matches(';'), &mut ancestry, &mut bls);

//     (ancestry, bls)
// }

pub fn to_matrix(newick: &str) -> Vec<Vec<f32>> {
    // Get the ancestry and branch lengths
    let (mut ancestry, bls) = get_cherries_with_bls(newick);
    let indices = _get_sorted_indices(&ancestry);

    order_cherries(&mut ancestry);  // Order the cherries in the ancestry matrix based on parent values
    let vector = build_vector(&ancestry);  // Build the ordered  vector

    let reordered_bls: Vec<[f32; 2]> = indices.iter()
        .map(|&idx| bls[idx])  // Access each element of `bls` using the index from `indices`
        .collect();

    // Combine the vector with the branch lengths into a matrix
    let mut matrix: Vec<Vec<f32>> = Vec::new();
    
    for i in 0..vector.len() {
        let row = vec![vector[i] as f32, reordered_bls[i][0], reordered_bls[i][1]]; 
        matrix.push(row); 
    }

    matrix // Return the matrix
}

// Matrix construction for the "no parents" case
// pub fn to_matrix_no_parents(newick: &str) -> Vec<Vec<f32>> {
//     let ancestry = get_cherries_no_parents(newick);  // Using the `get_cherries_no_parents` function directly
//     let (cherries, idxs) = order_cherries_no_parents(&mut ancestry.clone());

//     // Extract branch lengths from the Newick string or ensure they are included in get_cherries
//     let selected_bls: Vec<Vec<f32>> = idxs.iter().map(|&i| vec![0.0]).collect(); // Example BLs, replace with actual data
    
//     let v = build_vector(&cherries);  

//     let mut m = Vec::new();
//     for (v_elem, bl_row) in v.iter().zip(selected_bls.iter()) {
//         let mut row = vec![*v_elem as f32];  // Convert the element into f32 and push
//         row.push(bl_row[0]);
//         m.push(row);
//     }

//     m
// }

// Helper function that takes an ancestry array, and returns an array of indices, 
//sorted by the parent values in the ancestry array. 
fn _get_sorted_indices(ancestry: &Ancestry) -> Vec<usize> {
    // let mut indices = (0..ancestry.len()).collect::<Vec<usize>>();
    // indices.sort_by(|&a, &b| ancestry[a][2].cmp(&ancestry[b][2]));
    let num_cherries = ancestry.len();

    // Create a vector of indices from 0 to num_cherries - 1
    let mut indices: Vec<usize> = (0..num_cherries).collect();

    // Sort the indices based on the parent value (ancestry[i][2])
    indices.sort_by_key(|&i| ancestry[i][2]);
    indices
}


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // Test for the `to_matrix` function
    // Verifies correct matrix generation from a Newick string.
    #[rstest]
    #[case("((1:0.5,2:0.6):0.7,(3:0.8,4:0.9):1.0);", vec![
        vec![1.0, 2.0, 0.5],
        vec![3.0, 4.0, 0.8],
        vec![2.0, 3.0, 0.7],
    ])]
    fn test_to_matrix(#[case] newick: String, #[case] expected_matrix: Vec<Vec<f32>>) {
        let matrix = to_matrix(&newick);
        
        // Check if the matrix matches the expected matrix
        assert_eq!(matrix, expected_matrix);
    }

    // Test for the `to_matrix_no_parents` function
    // Verifies correct matrix generation from a Newick string without parent nodes.
    #[rstest]
    #[case("((1:0.5,2:0.6):0.7,(3:0.8,4:0.9):1.0);", vec![
        vec![1.0, 2.0, 0.5],
        vec![3.0, 4.0, 0.8],
        vec![2.0, 3.0, 0.7],
    ])]
    // fn test_to_matrix_no_parents(#[case] newick_no_parents: String, #[case] expected_matrix: Vec<Vec<f32>>) {
    //     let matrix = to_matrix_no_parents(newick_no_parents);
        
    //     // Check if the matrix matches the expected matrix
    //     assert_eq!(matrix, expected_matrix);
    // }

    // Test for an empty Newick string in the `to_matrix` function
    // Ensures that an empty Newick string results in an empty matrix.
    #[rstest]
    #[case("".to_string(), vec![])]
    fn test_empty_newick_to_matrix(#[case] newick: String, #[case] expected_matrix: Vec<Vec<f32>>) {
        let matrix = to_matrix(&newick);
        
        // Empty Newick should result in an empty matrix
        assert_eq!(matrix, expected_matrix);
    }

    // // Test for an empty Newick string in the `to_matrix_no_parents` function
    // // Ensures that an empty Newick string results in an empty matrix when no parents are considered.
    // #[rstest]
    // #[case("".to_string(), vec![])]
    // fn test_empty_newick_to_matrix_no_parents(#[case] newick_no_parents: String, #[case] expected_matrix: Vec<Vec<f32>>) {
    //     let matrix = to_matrix_no_parents(newick_no_parents);
        
    //     // Empty Newick should result in an empty matrix
    //     assert_eq!(matrix, expected_matrix);
    // }
}


