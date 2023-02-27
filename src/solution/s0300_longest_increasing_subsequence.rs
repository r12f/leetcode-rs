//!
//! Problem #300: Longest Increasing Subsequence
//!
//! - Link: <https://leetcode.com/problems/longest-increasing-subsequence/>
//! - Discussions: <https://leetcode.com/problems/longest-increasing-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given an integer array `nums`, return *the length of the longest **strictly increasing*** ***subsequence***.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [10,9,2,5,3,7,101,18]
//! Output: 4
//! Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [0,1,0,3,2,3]
//! Output: 4
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: nums = [7,7,7,7,7,7,7]
//! Output: 1
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= nums.length <= 2500`
//! * `-10<sup>4</sup> <= nums[i] <= 10<sup>4</sup>`
//!
//! **Follow up:** Can you come up with an algorithm that runs in `O(n log(n))` time complexity?
//!

#[solution]
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut result = 1;
        let mut dp = vec![1; nums.len()];

        for i in 1..nums.len() {
            for prev in 0..i {
                if nums[i] > nums[prev] {
                    dp[i] = dp[i].max(dp[prev] + 1);
                }
            }

            result = result.max(dp[i]);
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
    fn test_300() {
        assert_eq!(6, Solution::length_of_lis(vec![1, 3, 6, 7, 9, 4, 10, 5, 6]));
        assert_eq!(4, Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]));
        assert_eq!(4, Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]));
        assert_eq!(1, Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]));
    }
}
