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
        0
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
    }
}
