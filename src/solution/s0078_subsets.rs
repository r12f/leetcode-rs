//!
//! Problem #78: Subsets
//!
//! - Link: <https://leetcode.com/problems/subsets/>
//! - Discussions: <https://leetcode.com/problems/subsets/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given an integer array `nums` of **unique** elements, return *all possible* *subsets* *(the power set)*.
//!
//! The solution set **must not** contain duplicate subsets. Return the solution in **any order**.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [1,2,3]
//! Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [0]
//! Output: [[],[0]]
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= nums.length <= 10`
//! * `-10 <= nums[i] <= 10`
//! * All the numbers of `nums` are **unique**.
//!

#[solution]
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results = Vec::new();

        let mut result = Vec::new();
        results.push(result.clone());

        Solution::dfs(&mut results, &mut result, &nums, 0);

        results
    }

    fn dfs(results: &mut Vec<Vec<i32>>, result: &mut Vec<i32>, nums: &Vec<i32>, start: usize) {
        if start == nums.len() {
            return;
        }

        for i in start..nums.len() {
            result.push(nums[i]);
            results.push(result.clone());
            Solution::dfs(results, result, nums, i + 1);
            result.pop();
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
    fn test_78() {
        assert_eq!(
            lc_vecvec![[], [1], [1, 2], [1, 2, 3], [1, 3], [2], [2, 3], [3]],
            Solution::subsets(vec![1, 2, 3])
        );
    }
}
