#[derive(Debug)]
pub struct NewickPatterns {
    pub left_node: regex::Regex,
    pub right_node: regex::Regex,
    pub pairs: regex::Regex,
    pub branch_lengths: regex::Regex,
    pub parents: regex::Regex,
}

impl NewickPatterns {
    pub fn new() -> Self {
        let _left_node = r"\(\b(\d+)\b";
        let _right_node = r",\b(\d+)\b";
        let _branch_lengths = r":\d+(\.\d+)?";
        let _parents = r"\)(\d+)";
        let _pairs = format!(r"({})|({})", _left_node, _right_node);
        NewickPatterns {
            // Pattern of an integer label on the left of a pair
            left_node: regex::Regex::new(&_left_node).unwrap(),
            // Pattern of an integer label on the right of a pair
            right_node: regex::Regex::new(&_right_node).unwrap(),
            // Pattern of a pair of integer labels
            pairs: regex::Regex::new(&_pairs).unwrap(),
            // Pattern of a branch length annotation
            branch_lengths: regex::Regex::new(&_branch_lengths).unwrap(),
            // Pattern of a parent label
            parents: regex::Regex::new(&_parents).unwrap(),
        }
    }
}
