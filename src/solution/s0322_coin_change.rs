//!
//! Problem #322: Coin Change
//!
//! - Link: <https://leetcode.com/problems/coin-change/>
//! - Discussions: <https://leetcode.com/problems/coin-change/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! You are given an integer array `coins` representing coins of different denominations and an integer `amount` representing a total amount of money.
//!
//! Return *the fewest number of coins that you need to make up that amount*. If that amount of money cannot be made up by any combination of the coins, return `-1`.
//!
//! You may assume that you have an infinite number of each kind of coin.
//!
//! **Example 1:**
//!
//! ```
//! Input: coins = [1,2,5], amount = 11
//! Output: 3
//! Explanation: 11 = 5 + 5 + 1
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: coins = [2], amount = 3
//! Output: -1
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: coins = [1], amount = 0
//! Output: 0
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= coins.length <= 12`
//! * `1 <= coins[i] <= 2<sup>31</sup> - 1`
//! * `0 <= amount <= 10<sup>4</sup>`
//!

#[solution]
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![i32::MAX; amount as usize + 1];
        dp[0] = 0;

        for target in 1..=amount {
            for coin in coins.iter() {
                if *coin <= target && dp[(target - *coin) as usize] != i32::MAX {
                    dp[target as usize] =
                        dp[target as usize].min(dp[(target - *coin) as usize] + 1);
                }
            }
        }

        let result = dp[amount as usize];
        if result == i32::MAX {
            return -1;
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
    fn test_322() {
        assert_eq!(3, Solution::coin_change(vec![1, 2, 5], 11));
        assert_eq!(-1, Solution::coin_change(vec![2], 3));
        assert_eq!(0, Solution::coin_change(vec![1], 0));
    }
}
