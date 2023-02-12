//!
//! Problem #253: Meeting Rooms II
//!
//! - Link: <https://leetcode.com/problems/meeting-rooms-ii/>
//! - Discussions: <https://leetcode.com/problems/meeting-rooms-ii/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given an array of meeting time intervals `intervals` where `intervals[i] = [start<sub>i</sub>, end<sub>i</sub>]`, return *the minimum number of conference rooms required*.
//!
//! **Example 1:**
//!
//! ```
//! Input: intervals = [[0,30],[5,10],[15,20]]
//! Output: 2
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: intervals = [[7,10],[2,4]]
//! Output: 1
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= intervals.length <= 10<sup>4</sup>`
//! * `0 <= start<sub>i</sub> < end<sub>i</sub> <= 10<sup>6</sup>`
//!
#[derive(Eq, PartialEq)]
struct Interval(i32, i32);

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match other.1.cmp(&self.1) {
            Ordering::Equal => other.0.cmp(&self.0),
            v @ _ => v,
        }
    }
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match other.1.partial_cmp(&self.1) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        other.0.partial_cmp(&self.0)
    }
}

use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};
#[solution]
impl Solution {
    pub fn min_meeting_rooms(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.len() == 0 {
            return 0;
        }

        intervals.sort_by(|l, r| {
            if l[0] == r[0] {
                l[1].cmp(&r[1])
            } else {
                l[0].cmp(&r[0])
            }
        });

        let mut heap = BinaryHeap::<Interval>::new();

        let mut room_count = 0;
        for interval in &intervals {
            while heap.len() > 0 && interval[0] >= heap.peek().unwrap().1 {
                heap.pop();
            }
            heap.push(Interval(interval[0], interval[1]));

            if heap.len() > room_count {
                room_count = heap.len()
            }
        }

        return room_count as i32;
    }
}

//
// Tests
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_253() {
        assert_eq!(Solution::min_meeting_rooms(vec![vec![0, 1], vec![1, 2]]), 1);

        assert_eq!(
            Solution::min_meeting_rooms(vec![vec![0, 30], vec![5, 10], vec![15, 20]]),
            2
        );

        assert_eq!(
            Solution::min_meeting_rooms(vec![vec![6, 17], vec![8, 9], vec![11, 12], vec![6, 9]]),
            3
        );
    }
}
