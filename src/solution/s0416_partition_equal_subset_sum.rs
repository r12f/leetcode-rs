//!
//! Problem #416: Partition Equal Subset Sum
//!
//! - Link: <https://leetcode.com/problems/partition-equal-subset-sum/>
//! - Discussions: <https://leetcode.com/problems/partition-equal-subset-sum/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given an integer array `nums`, return `true` *if you can partition the array into two subsets such that the sum of the elements in both subsets is equal or* `false` *otherwise*.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [1,5,11,5]
//! Output: true
//! Explanation: The array can be partitioned as [1, 5, 5] and [11].
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [1,2,3,5]
//! Output: false
//! Explanation: The array cannot be partitioned into equal sum subsets.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= nums.length <= 200`
//! * `1 <= nums[i] <= 100`
//!

use std::collections::HashSet;

#[solution]
impl Solution {
    pub fn can_partition(mut nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum % 2 == 1 {
            return false;
        }

        let target_sum = sum / 2;
        let mut found_sums = HashSet::<i32>::new();
        found_sums.insert(0);
        for n in nums {
            let mut new_sums = vec![];
            for prev_sum in found_sums.iter() {
                let new_sum = *prev_sum + n;
                if new_sum == target_sum {
                    return true;
                }

                new_sums.push(new_sum);
            }

            for new_sum in new_sums {
                found_sums.insert(new_sum);
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
    fn test_416() {
        assert_eq!(true, Solution::can_partition(vec![1, 5, 11, 5]));
        assert_eq!(false, Solution::can_partition(vec![1, 2, 3, 5]));
    }
}
