//!
//! Problem #31: Next Permutation
//!
//! - Link: <https://leetcode.com/problems/next-permutation/>
//! - Discussions: <https://leetcode.com/problems/next-permutation/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! A **permutation** of an array of integers is an arrangement of its members into a sequence or linear order.
//!
//! * For example, for `arr = [1,2,3]`, the following are all the permutations of `arr`: `[1,2,3], [1,3,2], [2, 1, 3], [2, 3, 1], [3,1,2], [3,2,1]`.
//!
//! The **next permutation** of an array of integers is the next lexicographically greater permutation of its integer. More formally, if all the permutations of the array are sorted in one container according to their lexicographical order, then the **next permutation** of that array is the permutation that follows it in the sorted container. If such arrangement is not possible, the array must be rearranged as the lowest possible order (i.e., sorted in ascending order).
//!
//! * For example, the next permutation of `arr = [1,2,3]` is `[1,3,2]`.
//! * Similarly, the next permutation of `arr = [2,3,1]` is `[3,1,2]`.
//! * While the next permutation of `arr = [3,2,1]` is `[1,2,3]` because `[3,2,1]` does not have a lexicographical larger rearrangement.
//!
//! Given an array of integers `nums`, *find the next permutation of* `nums`.
//!
//! The replacement must be **[in place](http://en.wikipedia.org/wiki/In-place_algorithm)** and use only constant extra memory.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [1,2,3]
//! Output: [1,3,2]
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [3,2,1]
//! Output: [1,2,3]
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: nums = [1,1,5]
//! Output: [1,5,1]
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= nums.length <= 100`
//! * `0 <= nums[i] <= 100`
//!

#[solution]
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut inc_range_end = (nums.len() - 1);
        while inc_range_end > 0 && nums[inc_range_end - 1] >= nums[inc_range_end] {
            inc_range_end -= 1;
        }

        if inc_range_end > 0 {
            // Find the smallest number greater than our previous one.
            let mut to_swap_index = nums.len() - 1;
            while to_swap_index > inc_range_end && nums[to_swap_index] <= nums[inc_range_end - 1] {
                to_swap_index -= 1;
            }

            let mut temp = 0;
            std::mem::swap(&mut nums[inc_range_end - 1], &mut temp);
            std::mem::swap(&mut nums[to_swap_index], &mut temp);
            std::mem::swap(&mut nums[inc_range_end - 1], &mut temp);
        }

        nums[inc_range_end..].sort();
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
    fn test_31() {
        let mut arr = vec![1, 1];
        Solution::next_permutation(&mut arr);
        assert_eq!(vec![1, 1], arr);

        let mut arr = vec![1, 1, 5];
        Solution::next_permutation(&mut arr);
        assert_eq!(vec![1, 5, 1], arr);

        let mut arr = vec![5, 1, 1];
        Solution::next_permutation(&mut arr);
        assert_eq!(vec![1, 1, 5], arr);

        let mut arr = vec![1, 1, 3, 3, 5, 2];
        Solution::next_permutation(&mut arr);
        assert_eq!(vec![1, 1, 3, 5, 2, 3], arr);

        let mut arr = vec![1, 2, 3];
        Solution::next_permutation(&mut arr);
        assert_eq!(vec![1, 3, 2], arr);

        let mut arr = vec![1, 3, 2];
        Solution::next_permutation(&mut arr);
        assert_eq!(vec![2, 1, 3], arr);
    }
}
