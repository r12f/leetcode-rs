//!
//! Problem #136: Single Number
//!
//! - Link: <https://leetcode.com/problems/single-number/>
//! - Discussions: <https://leetcode.com/problems/single-number/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given a **non-empty** array of integers `nums`, every element appears *twice* except for one. Find that single one.
//!
//! You must implement a solution with a linear runtime complexity and use only constant extra space.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [2,2,1]
//! Output: 1
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [4,1,2,1,2]
//! Output: 4
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: nums = [1]
//! Output: 1
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= nums.length <= 3 * 10<sup>4</sup>`
//! * `-3 * 10<sup>4</sup> <= nums[i] <= 3 * 10<sup>4</sup>`
//! * Each element in the array appears twice except for one element which appears only once.
//!

#[solution]
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        for num in nums {
            result ^= num;
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
    fn test_136() {
        assert_eq!(1, Solution::single_number(vec![2, 2, 1]));
    }
}
