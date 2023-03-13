//!
//! Problem #1091: Shortest Path in Binary Matrix
//!
//! - Link: <https://leetcode.com/problems/shortest-path-in-binary-matrix/>
//! - Discussions: <https://leetcode.com/problems/shortest-path-in-binary-matrix/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given an `n x n` binary matrix `grid`, return *the length of the shortest **clear path** in the matrix*. If there is no clear path, return `-1`.
//!
//! A **clear path** in a binary matrix is a path from the **top-left** cell (i.e., `(0, 0)`) to the **bottom-right** cell (i.e., `(n - 1, n - 1)`) such that:
//!
//! * All the visited cells of the path are `0`.
//! * All the adjacent cells of the path are **8-directionally** connected (i.e., they are different and they share an edge or a corner).
//!
//! The **length of a clear path** is the number of visited cells of this path.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/02/18/example1_1.png)
//!
//! ```
//! Input: grid = [[0,1],[1,0]]
//! Output: 2
//!
//! ```
//!
//! **Example 2:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/02/18/example2_1.png)
//!
//! ```
//! Input: grid = [[0,0,0],[1,1,0],[1,1,0]]
//! Output: 4
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: grid = [[1,0,0],[1,1,0],[1,1,0]]
//! Output: -1
//!
//! ```
//!
//! **Constraints:**
//!
//! * `n == grid.length`
//! * `n == grid[i].length`
//! * `1 <= n <= 100`
//! * `grid[i][j] is 0 or 1`
//!

use std::collections::VecDeque;

#[solution]
impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][0] != 0 {
            return -1;
        }

        let dx = vec![0, 1, 1, 1, 0, -1, -1, -1];
        let dy = vec![1, 1, 0, -1, -1, -1, 0, 1];

        let mut queue = VecDeque::<(usize, usize, i32)>::new();
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

        queue.push_back((0, 0, 1));
        visited[0][0] = true;

        while let Some((row, col, step)) = queue.pop_front() {
            if row == grid.len() - 1 && col == grid[row].len() - 1 {
                return step;
            }

            for i in 0..dx.len() {
                let new_row = row as i32 + dx[i];
                let new_col = col as i32 + dy[i];
                if new_row < 0
                    || new_row >= grid.len() as i32
                    || new_col < 0
                    || new_col >= grid[row].len() as i32
                {
                    continue;
                }

                let new_row = new_row as usize;
                let new_col = new_col as usize;

                if grid[new_row][new_col] != 0 {
                    continue;
                }

                if visited[new_row][new_col] {
                    continue;
                }
                visited[new_row][new_col] = true;

                queue.push_back((new_row, new_col, step + 1));
            }
        }

        return -1;
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
    fn test_1091() {
        assert_eq!(
            4,
            Solution::shortest_path_binary_matrix(lc_vecvec![[0, 0, 0], [1, 1, 0], [1, 1, 0]])
        );
    }
}
