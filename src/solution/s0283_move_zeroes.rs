//!
//! Problem #283: Move Zeroes
//!
//! - Link: <https://leetcode.com/problems/move-zeroes/>
//! - Discussions: <https://leetcode.com/problems/move-zeroes/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given an integer array `nums`, move all `0`'s to the end of it while maintaining the relative order of the non-zero elements.
//!
//! **Note** that you must do this in-place without making a copy of the array.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [0,1,0,3,12]
//! Output: [1,3,12,0,0]
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [0]
//! Output: [0]
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= nums.length <= 10<sup>4</sup>`
//! * `-2<sup>31</sup> <= nums[i] <= 2<sup>31</sup> - 1`
//!
//! **Follow up:** Could you minimize the total number of operations done?
//!

#[solution]
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut next_slot = 0usize;
        let mut i = 0usize;
        while i < nums.len() {
            if nums[i] != 0 {
                nums[next_slot] = nums[i];
                next_slot += 1;
            }

            i += 1;
        }

        for i in next_slot..nums.len() {
            nums[i] = 0;
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
    fn test_283() {
        let mut v = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut v);

        assert_eq!(vec![1, 3, 12, 0, 0], v);
    }
}
