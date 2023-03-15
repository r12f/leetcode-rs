//!
//! Problem #334: Increasing Triplet Subsequence
//!
//! - Link: <https://leetcode.com/problems/increasing-triplet-subsequence/>
//! - Discussions: <https://leetcode.com/problems/increasing-triplet-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given an integer array `nums`, return `true` *if there exists a triple of indices* `(i, j, k)` *such that* `i < j < k` *and* `nums[i] < nums[j] < nums[k]`. If no such indices exists, return `false`.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [1,2,3,4,5]
//! Output: true
//! Explanation: Any triplet where i < j < k is valid.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [5,4,3,2,1]
//! Output: false
//! Explanation: No triplet exists.
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: nums = [2,1,5,0,4,6]
//! Output: true
//! Explanation: The triplet (3, 4, 5) is valid because nums[3] == 0 < nums[4] == 4 < nums[5] == 6.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= nums.length <= 5 * 10<sup>5</sup>`
//! * `-2<sup>31</sup> <= nums[i] <= 2<sup>31</sup> - 1`
//!
//! **Follow up:** Could you implement a solution that runs in `O(n)` time complexity and `O(1)` space complexity?
//!

#[solution]
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut min_num = i32::MAX;
        let mut mid_num = i32::MAX;
        let mut high_num = i32::MAX;

        for n in nums {
            if n <= min_num {
                min_num = n;
            } else if n <= mid_num {
                mid_num = n;
            } else if n <= high_num {
                return true;
            }
        }

        return false;
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
    fn test_334() {}
}
