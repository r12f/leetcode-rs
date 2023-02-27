//!
//! Problem #542: 01 Matrix
//!
//! - Link: <https://leetcode.com/problems/01-matrix/>
//! - Discussions: <https://leetcode.com/problems/01-matrix/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given an `m x n` binary matrix `mat`, return *the distance of the nearest* `0` *for each cell*.
//!
//! The distance between two adjacent cells is `1`.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/04/24/01-1-grid.jpg)
//!
//! ```
//! Input: mat = [[0,0,0],[0,1,0],[0,0,0]]
//! Output: [[0,0,0],[0,1,0],[0,0,0]]
//!
//! ```
//!
//! **Example 2:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/04/24/01-2-grid.jpg)
//!
//! ```
//! Input: mat = [[0,0,0],[0,1,0],[1,1,1]]
//! Output: [[0,0,0],[0,1,0],[1,2,1]]
//!
//! ```
//!
//! **Constraints:**
//!
//! * `m == mat.length`
//! * `n == mat[i].length`
//! * `1 <= m, n <= 10<sup>4</sup>`
//! * `1 <= m * n <= 10<sup>4</sup>`
//! * `mat[i][j]` is either `0` or `1`.
//! * There is at least one `0` in `mat`.
//!

#[solution]
impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![i32::MAX; mat[0].len()]; mat.len()];

        // DP: DP[i][j] = min(DP[i][j - 1], DP[i - 1][j])
        for row in 0..mat.len() {
            for col in 0..mat[row].len() {
                if mat[row][col] == 0 {
                    result[row][col] = 0;
                } else {
                    if row > 0 && result[row - 1][col] != i32::MAX {
                        result[row][col] = result[row][col].min(result[row - 1][col] + 1);
                    }
                    if col > 0 && result[row][col - 1] != i32::MAX {
                        result[row][col] = result[row][col].min(result[row][col - 1] + 1);
                    }
                };
            }
        }

        // DP: DP[i][j] = min(DP[i][j - 1], DP[i - 1][j])
        for row in (0..mat.len()).rev() {
            for col in (0..mat[row].len()).rev() {
                if mat[row][col] == 0 {
                    result[row][col] = 0;
                } else {
                    if row < mat.len() - 1 && result[row + 1][col] != i32::MAX {
                        result[row][col] = result[row][col].min(result[row + 1][col] + 1);
                    }
                    if col < mat[row].len() - 1 && result[row][col + 1] != i32::MAX {
                        result[row][col] = result[row][col].min(result[row][col + 1] + 1);
                    }
                };
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
    fn test_542() {
        assert_eq!(
            lc_vecvec![[0, 0, 0], [0, 1, 0], [0, 0, 0]],
            Solution::update_matrix(lc_vecvec![[0, 0, 0], [0, 1, 0], [0, 0, 0]])
        );
    }
}
