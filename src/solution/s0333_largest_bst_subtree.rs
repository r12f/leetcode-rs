//!
//! Problem #333: Largest BST Subtree
//!
//! - Link: <https://leetcode.com/problems/largest-bst-subtree/>
//! - Discussions: <https://leetcode.com/problems/largest-bst-subtree/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given the root of a binary tree, find the largest subtree, which is also a Binary Search Tree (BST), where the largest means subtree has the largest number of nodes.
//!
//! A **Binary Search Tree (BST)** is a tree in which all the nodes follow the below-mentioned properties:
//!
//! * The left subtree values are less than the value of their parent (root) node's value.
//! * The right subtree values are greater than the value of their parent (root) node's value.
//!
//! **Note:** A subtree must include all of its descendants.
//!
//! **Example 1:**
//!
//! **![](https://assets.leetcode.com/uploads/2020/10/17/tmp.jpg)**
//!
//! ```
//! Input: root = [10,5,15,1,8,null,7]
//! Output: 3
//! Explanation: The Largest BST Subtree in this case is the highlighted one. The return value is the subtree's size, which is 3.
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: root = [4,2,7,2,3,5,null,2,null,null,null,null,null,1]
//! Output: 2
//!
//! ```
//!
//! **Constraints:**
//!
//! * The number of nodes in the tree is in the range `[0, 10<sup>4</sup>]`.
//! * `-10<sup>4</sup> <= Node.val <= 10<sup>4</sup>`
//!
//! **Follow up:** Can you figure out ways to solve it with `O(n)` time complexity?
//!

use std::cell::RefCell;
use std::rc::Rc;

#[solution]
impl Solution {
    pub fn largest_bst_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (_, _, _, max_node_count) = Self::dfs(root);
        max_node_count
    }

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32, i32) {
        match node {
            None => (i32::MAX, i32::MIN, 0, 0),
            Some(node) => {
                let val = node.borrow().val;
                let left_result = Self::dfs(node.borrow().left.clone());
                let right_result = Self::dfs(node.borrow().right.clone());

                if left_result.1 < val && right_result.0 > val {
                    let min_val = if left_result.3 != 0 {
                        left_result.0
                    } else {
                        val
                    };

                    let max_val = if right_result.3 != 0 {
                        right_result.1
                    } else {
                        val
                    };

                    let node_count = left_result.2 + right_result.2 + 1;
                    let max_node_count = node_count.max(left_result.3).max(right_result.3);
                    (min_val, max_val, node_count, max_node_count)
                } else {
                    let max_node_count = 0.max(left_result.3).max(right_result.3);
                    // (val, val, 1, max_node_count)

                    // A subtree must include all of its descendants, a partial subtree doesn't count.
                    // So here, we are making a case where the parent will always fail.
                    (i32::MIN, i32::MAX, 0, max_node_count)
                }
            }
        }
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
    fn test_333() {
        assert_eq!(
            3,
            Solution::largest_bst_subtree(lc_tree!("[10,5,15,1,8,null,7]"))
        );

        assert_eq!(
            2,
            Solution::largest_bst_subtree(lc_tree!(
                "[4,2,7,2,3,5,null,2,null,null,null,null,null,1]"
            ))
        );
    }
}
