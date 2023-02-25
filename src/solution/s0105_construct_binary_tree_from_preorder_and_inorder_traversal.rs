//!
//! Problem #105: Construct Binary Tree from Preorder and Inorder Traversal
//!
//! - Link: <https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/>
//! - Discussions: <https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given two integer arrays `preorder` and `inorder` where `preorder` is the preorder traversal of a binary tree and `inorder` is the inorder traversal of the same tree, construct and return *the binary tree*.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/02/19/tree.jpg)
//!
//! ```
//! Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
//! Output: [3,9,20,null,null,15,7]
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: preorder = [-1], inorder = [-1]
//! Output: [-1]
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= preorder.length <= 3000`
//! * `inorder.length == preorder.length`
//! * `-3000 <= preorder[i], inorder[i] <= 3000`
//! * `preorder` and `inorder` consist of **unique** values.
//! * Each value of `inorder` also appears in `preorder`.
//! * `preorder` is **guaranteed** to be the preorder traversal of the tree.
//! * `inorder` is **guaranteed** to be the inorder traversal of the tree.
//!

use std::cell::RefCell;
use std::rc::Rc;

use lcrt::TreeNode;

fn new_tree_node(val: i32) -> Rc<RefCell<TreeNode>> {
    Rc::new(RefCell::new(TreeNode::new(val)))
}

#[solution]
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        assert!(!preorder.is_empty());

        let root = new_tree_node(preorder[0]);

        let root_inorder_index = inorder
            .iter()
            .position(|&v| v == root.borrow().val)
            .unwrap();

        Solution::build_tree_recursive(
            root.clone(),
            &preorder[1..],
            &inorder[..root_inorder_index],
            &inorder[(root_inorder_index + 1)..],
        );

        Some(root)
    }

    fn build_tree_recursive(
        parent: Rc<RefCell<TreeNode>>,
        child_preorder: &[i32],
        left_inorder: &[i32],
        right_inorder: &[i32],
    ) {
        assert!(child_preorder.len() == left_inorder.len() + right_inorder.len());

        if child_preorder.is_empty() {
            return;
        }

        let mut child_index = 0;

        // Found in left.
        if let Some(left_index) = left_inorder
            .iter()
            .position(|&v| v == child_preorder[child_index])
        {
            let left_node = new_tree_node(child_preorder[child_index]);

            Solution::build_tree_recursive(
                left_node.clone(),
                &child_preorder[1..left_inorder.len()],
                &left_inorder[..left_index],
                &left_inorder[(left_index + 1)..],
            );

            child_index += left_inorder.len();
            parent.borrow_mut().left = Some(left_node);
        }

        // Found in right
        if let Some(right_index) = right_inorder
            .iter()
            .position(|&v| v == child_preorder[child_index])
        {
            let right_node = new_tree_node(child_preorder[child_index]);

            Solution::build_tree_recursive(
                right_node.clone(),
                &child_preorder[child_index + 1..(child_index + right_inorder.len())],
                &right_inorder[..right_index],
                &right_inorder[(right_index + 1)..],
            );

            parent.borrow_mut().right = Some(right_node);
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
    fn test_105() {
        lc_tree_assert_eq!(Solution::build_tree(vec![0], vec![0]), "[0]");
        lc_tree_assert_eq!(Solution::build_tree(vec![1, 2], vec![2, 1]), "[1, 2]");
        lc_tree_assert_eq!(Solution::build_tree(vec![1, 2], vec![1, 2]), "[1,null,2]");

        lc_tree_assert_eq!(
            Solution::build_tree(vec![3, 1, 2, 4], vec![1, 2, 3, 4]),
            "[3,1,4,null,2]"
        );

        lc_tree_assert_eq!(
            Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
            "[3,9,20,null,null,15,7]"
        );
    }
}
