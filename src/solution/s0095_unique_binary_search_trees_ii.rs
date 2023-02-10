//!
//! Problem #95: Unique Binary Search Trees II
//!
//! - Link: <https://leetcode.com/problems/unique-binary-search-trees-ii/>
//! - Discussions: <https://leetcode.com/problems/unique-binary-search-trees-ii/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given an integer `n`, return *all the structurally unique **BST'**s (binary search trees), which has exactly* `n` *nodes of unique values from* `1` *to* `n`. Return the answer in **any order**.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/01/18/uniquebstn3.jpg)
//!
//! ```
//! Input: n = 3
//! Output: [[1,null,2,null,3],[1,null,3,2],[2,1,3],[3,1,null,null,2],[3,2,null,1]]
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: n = 1
//! Output: [[1]]
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= n <= 8`
//!

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

#[solution]
impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if (n == 0) {
            return vec![];
        }

        let mut dp_cur: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
        dp_cur.push(Some(Solution::new_tree_node(1, None, None)));

        for i in 2..=n {
            let mut dp_prev = dp_cur;
            dp_cur = Vec::new();

            for prev_tree in &mut dp_prev {
                Solution::insert_node_to_tree(&mut dp_cur, prev_tree, i);
            }
        }

        return dp_cur;
    }

    fn insert_node_to_tree(
        dp_cur: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        root: &mut Option<Rc<RefCell<TreeNode>>>,
        i: i32,
    ) {
        // Put entire tree as left child node:
        let mut new_node = Some(Solution::new_tree_node(i, root.clone(), None));
        dp_cur.push(Self::clone_tree(&new_node));
        new_node.as_mut().unwrap().borrow_mut().left = None;

        // Put the new node as the right child all the way thru
        let mut node = root.clone();
        while node.is_some() {
            let mut node_inner = node.as_mut().unwrap().borrow_mut();
            new_node.as_mut().unwrap().borrow_mut().left = node_inner.right.clone();
            node_inner.right = new_node.clone();
            drop(node_inner);

            dp_cur.push(Self::clone_tree(root));

            let mut node_inner = node.as_mut().unwrap().borrow_mut();
            let mut new_node_inner = new_node.as_mut().unwrap().borrow_mut();
            node_inner.right = new_node_inner.left.clone();
            new_node_inner.left = None;

            let next_node = node_inner.right.clone();
            drop(node_inner); // Drop node_inner, due to reference to node.
            node = next_node;
        }
    }

    fn clone_tree(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if node.is_none() {
            return None;
        }

        let node_ref = node.as_ref().unwrap().borrow();
        let left_tree = Self::clone_tree(&node_ref.left);
        let right_tree = Self::clone_tree(&node_ref.right);
        Some(Solution::new_tree_node(node_ref.val, left_tree, right_tree))
    }

    fn new_tree_node(
        v: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Rc<RefCell<TreeNode>> {
        let mut node = Rc::new(RefCell::new(TreeNode::new(v)));
        node.borrow_mut().left = left;
        node.borrow_mut().right = right;
        node
    }
}

//
// Tests
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_95() {
        lc_tree_assert_list_eq!(Solution::generate_trees(0), ());
        lc_tree_assert_list_eq!(Solution::generate_trees(1), ("1"));
        lc_tree_assert_list_eq!(Solution::generate_trees(2), ("1,null,2", "2,1"));
        lc_tree_assert_list_eq!(
            Solution::generate_trees(3),
            (
                "1,null,2,null,3",
                "1,null,3,2",
                "2,1,3",
                "3,1,null,null,2",
                "3,2,null,1"
            )
        );
    }
}
