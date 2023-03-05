use std::cell::RefCell;
use std::rc::Rc;

#[solution]
impl Solution {
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let mut level_sum = Vec::<i64>::new();

        Solution::dfs(root, 0, &mut level_sum);
        level_sum.sort();

        if level_sum.len() < k as usize {
            return -1;
        }

        level_sum[level_sum.len() - k as usize]
    }

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, level: i32, level_sum: &mut Vec<i64>) {
        if let Some(node) = node {
            if level_sum.len() <= level as usize {
                level_sum.resize(level as usize + 1, 0);
            }

            level_sum[level as usize] += node.borrow().val as i64;

            Solution::dfs(node.borrow().left.clone(), level + 1, level_sum);
            Solution::dfs(node.borrow().right.clone(), level + 1, level_sum);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_6308() {
        assert_eq!(
            13,
            Solution::kth_largest_level_sum(lc_tree!("[5,8,9,2,1,3,7,4,6]"), 2)
        );
    }
}
