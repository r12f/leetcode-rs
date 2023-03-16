//!
//! Problem #162: Find Peak Element
//!
//! - Link: <https://leetcode.com/problems/find-peak-element/>
//! - Discussions: <https://leetcode.com/problems/find-peak-element/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! A peak element is an element that is strictly greater than its neighbors.
//!
//! Given a **0-indexed** integer array `nums`, find a peak element, and return its index. If the array contains multiple peaks, return the index to **any of the peaks**.
//!
//! You may imagine that `nums[-1] = nums[n] = -∞`. In other words, an element is always considered to be strictly greater than a neighbor that is outside the array.
//!
//! You must write an algorithm that runs in `O(log n)` time.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [1,2,3,1]
//! Output: 2
//! Explanation: 3 is a peak element and your function should return the index number 2.
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [1,2,1,3,5,6,4]
//! Output: 5
//! Explanation: Your function can return either index number 1 where the peak element is 2, or index number 5 where the peak element is 6.
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= nums.length <= 1000`
//! * `-2<sup>31</sup> <= nums[i] <= 2<sup>31</sup> - 1`
//! * `nums[i] != nums[i + 1]` for all valid `i`.
//!

#[solution]
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left <= right {
            let mid = (left + right) / 2;

            if mid > 0 && nums[mid] < nums[mid - 1] {
                right = mid - 1;
            } else if mid < nums.len() - 1 && nums[mid] < nums[mid + 1] {
                left = mid + 1;
            } else {
                return mid as i32;
            }
        }

        return -1;
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
    fn test_162() {
        assert_eq!(0, Solution::find_peak_element(vec![1]));
        assert_eq!(1, Solution::find_peak_element(vec![1, 2]));
        assert_eq!(2, Solution::find_peak_element(vec![1, 2, 3, 1]));
        assert_eq!(5, Solution::find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]));
    }
}
