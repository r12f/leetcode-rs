//!
//! Problem #2136: Earliest Possible Day of Full Bloom
//!
//! - Link: <https://leetcode.com/problems/earliest-possible-day-of-full-bloom/>
//! - Discussions: <https://leetcode.com/problems/earliest-possible-day-of-full-bloom/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! You have `n` flower seeds. Every seed must be planted first before it can begin to grow, then bloom. Planting a seed takes time and so does the growth of a seed. You are given two **0-indexed** integer arrays `plantTime` and `growTime`, of length `n` each:
//!
//! * `plantTime[i]` is the number of **full days** it takes you to **plant** the `i<sup>th</sup>` seed. Every day, you can work on planting exactly one seed. You **do not** have to work on planting the same seed on consecutive days, but the planting of a seed is not complete **until** you have worked `plantTime[i]` days on planting it in total.
//! * `growTime[i]` is the number of **full days** it takes the `i<sup>th</sup>` seed to grow after being completely planted. **After** the last day of its growth, the flower **blooms** and stays bloomed forever.
//!
//! From the beginning of day `0`, you can plant the seeds in **any** order.
//!
//! Return *the **earliest** possible day where **all** seeds are blooming*.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/12/21/1.png)
//!
//! ```
//! Input: plantTime = [1,4,3], growTime = [2,3,1]
//! Output: 9
//! Explanation: The grayed out pots represent planting days, colored pots represent growing days, and the flower represents the day it blooms.
//! One optimal way is:
//! On day 0, plant the 0th seed. The seed grows for 2 full days and blooms on day 3.
//! On days 1, 2, 3, and 4, plant the 1st seed. The seed grows for 3 full days and blooms on day 8.
//! On days 5, 6, and 7, plant the 2nd seed. The seed grows for 1 full day and blooms on day 9.
//! Thus, on day 9, all the seeds are blooming.
//!
//! ```
//!
//! **Example 2:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/12/21/2.png)
//!
//! ```
//! Input: plantTime = [1,2,3,2], growTime = [2,1,2,1]
//! Output: 9
//! Explanation: The grayed out pots represent planting days, colored pots represent growing days, and the flower represents the day it blooms.
//! One optimal way is:
//! On day 1, plant the 0th seed. The seed grows for 2 full days and blooms on day 4.
//! On days 0 and 3, plant the 1st seed. The seed grows for 1 full day and blooms on day 5.
//! On days 2, 4, and 5, plant the 2nd seed. The seed grows for 2 full days and blooms on day 8.
//! On days 6 and 7, plant the 3rd seed. The seed grows for 1 full day and blooms on day 9.
//! Thus, on day 9, all the seeds are blooming.
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: plantTime = [1], growTime = [1]
//! Output: 2
//! Explanation: On day 0, plant the 0th seed. The seed grows for 1 full day and blooms on day 2.
//! Thus, on day 2, all the seeds are blooming.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `n == plantTime.length == growTime.length`
//! * `1 <= n <= 10<sup>5</sup>`
//! * `1 <= plantTime[i], growTime[i] <= 10<sup>4</sup>`
//!

use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct Flower {
    plant_time: i32,
    grow_time: i32,
}

impl Ord for Flower {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.grow_time.cmp(&other.grow_time)
    }
}

impl PartialOrd for Flower {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[solution]
impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::<Flower>::new();

        for i in 0..plant_time.len() {
            heap.push(Flower {
                plant_time: plant_time[i],
                grow_time: grow_time[i],
            });
        }

        let mut plant_days = 0;
        let mut max_bloom_days = 0;
        while let Some(flower) = heap.pop() {
            plant_days += flower.plant_time;
            max_bloom_days = max_bloom_days.max(plant_days + flower.grow_time);
        }

        max_bloom_days
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
    fn test_2136() {}
}
