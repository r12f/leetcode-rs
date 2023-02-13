//!
//! Problem #314: Binary Tree Vertical Order Traversal
//!
//! - Link: <https://leetcode.com/problems/binary-tree-vertical-order-traversal/>
//! - Discussions: <https://leetcode.com/problems/binary-tree-vertical-order-traversal/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given the `root` of a binary tree, return ***the vertical order traversal** of its nodes' values*. (i.e., from top to bottom, column by column).
//!
//! If two nodes are in the same row and column, the order should be from **left to right**.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/01/28/vtree1.jpg)
//!
//! ```
//! Input: root = [3,9,20,null,null,15,7]
//! Output: [[9],[3,15],[20],[7]]
//!
//! ```
//!
//! **Example 2:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/01/28/vtree2-1.jpg)
//!
//! ```
//! Input: root = [3,9,8,4,0,1,7]
//! Output: [[4],[9],[3,0,1],[8],[7]]
//!
//! ```
//!
//! **Example 3:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/01/28/vtree2.jpg)
//!
//! ```
//! Input: root = [3,9,8,4,0,1,7,null,null,null,2,5]
//! Output: [[4],[9,5],[3,0,1],[8,2],[7]]
//!
//! ```
//!
//! **Constraints:**
//!
//! * The number of nodes in the tree is in the range `[0, 100]`.
//! * `-100 <= Node.val <= 100`
//!

use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};
use std::io::Seek;
use std::rc::Rc;

use lcrt::TreeNode;
#[solution]
impl Solution {
    pub fn vertical_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut value_map = BTreeMap::<i32, Vec<i32>>::new();

        let mut queue = VecDeque::new();
        queue.push_back((root.clone(), 0));

        while (!queue.is_empty()) {
            let (node, level) = queue.pop_front().unwrap();
            if node.is_none() {
                continue;
            }

            let node_ref = node.as_ref().unwrap().borrow();

            let values = value_map.entry(level).or_default();
            values.push(node_ref.val);

            queue.push_back((node_ref.left.clone(), level - 1));
            queue.push_back((node_ref.right.clone(), level + 1));
        }

        let result = value_map
            .into_iter()
            .map(|(k, v)| v)
            .collect::<Vec<Vec<i32>>>();

        return result;
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
    fn test_314() {
        assert_eq!(
            lc_vecvec![[9], [3, 15], [20], [7]],
            Solution::vertical_order(lc_tree!("3,9,20,null,null,15,7"))
        );

        assert_eq!(
            lc_vecvec![[4], [9], [3, 0, 1], [8], [7]],
            Solution::vertical_order(lc_tree!("3,9,8,4,0,1,7"))
        );

        assert_eq!(
            lc_vecvec![[4], [9, 5], [3, 0, 1], [8, 2], [7]],
            Solution::vertical_order(lc_tree!("3,9,8,4,0,1,7,null,null,null,2,5"))
        );
    }
}
