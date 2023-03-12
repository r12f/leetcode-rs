//!
//! Problem #215: Kth Largest Element in an Array
//!
//! - Link: <https://leetcode.com/problems/kth-largest-element-in-an-array/>
//! - Discussions: <https://leetcode.com/problems/kth-largest-element-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given an integer array `nums` and an integer `k`, return *the* `k<sup>th</sup>` *largest element in the array*.
//!
//! Note that it is the `k<sup>th</sup>` largest element in the sorted order, not the `k<sup>th</sup>` distinct element.
//!
//! You must solve it in `O(n)` time complexity.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [3,2,1,5,6,4], k = 2
//! Output: 5
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [3,2,3,1,2,4,5,5,6], k = 4
//! Output: 4
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= k <= nums.length <= 10<sup>5</sup>`
//! * `-10<sup>4</sup> <= nums[i] <= 10<sup>4</sup>`
//!

use rand::Rng;

#[solution]
impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let right = nums.len() - 1;
        Self::kth_element(&mut nums, right + 1 - k as usize, 0, right)
    }

    fn kth_element(nums: &mut Vec<i32>, k: usize, left: usize, right: usize) -> i32 {
        let (index, n) = Self::partition(nums, left, right);

        if index == k {
            return n;
        } else if index > k {
            return Self::kth_element(nums, k, left, index - 1);
        } else {
            return Self::kth_element(nums, k, index + 1, right);
        }
    }

    fn partition(nums: &mut Vec<i32>, par_left: usize, par_right: usize) -> (usize, i32) {
        if par_left == par_right {
            return (par_left, nums[par_left]);
        }

        let mut left = par_left;
        let mut right = par_right;

        while left < right {
            while left < par_right && nums[left] < nums[par_right] {
                left += 1;
            }

            while par_left < right && nums[right] >= nums[par_right] {
                right -= 1;
            }

            if left >= right {
                break;
            }

            nums.swap(left, right);
        }

        nums.swap(par_right, left);

        (left, nums[left])
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
    fn test_215() {
        assert_eq!(0, Solution::find_kth_largest(vec![0, 1], 2));

        assert_eq!(
            4,
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4)
        );

        assert_eq!(5, Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2));
    }
}
