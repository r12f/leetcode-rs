//!
//! Problem #221: Maximal Square
//!
//! - Link: <https://leetcode.com/problems/maximal-square/>
//! - Discussions: <https://leetcode.com/problems/maximal-square/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given an `m x n` binary `matrix` filled with `0`'s and `1`'s, *find the largest square containing only* `1`'s *and return its area*.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2020/11/26/max1grid.jpg)
//!
//! ```
//! Input: matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
//! Output: 4
//!
//! ```
//!
//! **Example 2:**
//!
//! ![](https://assets.leetcode.com/uploads/2020/11/26/max2grid.jpg)
//!
//! ```
//! Input: matrix = [["0","1"],["1","0"]]
//! Output: 1
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: matrix = [["0"]]
//! Output: 0
//!
//! ```
//!
//! **Constraints:**
//!
//! * `m == matrix.length`
//! * `n == matrix[i].length`
//! * `1 <= m, n <= 300`
//! * `matrix[i][j]` is `'0'` or `'1'`.
//!

#[solution]
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let mut result = 0;

        let mut dp = vec![vec![0; matrix[0].len()]; matrix.len()];
        for i in 0..matrix[0].len() {
            dp[0][i] = if matrix[0][i] == '1' { 1 } else { 0 };
            if dp[0][i] == 1 {
                result = 1;
            }
        }

        for i in 0..matrix.len() {
            dp[i][0] = if matrix[i][0] == '1' { 1 } else { 0 };
            if dp[i][0] == 1 {
                result = 1;
            }
        }

        for row in 1..matrix.len() {
            for col in 1..matrix[row].len() {
                if matrix[row][col] == '1' {
                    dp[row][col] = dp[row - 1][col]
                        .min(dp[row][col - 1])
                        .min(dp[row - 1][col - 1])
                        + 1;

                    if dp[row][col] > result {
                        result = dp[row][col];
                    }
                }
            }
        }

        result * result
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
    fn test_221() {
        assert_eq!(1, Solution::maximal_square(vec![vec!['0'], vec!['1'],]),);

        assert_eq!(
            4,
            Solution::maximal_square(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
        );
    }
}
