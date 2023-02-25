//!
//! Problem #124: Binary Tree Maximum Path Sum
//!
//! - Link: <https://leetcode.com/problems/binary-tree-maximum-path-sum/>
//! - Discussions: <https://leetcode.com/problems/binary-tree-maximum-path-sum/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! A **path** in a binary tree is a sequence of nodes where each pair of adjacent nodes in the sequence has an edge connecting them. A node can only appear in the sequence **at most once**. Note that the path does not need to pass through the root.
//!
//! The **path sum** of a path is the sum of the node's values in the path.
//!
//! Given the `root` of a binary tree, return *the maximum **path sum** of any **non-empty** path*.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2020/10/13/exx1.jpg)
//!
//! ```
//! Input: root = [1,2,3]
//! Output: 6
//! Explanation: The optimal path is 2 -> 1 -> 3 with a path sum of 2 + 1 + 3 = 6.
//!
//! ```
//!
//! **Example 2:**
//!
//! ![](https://assets.leetcode.com/uploads/2020/10/13/exx2.jpg)
//!
//! ```
//! Input: root = [-10,9,20,null,null,15,7]
//! Output: 42
//! Explanation: The optimal path is 15 -> 20 -> 7 with a path sum of 15 + 20 + 7 = 42.
//!
//! ```
//!
//! **Constraints:**
//!
//! * The number of nodes in the tree is in the range `[1, 3 * 10<sup>4</sup>]`.
//! * `-1000 <= Node.val <= 1000`
//!

use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

use lcrt::TreeNode;
#[solution]
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = i32::MIN;
        Solution::tree_walk(root, &mut result);
        result
    }

    fn tree_walk(node: Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
        if node.is_none() {
            return 0;
        }

        let node = node.unwrap();
        let node = node.borrow();

        let left_max_sum = Solution::tree_walk(node.left.clone(), max_sum);
        let right_max_sum = Solution::tree_walk(node.right.clone(), max_sum);

        let max_single_path_sum = node.val + 0.max(left_max_sum).max(right_max_sum);
        let max_path_sum = max_single_path_sum.max(left_max_sum + right_max_sum + node.val);

        if max_path_sum > *max_sum {
            *max_sum = max_path_sum;
        }

        max_single_path_sum
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
    fn test_124() {
        assert_eq!(
            16,
            Solution::max_path_sum(lc_tree!(
                "[9,6,-3,null,null,-6,2,null,null,2,null,-6,-6,-6]"
            ))
        );

        assert_eq!(
            42,
            Solution::max_path_sum(lc_tree!("[-10,9,20,null,null,15,7]"))
        );
    }
}
