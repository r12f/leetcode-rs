//!
//! Problem #523: Continuous Subarray Sum
//!
//! - Link: <https://leetcode.com/problems/continuous-subarray-sum/>
//! - Discussions: <https://leetcode.com/problems/continuous-subarray-sum/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given an integer array nums and an integer k, return `true` *if* `nums` *has a **good subarray** or* `false` *otherwise*.
//!
//! A **good subarray** is a subarray where:
//!
//! * its length is **at least two**, and
//! * the sum of the elements of the subarray is a multiple of `k`.
//!
//! **Note** that:
//!
//! * A **subarray** is a contiguous part of the array.
//! * An integer `x` is a multiple of `k` if there exists an integer `n` such that `x = n * k`. `0` is **always** a multiple of `k`.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [23,2,4,6,7], k = 6
//! Output: true
//! Explanation: [2, 4] is a continuous subarray of size 2 whose elements sum up to 6.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [23,2,6,4,7], k = 6
//! Output: true
//! Explanation: [23, 2, 6, 4, 7] is an continuous subarray of size 5 whose elements sum up to 42.
//! 42 is a multiple of 6 because 42 = 7 * 6 and 7 is an integer.
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: nums = [23,2,6,4,7], k = 13
//! Output: false
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= nums.length <= 10<sup>5</sup>`
//! * `0 <= nums[i] <= 10<sup>9</sup>`
//! * `0 <= sum(nums[i]) <= 2<sup>31</sup> - 1`
//! * `1 <= k <= 2<sup>31</sup> - 1`
//!

use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[solution]
impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut prefix_sum_pos = HashMap::<i32, i32>::new();
        prefix_sum_pos.insert(0, -1);

        let mut prefix_sum = 0;
        for (i, &n) in nums.iter().enumerate() {
            prefix_sum = (prefix_sum + n) % k;

            match prefix_sum_pos.entry(prefix_sum) {
                Entry::Vacant(e) => {
                    e.insert(i as i32);
                }
                Entry::Occupied(e) => {
                    let last_pos = e.get();
                    if *last_pos + 1 < i as i32 {
                        return true;
                    }
                }
            }
        }

        false
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
    fn test_523() {
        assert_eq!(false, Solution::check_subarray_sum(vec![0], 2));
        assert_eq!(false, Solution::check_subarray_sum(vec![6], 2));
        assert_eq!(false, Solution::check_subarray_sum(vec![5], 2));
        assert_eq!(true, Solution::check_subarray_sum(vec![0, 0], 2));
        assert_eq!(false, Solution::check_subarray_sum(vec![1, 0], 2));
        assert_eq!(false, Solution::check_subarray_sum(vec![1, 2], 2));
        assert_eq!(true, Solution::check_subarray_sum(vec![2, 4, 3], 6));
        assert_eq!(true, Solution::check_subarray_sum(vec![23, 2, 4, 6, 6], 7));

        assert_eq!(
            false,
            Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 13)
        );
    }
}
