//!
//! Problem #56: Merge Intervals
//!
//! - Link: <https://leetcode.com/problems/merge-intervals/>
//! - Discussions: <https://leetcode.com/problems/merge-intervals/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given an array of `intervals` where `intervals[i] = [start<sub>i</sub>, end<sub>i</sub>]`, merge all overlapping intervals, and return *an array of the non-overlapping intervals that cover all the intervals in the input*.
//!
//! **Example 1:**
//!
//! ```
//! Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
//! Output: [[1,6],[8,10],[15,18]]
//! Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: intervals = [[1,4],[4,5]]
//! Output: [[1,5]]
//! Explanation: Intervals [1,4] and [4,5] are considered overlapping.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= intervals.length <= 10<sup>4</sup>`
//! * `intervals[i].length == 2`
//! * `0 <= start<sub>i</sub> <= end<sub>i</sub> <= 10<sup>4</sup>`
//!

use std::cmp::Ordering;

#[solution]
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|l, r| match l[0].cmp(&r[0]) {
            Ordering::Equal => l[1].cmp(&r[1]),
            v => v,
        });

        let mut result: Vec<Vec<i32>> = Vec::new();

        for interval in intervals {
            if result.is_empty() || result.last().unwrap()[1] < interval[0] {
                result.push(interval);
            } else {
                if result.last().unwrap()[1] < interval[1] {
                    result.last_mut().unwrap()[1] = interval[1];
                }
            }
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
    fn test_56() {
        assert_eq!(
            lc_vecvec![[1, 6], [8, 10], [15, 18]],
            Solution::merge(lc_vecvec![[1, 3], [2, 6], [8, 10], [15, 18]])
        );
    }
}
