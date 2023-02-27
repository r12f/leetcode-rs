//!
//! Problem #528: Random Pick with Weight
//!
//! - Link: <https://leetcode.com/problems/random-pick-with-weight/>
//! - Discussions: <https://leetcode.com/problems/random-pick-with-weight/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! You are given a **0-indexed** array of positive integers `w` where `w[i]` describes the **weight** of the `i<sup>th</sup>` index.
//!
//! You need to implement the function `pickIndex()`, which **randomly** picks an index in the range `[0, w.length - 1]` (**inclusive**) and returns it. The **probability** of picking an index `i` is `w[i] / sum(w)`.
//!
//! * For example, if `w = [1, 3]`, the probability of picking index `0` is `1 / (1 + 3) = 0.25` (i.e., `25%`), and the probability of picking index `1` is `3 / (1 + 3) = 0.75` (i.e., `75%`).
//!
//! **Example 1:**
//!
//! ```
//! Input
//! ["Solution","pickIndex"]
//! [[[1]],[]]
//! Output
//! [null,0]
//!
//! Explanation
//! Solution solution = new Solution([1]);
//! solution.pickIndex(); // return 0. The only option is to return 0 since there is only one element in w.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input
//! ["Solution","pickIndex","pickIndex","pickIndex","pickIndex","pickIndex"]
//! [[[1,3]],[],[],[],[],[]]
//! Output
//! [null,1,1,1,1,0]
//!
//! Explanation
//! Solution solution = new Solution([1, 3]);
//! solution.pickIndex(); // return 1. It is returning the second element (index = 1) that has a probability of 3/4.
//! solution.pickIndex(); // return 1
//! solution.pickIndex(); // return 1
//! solution.pickIndex(); // return 1
//! solution.pickIndex(); // return 0. It is returning the first element (index = 0) that has a probability of 1/4.
//!
//! Since this is a randomization problem, multiple answers are allowed.
//! All of the following outputs can be considered correct:
//! [null,1,1,1,1,0]
//! [null,1,1,1,1,1]
//! [null,1,1,1,0,0]
//! [null,1,1,1,0,1]
//! [null,1,0,1,0,0]
//! ......
//! and so on.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= w.length <= 10<sup>4</sup>`
//! * `1 <= w[i] <= 10<sup>5</sup>`
//! * `pickIndex` will be called at most `10<sup>4</sup>` times.
//!

use std::collections::BTreeMap;

use rand::{distributions::Uniform, prelude::*};

struct Solution {
    distribution: Uniform<i32>,
    rng: rand::rngs::ThreadRng,
    range_map: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(w: Vec<i32>) -> Self {
        let mut range_start = 0;
        let mut range_map = BTreeMap::new();

        for (i, weight) in w.iter().enumerate() {
            range_map.insert(range_start, i as i32);
            range_start += *weight;
        }

        Solution {
            distribution: Uniform::from(0..range_start),
            rng: rand::thread_rng(),
            range_map,
        }
    }

    fn pick_index(&mut self) -> i32 {
        let num = self.distribution.sample(&mut self.rng);

        match self.range_map.range(..=num).next_back() {
            None => 0,
            Some(v) => *v.1,
        }
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(w);
 * let ret_1: i32 = obj.pick_index();
 */

//
// Tests
//
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_528() {
        let mut s = Solution::new(vec![1, 3]);
        println!("{}", s.pick_index());
        println!("{}", s.pick_index());
        println!("{}", s.pick_index());
        println!("{}", s.pick_index());
    }
}
