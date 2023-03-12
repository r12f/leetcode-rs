//!
//! Problem #199: Binary Tree Right Side View
//!
//! - Link: <https://leetcode.com/problems/binary-tree-right-side-view/>
//! - Discussions: <https://leetcode.com/problems/binary-tree-right-side-view/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given the `root` of a binary tree, imagine yourself standing on the **right side** of it, return *the values of the nodes you can see ordered from top to bottom*.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/02/14/tree.jpg)
//!
//! ```
//! Input: root = [1,2,3,null,5,null,4]
//! Output: [1,3,4]
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: root = [1,null,3]
//! Output: [1,3]
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
//! * The number of nodes in the tree is in the range `[0, 100]`.
//! * `-100 <= Node.val <= 100`
//!

use std::cell::RefCell;
use std::rc::Rc;
#[solution]
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::<i32>::new();

        if let Some(root) = root {
            Self::dfs(&mut result, root, 0);
        }

        result
    }

    fn dfs(result: &mut Vec<i32>, node: Rc<RefCell<TreeNode>>, level: usize) {
        if result.len() <= level {
            result.push(node.borrow().val);
        }

        if let Some(right) = node.borrow().right.clone() {
            Self::dfs(result, right, level + 1);
        }

        if let Some(left) = node.borrow().left.clone() {
            Self::dfs(result, left, level + 1);
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
    fn test_199() {
        assert_eq!(
            vec![1, 3, 4],
            Solution::right_side_view(lc_tree!("[1,2,3,null,5,null,4]"))
        );

        assert_eq!(
            vec![1, 3, 4],
            Solution::right_side_view(lc_tree!("[1,2,3,4]"))
        );
    }
}
