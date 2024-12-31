use crate::tree_vec::types::Pair;

pub struct Node {
    value: Pair,
    height: usize,
    size: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: Pair) -> Self {
        Node {
            value,
            height: 1,
            size: 1,
            left: None,
            right: None,
        }
    }
}

pub struct AVLTree {
    root: Option<Box<Node>>,
}

impl AVLTree {
    pub fn new() -> Self {
        AVLTree { root: None }
    }

    fn get_height(node: &Option<Box<Node>>) -> usize {
        match node {
            Some(ref n) => n.height,
            None => 0,
        }
    }

    fn get_size(node: &Option<Box<Node>>) -> usize {
        match node {
            Some(ref n) => n.size,
            None => 0,
        }
    }

    fn update(n: &mut Box<Node>) {
        n.height = 1 + usize::max(Self::get_height(&n.left), Self::get_height(&n.right));
        n.size = 1 + Self::get_size(&n.left) + Self::get_size(&n.right);
    }

    fn right_rotate(y: &mut Option<Box<Node>>) -> Option<Box<Node>> {
        if let Some(mut y_node) = y.take() {
            if let Some(mut x) = y_node.left.take() {
                // Perform rotation
                let t2 = x.right.take();
                x.right = Some(y_node);
                x.right.as_mut().unwrap().left = t2;

                // Update heights
                Self::update(x.right.as_mut().unwrap());
                Self::update(&mut x);

                return Some(x);
            } else {
                // If no left child, revert the state and return `None`
                *y = Some(y_node);
                None
            }
        } else {
            None
        }
    }

    fn left_rotate(x: &mut Option<Box<Node>>) -> Option<Box<Node>> {
        if let Some(mut x_node) = x.take() {
            if let Some(mut y) = x_node.right.take() {
                // Perform rotation
                let t2 = y.left.take();
                y.left = Some(x_node);
                y.left.as_mut().unwrap().right = t2;

                // Update heights
                Self::update(y.left.as_mut().unwrap());
                Self::update(&mut y);

                return Some(y);
            } else {
                // If no right child, revert the state and return `None`
                *x = Some(x_node);
                None
            }
        } else {
            None
        }
    }

    fn get_balance(node: &Option<Box<Node>>) -> isize {
        match node {
            Some(ref n) => Self::get_height(&n.left) as isize - Self::get_height(&n.right) as isize,
            None => 0,
        }
    }

    fn balance(node: &mut Option<Box<Node>>) -> Option<Box<Node>> {
        let balance = Self::get_balance(node);
        if balance > 1 {
            if Self::get_balance(&node.as_ref().unwrap().left) >= 0 {
                return Self::right_rotate(node);
            } else {
                if let Some(ref mut n) = node {
                    n.left = Self::left_rotate(&mut n.left);
                }
                return Self::right_rotate(node);
            }
        }
        if balance < -1 {
            if Self::get_balance(&node.as_ref().unwrap().right) <= 0 {
                return Self::left_rotate(node);
            } else {
                if let Some(ref mut n) = node {
                    n.right = Self::right_rotate(&mut n.right);
                }
                return Self::left_rotate(node);
            }
        }
        node.take()
    }

    pub fn insert(&mut self, index: usize, value: Pair) {
        self.root = Self::insert_by_index(self.root.take(), value, index);
    }

    fn insert_by_index(node: Option<Box<Node>>, value: Pair, index: usize) -> Option<Box<Node>> {
        let mut n: Box<Node> = match node {
            Some(n) => n,
            None => return Some(Box::new(Node::new(value))),
        };

        let left_size = Self::get_size(&n.left);
        if index <= left_size {
            n.left = Self::insert_by_index(n.left.take(), value, index);
        } else {
            n.right = Self::insert_by_index(n.right.take(), value, index - left_size - 1);
        }

        Self::update(&mut n);
        return Self::balance(&mut Some(n));
    }

    pub fn lookup(&self, index: usize) -> Pair {
        Self::lookup_node(&self.root, index).unwrap_or((0, 0))
    }

    fn lookup_node(node: &Option<Box<Node>>, index: usize) -> Option<Pair> {
        match node {
            Some(ref n) => {
                let left_size = Self::get_size(&n.left);
                if index < left_size {
                    Self::lookup_node(&n.left, index)
                } else if index == left_size {
                    Some(n.value)
                } else {
                    Self::lookup_node(&n.right, index - left_size - 1)
                }
            }
            None => None,
        }
    }

    pub fn inorder_traversal(&self) -> Vec<Pair> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut current = &self.root;

        while current.is_some() || !stack.is_empty() {
            while let Some(ref n) = current {
                stack.push(n);
                current = &n.left;
            }

            let node = stack.pop().unwrap();
            result.push(node.value);

            current = &node.right;
        }

