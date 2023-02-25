//!
//! Problem #662: Maximum Width of Binary Tree
//!
//! - Link: <https://leetcode.com/problems/maximum-width-of-binary-tree/>
//! - Discussions: <https://leetcode.com/problems/maximum-width-of-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given the `root` of a binary tree, return *the **maximum width** of the given tree*.
//!
//! The **maximum width** of a tree is the maximum **width** among all levels.
//!
//! The **width** of one level is defined as the length between the end-nodes (the leftmost and rightmost non-null nodes), where the null nodes between the end-nodes that would be present in a complete binary tree extending down to that level are also counted into the length calculation.
//!
//! It is **guaranteed** that the answer will in the range of a **32-bit** signed integer.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/05/03/width1-tree.jpg)
//!
//! ```
//! Input: root = [1,3,2,5,3,null,9]
//! Output: 4
//! Explanation: The maximum width exists in the third level with length 4 (5,3,null,9).
//!
//! ```
//!
//! **Example 2:**
//!
//! ![](https://assets.leetcode.com/uploads/2022/03/14/maximum-width-of-binary-tree-v3.jpg)
//!
//! ```
//! Input: root = [1,3,2,5,null,null,9,6,null,7]
//! Output: 7
//! Explanation: The maximum width exists in the fourth level with length 7 (6,null,null,null,null,null,7).
//!
//! ```
//!
//! **Example 3:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/05/03/width3-tree.jpg)
//!
//! ```
//! Input: root = [1,3,2,5]
//! Output: 2
//! Explanation: The maximum width exists in the second level with length 2 (3,2).
//!
//! ```
//!
//! **Constraints:**
//!
//! * The number of nodes in the tree is in the range `[1, 3000]`.
//! * `-100 <= Node.val <= 100`
//!

use std::cell::RefCell;
use std::rc::Rc;

use lcrt::TreeNode;
#[solution]
impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let root = root.unwrap();

        let mut left_offsets: Vec<i32> = Vec::new();
        Solution::walk_left_most_nodes(root.clone(), &mut left_offsets, 0, 0);

        let mut max_width = 0;
        Solution::calculate_width(root.clone(), &mut left_offsets, &mut max_width, 0, 0);

        max_width
    }

    fn walk_left_most_nodes(
        node: Rc<RefCell<TreeNode>>,
        left_offsets: &mut Vec<i32>,
        depth: i32,
        node_offset: i32,
    ) {
        // First node arrives here will have the smallest offset.
        if depth == left_offsets.len() as i32 {
            left_offsets.push(node_offset);
        }

        // We cannot return here, because leftmost nodes in child tree might exist in right child tree.
        if let Some(left_child) = &node.borrow().left {
            Solution::walk_left_most_nodes(
                left_child.clone(),
                left_offsets,
                depth + 1,
                node_offset * 2,
            );
        }

        if let Some(right_child) = &node.borrow().right {
            Solution::walk_left_most_nodes(
                right_child.clone(),
                left_offsets,
                depth + 1,
                node_offset * 2 + 1,
            );
        }
    }

    fn calculate_width(
        node: Rc<RefCell<TreeNode>>,
        left_offsets: &mut Vec<i32>,
        max_width: &mut i32,
        depth: i32,
        node_offset: i32,
    ) {
        let width = node_offset - left_offsets[depth as usize] + 1;
        if width > *max_width {
            *max_width = width;
        }

        if let Some(right_child) = &node.borrow().right {
            Solution::calculate_width(
                right_child.clone(),
                left_offsets,
                max_width,
                depth + 1,
                node_offset * 2 + 1,
            );
        }

        if let Some(left_child) = &node.borrow().left {
            Solution::calculate_width(
                left_child.clone(),
                left_offsets,
                max_width,
                depth + 1,
                node_offset * 2,
            );
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
    fn test_662() {
        assert_eq!(1, Solution::width_of_binary_tree(lc_tree!("[0]")));
        assert_eq!(1, Solution::width_of_binary_tree(lc_tree!("[0, 1]")));
        assert_eq!(
            1,
            Solution::width_of_binary_tree(lc_tree!("[0, 1, null, 2]"))
        );

        assert_eq!(
            1,
            Solution::width_of_binary_tree(lc_tree!("[0, null, 1, null, 2, null, 3]"))
        );

        assert_eq!(
            4,
            Solution::width_of_binary_tree(lc_tree!("[1,3,2,5,3,null,9]"))
        );

        assert_eq!(
            7,
            Solution::width_of_binary_tree(lc_tree!("[1,3,2,5,null,null,9,6,null,7]"))
        );
    }
}
