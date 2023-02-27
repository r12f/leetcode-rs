//!
//! Problem #632: Smallest Range Covering Elements from K Lists
//!
//! - Link: <https://leetcode.com/problems/smallest-range-covering-elements-from-k-lists/>
//! - Discussions: <https://leetcode.com/problems/smallest-range-covering-elements-from-k-lists/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! You have `k` lists of sorted integers in **non-decreasing order**. Find the **smallest** range that includes at least one number from each of the `k` lists.
//!
//! We define the range `[a, b]` is smaller than range `[c, d]` if `b - a < d - c` **or** `a < c` if `b - a == d - c`.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [[4,10,15,24,26],[0,9,12,20],[5,18,22,30]]
//! Output: [20,24]
//! Explanation:
//! List 1: [4, 10, 15, 24,26], 24 is in range [20,24].
//! List 2: [0, 9, 12, 20], 20 is in range [20,24].
//! List 3: [5, 18, 22, 30], 22 is in range [20,24].
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [[1,2,3],[1,2,3],[1,2,3]]
//! Output: [1,1]
//!
//! ```
//!
//! **Constraints:**
//!
//! * `nums.length == k`
//! * `1 <= k <= 3500`
//! * `1 <= nums[i].length <= 50`
//! * `-10<sup>5</sup> <= nums[i][j] <= 10<sup>5</sup>`
//! * `nums[i]` is sorted in **non-decreasing** order.
//!

use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct State {
    num: i32,
    queue: usize,
    index: usize,
}

impl State {
    pub fn new(num: i32, queue: usize, index: usize) -> Self {
        State { num, queue, index }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.num.cmp(&self.num)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[solution]
impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result_min = 0;
        let mut result_max = i32::MAX;

        let mut min = i32::MIN;
        let mut max = i32::MIN;

        let mut heap = BinaryHeap::<State>::new();
        for i in 0..nums.len() {
            heap.push(State::new(nums[i][0], i, 0));
            max = max.max(nums[i][0]);
        }

        while let Some(state) = heap.pop() {
            min = state.num;
            if max - min < result_max - result_min {
                result_min = min;
                result_max = max;
            }

            // If one queue is drained, then there is no way to make the range smaller.
            if state.index == nums[state.queue].len() - 1 {
                break;
            }

            let next = State::new(
                nums[state.queue][state.index + 1],
                state.queue,
                state.index + 1,
            );
            max = max.max(next.num);
            heap.push(next);
        }

        vec![result_min, result_max]
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
    fn test_632() {
        assert_eq!(
            vec![1, 1],
            Solution::smallest_range(lc_vecvec![[1, 2, 3], [1, 2, 3], [1, 2, 3]])
        );

        assert_eq!(
            vec![20, 24],
            Solution::smallest_range(lc_vecvec![
                [4, 10, 15, 24, 26],
                [0, 9, 12, 20],
                [5, 18, 22, 30]
            ])
        );
    }
}
