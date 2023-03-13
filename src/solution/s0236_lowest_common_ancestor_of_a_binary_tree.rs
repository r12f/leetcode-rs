//!
//! Problem #236: Lowest Common Ancestor of a Binary Tree
//!
//! - Link: <https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/>
//! - Discussions: <https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given a binary tree, find the lowest common ancestor (LCA) of two given nodes in the tree.
//!
//! According to the [definition of LCA on Wikipedia](https://en.wikipedia.org/wiki/Lowest_common_ancestor): “The lowest common ancestor is defined between two nodes `p` and `q` as the lowest node in `T` that has both `p` and `q` as descendants (where we allow **a node to be a descendant of itself**).”
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2018/12/14/binarytree.png)
//!
//! ```
//! Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 1
//! Output: 3
//! Explanation: The LCA of nodes 5 and 1 is 3.
//!
//! ```
//!
//! **Example 2:**
//!
//! ![](https://assets.leetcode.com/uploads/2018/12/14/binarytree.png)
//!
//! ```
//! Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 4
//! Output: 5
//! Explanation: The LCA of nodes 5 and 4 is 5, since a node can be a descendant of itself according to the LCA definition.
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: root = [1,2], p = 1, q = 2
//! Output: 1
//!
//! ```
//!
//! **Constraints:**
//!
//! * The number of nodes in the tree is in the range `[2, 10<sup>5</sup>]`.
//! * `-10<sup>9</sup> <= Node.val <= 10<sup>9</sup>`
//! * All `Node.val` are **unique**.
//! * `p != q`
//! * `p` and `q` will exist in the tree.
//!

use std::cell::RefCell;
use std::rc::Rc;

#[solution]
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        assert!(root.is_some());

        let (_, _, result) = Self::dfs(root, p, q);
        result
    }

    fn dfs(
        node: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> (bool, bool, Option<Rc<RefCell<TreeNode>>>) {
        if node.is_none() {
            return (false, false, None);
        }

        let left_child_result = Self::dfs(
            node.as_ref().unwrap().borrow().left.clone(),
            p.clone(),
            q.clone(),
        );

        if left_child_result.0 && left_child_result.1 {
            return left_child_result;
        }

        let right_child_result = Self::dfs(
            node.as_ref().unwrap().borrow().right.clone(),
            p.clone(),
            q.clone(),
        );

        if right_child_result.0 && right_child_result.1 {
            return right_child_result;
        }

        let p_found = node == p || left_child_result.0 || right_child_result.0;
        let q_found = node == q || left_child_result.1 || right_child_result.1;
        let node_found = if p_found && q_found { node } else { None };
        (p_found, q_found, node_found)
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
    fn test_236() {
        let tests = vec![
            ("[3,5,1,6,2,0,8,null,null,7,4]", 5, 1, 3),
            ("[3,5,1,6,2,0,8,null,null,7,4]", 5, 4, 5),
            ("[3,5,1,6,2,0,8,null,null,7,4]", 6, 4, 5),
        ];

        for test in tests {
            let tree = TreeNode::from_str(test.0);
            let p = TreeNode::get_child(&tree, test.1);
            let q = TreeNode::get_child(&tree, test.2);

            assert_eq!(
                test.3,
                Solution::lowest_common_ancestor(tree, q, p)
                    .unwrap()
                    .borrow()
                    .val
            );
        }
    }
}
