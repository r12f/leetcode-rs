//!
//! Problem #239: Sliding Window Maximum
//!
//! - Link: <https://leetcode.com/problems/sliding-window-maximum/>
//! - Discussions: <https://leetcode.com/problems/sliding-window-maximum/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! You are given an array of integers `nums`, there is a sliding window of size `k` which is moving from the very left of the array to the very right. You can only see the `k` numbers in the window. Each time the sliding window moves right by one position.
//!
//! Return *the max sliding window*.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [1,3,-1,-3,5,3,6,7], k = 3
//! Output: [3,3,5,5,6,7]
//! Explanation:
//! Window position                Max
//! ---------------               -----
//! [1  3  -1] -3  5  3  6  7       3
//!  1 [3  -1  -3] 5  3  6  7       3
//!  1  3 [-1  -3  5] 3  6  7       5
//!  1  3  -1 [-3  5  3] 6  7       5
//!  1  3  -1  -3 [5  3  6] 7       6
//!  1  3  -1  -3  5 [3  6  7]      7
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [1], k = 1
//! Output: [1]
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= nums.length <= 10<sup>5</sup>`
//! * `-10<sup>4</sup> <= nums[i] <= 10<sup>4</sup>`
//! * `1 <= k <= nums.length`
//!

use std::collections::VecDeque;

#[solution]
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut results = Vec::new();

        let mut max_queue = VecDeque::<(usize, i32)>::new();
        for (i, n) in nums.iter().enumerate() {
            Solution::push_to_window(&mut max_queue, i, *n, k as usize);
            if i + 1 >= k as usize {
                results.push(max_queue.front().unwrap().1);
            }
        }

        results
    }

    fn push_to_window(
        max_queue: &mut VecDeque<(usize, i32)>,
        i: usize,
        n: i32,
        window_size: usize,
    ) {
        // Remove all outdated
        while let Some((bottom_index, _)) = max_queue.front() {
            if bottom_index + window_size <= i {
                max_queue.pop_front();
            } else {
                break;
            }
        }

        // Remove all smaller
        while let Some((_, top_num)) = max_queue.back() {
            if *top_num < n {
                max_queue.pop_back();
            } else {
                break;
            }
        }

        max_queue.push_back((i, n));
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
    fn test_239() {
        assert_eq!(
            vec![3, 3, 5, 5, 6, 7],
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3)
        );
    }
}
