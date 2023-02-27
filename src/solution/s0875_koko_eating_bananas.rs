//!
//! Problem #875: Koko Eating Bananas
//!
//! - Link: <https://leetcode.com/problems/koko-eating-bananas/>
//! - Discussions: <https://leetcode.com/problems/koko-eating-bananas/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Koko loves to eat bananas. There are `n` piles of bananas, the `i<sup>th</sup>` pile has `piles[i]` bananas. The guards have gone and will come back in `h` hours.
//!
//! Koko can decide her bananas-per-hour eating speed of `k`. Each hour, she chooses some pile of bananas and eats `k` bananas from that pile. If the pile has less than `k` bananas, she eats all of them instead and will not eat any more bananas during this hour.
//!
//! Koko likes to eat slowly but still wants to finish eating all the bananas before the guards return.
//!
//! Return *the minimum integer* `k` *such that she can eat all the bananas within* `h` *hours*.
//!
//! **Example 1:**
//!
//! ```
//! Input: piles = [3,6,7,11], h = 8
//! Output: 4
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: piles = [30,11,23,4,20], h = 5
//! Output: 30
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: piles = [30,11,23,4,20], h = 6
//! Output: 23
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= piles.length <= 10<sup>4</sup>`
//! * `piles.length <= h <= 10<sup>9</sup>`
//! * `1 <= piles[i] <= 10<sup>9</sup>`
//!

#[solution]
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut right = i32::MIN;
        for pile in piles.iter() {
            right = right.max(*pile);
        }

        let mut left = 1;
        while left < right {
            let mid = (left + right) / 2;

            let hours: i32 = piles
                .iter()
                .map(|v| (*v as f64 / mid as f64).ceil() as i32)
                .sum();
            if hours <= h {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
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
    fn test_875() {
        assert_eq!(3, Solution::min_eating_speed(vec![3], 1));
        assert_eq!(1, Solution::min_eating_speed(vec![3], 3));
        assert_eq!(4, Solution::min_eating_speed(vec![3, 6, 7, 11], 8));
    }
}
