//!
//! Problem #437: Path Sum III
//!
//! - Link: <https://leetcode.com/problems/path-sum-iii/>
//! - Discussions: <https://leetcode.com/problems/path-sum-iii/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given the `root` of a binary tree and an integer `targetSum`, return *the number of paths where the sum of the values along the path equals* `targetSum`.
//!
//! The path does not need to start or end at the root or a leaf, but it must go downwards (i.e., traveling only from parent nodes to child nodes).
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/04/09/pathsum3-1-tree.jpg)
//!
//! ```
//! Input: root = [10,5,-3,3,2,null,11,3,-2,null,1], targetSum = 8
//! Output: 3
//! Explanation: The paths that sum to 8 are shown.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22
//! Output: 3
//!
//! ```
//!
//! **Constraints:**
//!
//! * The number of nodes in the tree is in the range `[0, 1000]`.
//! * `-10<sup>9</sup> <= Node.val <= 10<sup>9</sup>`
//! * `-1000 <= targetSum <= 1000`
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
#[solution]
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut prefix_sums = HashMap::<i64, i32>::new();
        prefix_sums.insert(0, 1);

        Solution::dfs(root, target_sum as i64, 0, &mut prefix_sums)
    }

    fn dfs(
        node: Option<Rc<RefCell<TreeNode>>>,
        target_sum: i64,
        parent_sum: i64,
        prefix_sums: &mut HashMap<i64, i32>,
    ) -> i32 {
        let mut result = 0;

        match node {
            None => return 0,
            Some(node) => {
                let val = node.borrow_mut().val;
                let sum = parent_sum + val as i64;
                let to_find = sum - target_sum;
                if let Some(n) = prefix_sums.get(&to_find) {
                    result += n;
                }

                *prefix_sums.entry(sum).or_default() += 1;

                result +=
                    Solution::dfs(node.borrow_mut().left.clone(), target_sum, sum, prefix_sums);

                result += Solution::dfs(
                    node.borrow_mut().right.clone(),
                    target_sum,
                    sum,
                    prefix_sums,
                );

                let saved_sum = prefix_sums.get_mut(&sum).unwrap();
                *saved_sum -= 1;
                if *saved_sum == 0 {
                    prefix_sums.remove(&sum);
                }
            }
        }

        result
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
    fn test_437() {
        assert_eq!(
            3,
            Solution::path_sum(lc_tree!("10, 5, -3, 3, 2, null, 11, 3, -2, null, 1"), 8)
        );

        assert_eq!(
            3,
            Solution::path_sum(lc_tree!("5,4,8,11,null,13,4,7,2,null,null,5,1"), 22)
        );

        assert_eq!(
            0,
            Solution::path_sum(lc_tree!("1000000000,1000000000,null,294967296,null,1000000000,null,1000000000,null,1000000000"), 0)
        );
    }
}
