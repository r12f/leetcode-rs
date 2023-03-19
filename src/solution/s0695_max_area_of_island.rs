//!
//! Problem #695: Max Area of Island
//!
//! - Link: <https://leetcode.com/problems/max-area-of-island/>
//! - Discussions: <https://leetcode.com/problems/max-area-of-island/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! You are given an `m x n` binary matrix `grid`. An island is a group of `1`'s (representing land) connected **4-directionally** (horizontal or vertical.) You may assume all four edges of the grid are surrounded by water.
//!
//! The **area** of an island is the number of cells with a value `1` in the island.
//!
//! Return *the maximum **area** of an island in* `grid`. If there is no island, return `0`.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/05/01/maxarea1-grid.jpg)
//!
//! ```
//! Input: grid = [[0,0,1,0,0,0,0,1,0,0,0,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,1,1,0,1,0,0,0,0,0,0,0,0],[0,1,0,0,1,1,0,0,1,0,1,0,0],[0,1,0,0,1,1,0,0,1,1,1,0,0],[0,0,0,0,0,0,0,0,0,0,1,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,0,0,0,0,0,0,1,1,0,0,0,0]]
//! Output: 6
//! Explanation: The answer is not 11, because the island must be connected 4-directionally.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: grid = [[0,0,0,0,0,0,0,0]]
//! Output: 0
//!
//! ```
//!
//! **Constraints:**
//!
//! * `m == grid.length`
//! * `n == grid[i].length`
//! * `1 <= m, n <= 50`
//! * `grid[i][j]` is either `0` or `1`.
//!

#[solution]
impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut max_area = 0;

        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                let area = Self::dfs(&grid, row, col, &mut visited);
                max_area = max_area.max(area);
            }
        }

        max_area
    }

    fn dfs(grid: &Vec<Vec<i32>>, row: usize, col: usize, visited: &mut Vec<Vec<bool>>) -> i32 {
        if visited[row][col] {
            return 0;
        }
        visited[row][col] = true;

        if grid[row][col] == 0 {
            return 0;
        }

        let mut area = 1;

        let dx = vec![0, 1, 0, -1];
        let dy = vec![1, 0, -1, 0];
        for i in 0..dx.len() {
            let new_row = row as i32 + dx[i];
            let new_col = col as i32 + dy[i];

            if new_row < 0
                || new_row >= grid.len() as i32
                || new_col < 0
                || new_col >= grid[0].len() as i32
            {
                continue;
            }

            let new_row = new_row as usize;
            let new_col = new_col as usize;

            area += Self::dfs(grid, new_row, new_col, visited);
        }

        area
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
    fn test_695() {
        assert_eq!(
            6,
            Solution::max_area_of_island(lc_vecvec![
                [0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                [0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
                [0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
            ])
        );
    }
}
