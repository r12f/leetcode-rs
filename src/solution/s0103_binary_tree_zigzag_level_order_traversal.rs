//!
//! Problem #103: Binary Tree Zigzag Level Order Traversal
//!
//! - Link: <https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/>
//! - Discussions: <https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given the `root` of a binary tree, return *the zigzag level order traversal of its nodes' values*. (i.e., from left to right, then right to left for the next level and alternate between).
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/02/19/tree1.jpg)
//!
//! ```
//! Input: root = [3,9,20,null,null,15,7]
//! Output: [[3],[20,9],[15,7]]
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: root = [1]
//! Output: [[1]]
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: root = []
//! Output: []
//!
//! ```
//!
//! **Constraints:**
//!
//! * The number of nodes in the tree is in the range `[0, 2000]`.
//! * `-100 <= Node.val <= 100`
//!

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[solution]
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }

        let root = root.unwrap();
        let mut results: Vec<VecDeque<i32>> = Vec::new();

        let mut queue = VecDeque::new();
        queue.push_back((root, 0));

        while let Some((node, depth)) = queue.pop_front() {
            if depth >= results.len() {
                results.resize(depth + 1, VecDeque::new());
            }

            if depth % 2 == 0 {
                results[depth].push_back(node.borrow().val);
            } else {
                results[depth].push_front(node.borrow().val);
            }

            if let Some(left_child) = node.borrow().left.clone() {
                queue.push_back((left_child, depth + 1));
            }

            if let Some(right_child) = node.borrow().right.clone() {
                queue.push_back((right_child, depth + 1));
            }
        }

        results
            .into_iter()
            .map(|v| v.into_iter().collect::<Vec<i32>>())
            .collect::<Vec<Vec<i32>>>()
    }
}

//
// Tests
//
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_103() {
        assert_eq!(
            lc_vecvec![[3], [20, 9], [15, 7]],
            Solution::zigzag_level_order(lc_tree!("[3,9,20,null,null,15,7]"))
        );
    }
}
