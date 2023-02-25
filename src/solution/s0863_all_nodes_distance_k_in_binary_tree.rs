//!
//! Problem #863: All Nodes Distance K in Binary Tree
//!
//! - Link: <https://leetcode.com/problems/all-nodes-distance-k-in-binary-tree/>
//! - Discussions: <https://leetcode.com/problems/all-nodes-distance-k-in-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given the `root` of a binary tree, the value of a target node `target`, and an integer `k`, return *an array of the values of all nodes that have a distance* `k` *from the target node.*
//!
//! You can return the answer in **any order**.
//!
//! **Example 1:**
//!
//! ![](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/06/28/sketch0.png)
//!
//! ```
//! Input: root = [3,5,1,6,2,0,8,null,null,7,4], target = 5, k = 2
//! Output: [7,4,1]
//! Explanation: The nodes that are a distance 2 from the target node (with value 5) have values 7, 4, and 1.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: root = [1], target = 1, k = 3
//! Output: []
//!
//! ```
//!
//! **Constraints:**
//!
//! * The number of nodes in the tree is in the range `[1, 500]`.
//! * `0 <= Node.val <= 500`
//! * All the values `Node.val` are **unique**.
//! * `target` is the value of one of the nodes in the tree.
//! * `0 <= k <= 1000`
//!

use std::cell::RefCell;
use std::rc::Rc;

use lcrt::TreeNode;
#[solution]
impl Solution {
    pub fn distance_k(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
    ) -> Vec<i32> {
        // These will never be None.
        let root = root.unwrap();
        let target = target.unwrap();

        let mut target_path: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        Solution::get_target_path(root.clone(), &mut target_path, target.borrow().val);

        let mut results: Vec<i32> = Vec::new();
        for i in 0..target_path.len() {
            let initial_distance = i as i32;

            let mut banned_child = None;
            if i > 0 {
                banned_child = Some(target_path[i - 1].borrow().val);
            }

            Solution::get_path_with_distance(
                &mut results,
                target_path[i].clone(),
                banned_child,
                k - initial_distance,
            );
        }

        results
    }

    fn get_target_path(
        node: Rc<RefCell<TreeNode>>,
        target_path: &mut Vec<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> bool {
        if node.borrow().val == target {
            target_path.push(node);
            return true;
        }

        if let Some(left_child) = node.borrow().left.clone() {
            if Solution::get_target_path(left_child, target_path, target) {
                target_path.push(node.clone());
                return true;
            }
        }

        if let Some(right_child) = node.borrow().right.clone() {
            if Solution::get_target_path(right_child, target_path, target) {
                target_path.push(node.clone());
                return true;
            }
        }

        return false;
    }

    fn get_path_with_distance(
        results: &mut Vec<i32>,
        node: Rc<RefCell<TreeNode>>,
        banned_child: Option<i32>,
        distance: i32,
    ) {
        if distance < 0 {
            return;
        }

        if distance == 0 {
            results.push(node.borrow().val);
            return;
        }

        if let Some(left_child) = node.borrow().left.clone() {
            // The node on the path to the target will be covered when we handle the child nodes, so we ignore them here.
            if banned_child.is_none() || banned_child.unwrap() != left_child.borrow().val {
                Solution::get_path_with_distance(results, left_child, None, distance - 1)
            }
        }

        if let Some(right_child) = node.borrow().right.clone() {
            // The node on the path to the target will be covered when we handle the child nodes, so we ignore them here.
            if banned_child.is_none() || banned_child.unwrap() != right_child.borrow().val {
                Solution::get_path_with_distance(results, right_child, None, distance - 1)
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
    fn test_863() {
        assert_eq!(
            Vec::<i32>::new(),
            Solution::distance_k(lc_tree!("[1]"), lc_tree!("1"), 3)
        );

        assert_eq!(
            Vec::<i32>::new(),
            Solution::distance_k(lc_tree!("[1]"), lc_tree!("5"), 1)
        );

        assert_eq!(
            vec![7, 4, 1],
            Solution::distance_k(lc_tree!("[3,5,1,6,2,0,8,null,null,7,4]"), lc_tree!("5"), 2)
        );
    }
}
