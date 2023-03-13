//!
//! Problem #560: Subarray Sum Equals K
//!
//! - Link: <https://leetcode.com/problems/subarray-sum-equals-k/>
//! - Discussions: <https://leetcode.com/problems/subarray-sum-equals-k/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given an array of integers `nums` and an integer `k`, return *the total number of subarrays whose sum equals to* `k`.
//!
//! A subarray is a contiguous **non-empty** sequence of elements within an array.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [1,1,1], k = 2
//! Output: 2
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [1,2,3], k = 3
//! Output: 2
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= nums.length <= 2 * 10<sup>4</sup>`
//! * `-1000 <= nums[i] <= 1000`
//! * `-10<sup>7</sup> <= k <= 10<sup>7</sup>`
//!

use std::collections::HashMap;

#[solution]
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;

        let mut prefix_sum = 0;
        let mut prefix_sum_counts = HashMap::<i32, i32>::new();
        prefix_sum_counts.insert(0, 1);

        for num in nums {
            prefix_sum += num;

            let needed_value = prefix_sum - k;
            if let Some(&needed_value_count) = prefix_sum_counts.get(&needed_value) {
                result += needed_value_count;
            }

            *prefix_sum_counts.entry(prefix_sum).or_insert(0) += 1;
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
    fn test_560() {
        assert_eq!(2, Solution::subarray_sum(vec![1, 1, 1], 2));
    }
}