        result
    }

    pub fn get_pairs(&self) -> Vec<Pair> {
        self.inorder_traversal()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn sample_tree() -> AVLTree {
        let mut tree = AVLTree::new();
        tree.insert(0, (1, 1));
        tree.insert(1, (2, 2));
        tree.insert(2, (3, 3));
        tree
    }

    // #[rstest]
    // #[case(Some(Box::new(Node { value: (2, 2), height: 1, size: 1, left: None, right: None })), 1)]
    // #[case(Some(Box::new(Node { value: (2, 2), height: 2, size: 1, left: None, right: None })), 2)]
    // #[case(None, 0)]
    // #[case(sample_tree().root, 2)]
    // fn test_get_height(#[case] node: Option<Box<Node>>, #[case] expected: usize) {
    //     assert_eq!(AVLTree::get_height(&node), expected);
    // }


    #[rstest]
    #[case(0, (1, 1))]
    #[case(1, (2, 2))]
    #[case(2, (3, 3))]
    fn test_lookup(#[case] lookup_index: usize, #[case] expected: Pair) {
        let tree = sample_tree();
        assert_eq!(tree.lookup(lookup_index), expected);
    }
    
    #[rstest]
    #[case(vec![(0, (1, 1))], 0, (1, 1))]
    #[case(vec![(0, (1, 1)), (1, (2, 2))], 1, (2, 2))]
    #[case(vec![(0, (1, 1)), (0, (2, 2)), (0, (3, 3))], 0, (3, 3))]
    #[case(vec![(0, (1, 1)), (0, (2, 2)), (0, (3, 3))], 2, (1, 1))]
    fn test_insert(#[case] inserts: Vec<(usize, Pair)>, #[case] lookup_index: usize, #[case] expected: Pair) {
        let mut tree = AVLTree::new();
        for (index, value) in inserts {
            tree.insert(index, value);
        }
        assert_eq!(tree.lookup(lookup_index), expected); //any way to not use lookup here?
    }



    #[rstest]
    #[case(vec![(0, (1, 1)), (1, (2, 2)), (2, (3, 3))], vec![(1, 1), (2, 2), (3, 3)])]
    #[case(vec![(0, (3, 3)), (0, (2, 2)), (0, (1, 1))], vec![(1, 1), (2, 2), (3, 3)])]
    #[case(vec![(0, (2, 2)), (1, (1, 1)), (0, (3, 3))], vec![(1, 1), (2, 2), (3, 3)])]
    fn test_inorder_traversal(#[case] inserts: Vec<(usize, Pair)>, #[case] expected: Vec<Pair>) {
        let mut tree = AVLTree::new();
        for (index, value) in inserts {
            tree.insert(index, value);
        }
        assert_eq!(tree.inorder_traversal(), expected);
    }

    #[rstest]
    #[case (vec![(1, 1), (2, 2), (3, 3)])]
   // #[case(vec![(0, (3, 3)), (0, (2, 2)), (0, (1, 1))], vec![(1, 1), (2, 2), (3, 3)])]
   // #[case(vec![(0, (2, 2)), (1, (1, 1)), (0, (3, 3))], vec![(1, 1), (2, 2), (3, 3)])]
    fn test_get_pairs( #[case] expected: Vec<Pair>) {
        // let mut tree = AVLTree::new();
        // for (index, value) in inserts {
        //     tree.insert(index, value);
        // }
        assert_eq!(sample_tree().get_pairs(), expected);
    }

    // #[rstest]
    // #[case(vec![0, 1, 2, 3, 4, 5])]
    // #[case(vec![5, 4, 3, 2, 1, 0])]
    // #[case(vec![3, 1, 4, 0, 2, 5])]
    // fn test_balance_after_insert(#[case] insert_order: Vec<usize>) {
    //     let mut tree = AVLTree::new();
    //     for (i, &index in insert_order.iter().enumerate() {
    //         tree.insert(index, (i as i16, i as i16));
    //     }
    //     // After balancing, the height should be significantly less than the number of nodes
    //     assert!(AVLTree::get_height(&tree.root) <= 4);
    // }

    #[rstest]
    #[case(3, (0, 0))]
    #[case(10, (0, 0))]
    #[case(usize::MAX, (0, 0))]
    fn test_lookup_out_of_bounds(sample_tree: AVLTree, #[case] index: usize, #[case] expected: Pair) {
        assert_eq!(sample_tree.lookup(index), expected);
    }

    #[rstest]
    #[case(Some(Box::new(Node {
        value: (2, 2),
        height: 1,
        size: 1,
        left: None,
        right: None,
    })), 1)]
    #[case(sample_tree().root, 2)]
    #[case(None, 0)]
    fn test_insert2(#[case] mut node: Option<Box<Node>>, #[case] expected: usize) {
        assert_eq!(AVLTree::get_height(&node), expected);

        //TODO: This test exposes:
        //  - the fact that all new nodes are created with height and size values hard-coded to 1. Is this the desired behavior, regarding inserts? 
        //  - the fact that the height and size values are not mock-able/isolable - they require a real AVLTree instance to be tested. 
        //    Basically, this test will only test the fact that update_height does not change increment the height of the node. Is this the desired behavior?
    }

    // test update size

    // test insert


}
