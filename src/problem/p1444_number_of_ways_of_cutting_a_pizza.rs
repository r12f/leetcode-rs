//!
//! Problem #1444: Number of Ways of Cutting a Pizza
//!
//! - Link: <https://leetcode.com/problems/number-of-ways-of-cutting-a-pizza/>
//! - Discussions: <https://leetcode.com/problems/number-of-ways-of-cutting-a-pizza/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given a rectangular pizza represented as a `rows x cols` matrix containing the following characters: `'A'` (an apple) and `'.'` (empty cell) and given the integer `k`. You have to cut the pizza into `k` pieces using `k-1` cuts.
//!
//! For each cut you choose the direction: vertical or horizontal, then you choose a cut position at the cell boundary and cut the pizza into two pieces. If you cut the pizza vertically, give the left part of the pizza to a person. If you cut the pizza horizontally, give the upper part of the pizza to a person. Give the last piece of pizza to the last person.
//!
//! *Return the number of ways of cutting the pizza such that each piece contains **at least** one apple.* Since the answer can be a huge number, return this modulo 10^9 + 7.
//!
//! **Example 1:**
//!
//! **![](https://assets.leetcode.com/uploads/2020/04/23/ways_to_cut_apple_1.png)**
//!
//! ```
//! Input: pizza = ["A..","AAA","..."], k = 3
//! Output: 3
//! Explanation: The figure above shows the three ways to cut the pizza. Note that pieces must contain at least one apple.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: pizza = ["A..","AA.","..."], k = 3
//! Output: 1
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: pizza = ["A..","A..","..."], k = 1
//! Output: 1
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= rows, cols <= 50`
//! * `rows == pizza.length`
//! * `cols == pizza[i].length`
//! * `1 <= k <= 10`
//! * `pizza` consists of characters `'A'` and `'.'` only.
//!

#[solution]
impl Solution {
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        let pizza_bytes = pizza.iter().map(|v| v.as_bytes()).collect::<Vec<&[u8]>>();
        let mut dp = vec![vec![vec![0; 2]; pizza[0].len() + 1]; pizza.len() + 1];

        let total_row = pizza.len() - 1;
        let total_col = pizza.len() - 1;

        for row in 0..total_row {
            for col in 0..total_col {
                // Cut left
                if col > 0 {
                    dp[row][col][0] = dp[row][col - 1][0];
                }
                if pizza_bytes[row][col] == 'A' as u8 {
                    dp[row][col][0] += 1;
                }

                // Cut top
                if row > 0 {
                    dp[row][col][1] = dp[row - 1][col][1];
                }
                if pizza_bytes[row][col] == 'A' as u8 {
                    dp[row][col][1] += 1;
                }
            }
        }

        dp[total_row - 1][total_col - 1][0] + dp[total_row - 1][total_col - 1][1]
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
    fn test_1444() {
        assert_eq!(3, Solution::ways(lc_strvec!["A..", "AAA", "..."], 3));
        assert_eq!(1, Solution::ways(lc_strvec!["A..", "AA.", "..."], 3));
    }
}
